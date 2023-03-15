// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.13;

import "./Objective.sol";

/**
 * @title   Portfolio
 * @author  Primitive™
 * @custom:contributor TomAFrench
 */
abstract contract PortfolioVirtual is Objective {
    using SafeCastLib for uint256;
    using FixedPointMathLib for int256;
    using FixedPointMathLib for uint256;
    using AssemblyLib for uint8;
    using AssemblyLib for int256;
    using AssemblyLib for uint256;

    function VERSION() public pure returns (string memory) {
        assembly ("memory-safe") {
            // Load 0x20 (32) in memory at slot 0x00, this corresponds to the
            // offset location of the next data.
            mstore(0x00, 0x20)

            // Then we load both the length of our string (11 bytes, 0x0b in hex) and its
            // actual hex value (0x626574612d76302e312e30) using the offset 0x2b. Using this
            // particular offset value will right pad the length at the end of the slot
            // and left pad the string at the beginning of the next slot, assuring the
            // right ABI format to return a string.
            mstore(0x2b, 0x0b76312e302e302d62657461) // "v1.0.0-beta"

            // Return all the 96 bytes (0x60) of data that was loaded into the memory.
            return(0x00, 0x60)
        }
    }

    Account.AccountSystem public __account__;

    /// @inheritdoc IPortfolioGetters
    address public immutable WETH;
    /// @inheritdoc IPortfolioGetters
    address public immutable REGISTRY;
    /// @inheritdoc IPortfolioGetters
    uint24 public getPairNonce;

    mapping(uint24 => uint32) public getPoolNonce;
    mapping(uint24 => PortfolioPair) public pairs;
    mapping(uint64 => PortfolioPool) public pools;
    mapping(address => mapping(address => uint24)) public getPairId;
    mapping(address => mapping(uint64 => PortfolioPosition)) public positions;

    uint256 internal _locked = 1;
    uint256 internal _liquidityPolicy = JUST_IN_TIME_LIQUIDITY_POLICY;
    uint256 private _protocolFee;

    /**
     * @dev Manipulated in `_settlement` only.
     * @custom:invariant MUST be deleted after every transaction that uses it.
     */
    Payment[] private _payments;

    /**
     * @dev
     * Manipulated in `_swap` to avoid stack too deep.
     * Utilized in virtual function implementations to handle fee growth, if any.
     * Implements internal functions to manipulate `feeGrowthGlobal` and `invariantGrowthGlobal`.
     *
     * @custom:invariant MUST be deleted after every transaction that uses it.
     */
    SwapState private _state;

    /**
     * @dev
     * Protects against re-entrancy and getting to invalid settlement states.
     * Used on all external non-view functions.
     *
     * @custom:guide
     * Step 1. Enter `_locked` re-entrancy guard.
     * Step 3. Wrap the entire ether balance of this contract and credit the wrapped ether to the msg.sender account.
     * Step 5. Execute the function logic.
     * Step 7. Enter the settlement function, requesting token payments or sending them out to msg.sender.
     * Step 8. Validate Portfolio's account system was settled.
     * Step 10. Exit `_locked` re-entrancy guard.
     */
    modifier lock() {
        if (_locked != 1) revert InvalidReentrancy();

        _locked = 2;
        _;
        _locked = 1;

        if (!__account__.settled) revert InvalidSettlement();
    }

    /**
     * @dev
     * Failing to pass a valid WETH contract that implements the `deposit()` function,
     * will cause all transactions with Portfolio to fail once address(this).balance > 0.
     *
     * @notice
     * Tokens sent to this contract are lost.
     */
    constructor(address weth, address registry) {
        WETH = weth;
        REGISTRY = registry;
        __account__.settled = true;
    }

    receive() external payable {
        if (msg.sender != WETH) revert();
    }

    // ===== Account Getters ===== //

    /// @inheritdoc IPortfolioGetters
    function getNetBalance(address token) public view returns (int256) {
        return __account__.getNetBalance(token, address(this));
    }

    /// @inheritdoc IPortfolioGetters
    function getReserve(address token) public view returns (uint256) {
        return __account__.reserves[token];
    }

    /// @inheritdoc IPortfolioGetters
    function getBalance(
        address owner,
        address token
    ) public view returns (uint256) {
        return __account__.balances[owner][token];
    }

    // ===== External Actions ===== //

    /// @inheritdoc IPortfolioActions
    function deposit() external payable override lock {
        // Checks
        if (msg.value == 0) revert ZeroValue();

        // Wraps msg.value.
        _deposit();

        // Interactions
        _settlement();
    }

    /// @inheritdoc IPortfolioActions
    function multiprocess(bytes calldata data) external payable lock {
        // Wraps msg.value.
        _deposit();

        // Effects
        if (data[0] != FVM.INSTRUCTION_JUMP) _process(data);
        else FVM._jumpProcess(data, _process);

        // Interactions
        _settlement();
    }

    /// @inheritdoc IPortfolioActions
    function draw(
        address token,
        uint256 amount,
        address to
    ) external override lock {
        // Checks
        if (to == address(this)) revert InvalidTransfer();

        uint256 balance = getBalance(msg.sender, token);
        if (amount == type(uint256).max) amount = balance;
        if (amount > balance) revert DrawBalance();

        // Effects
        _applyDebit(token, amount);
        _decreaseReserves(token, amount);

        if (token == WETH) {
            Account.__dangerousUnwrapEther__(WETH, to, amount);
        } else {
            Account.SafeTransferLib.safeTransfer(
                Account.ERC20(token), to, amount
            );
        }

        // Interactions
        _settlement();
    }

    /// @inheritdoc IPortfolioActions
    function fund(address token, uint256 amount) external override lock {
        // Checks
        if (amount == type(uint256).max) {
            amount = Account.__balanceOf__(token, msg.sender);
        }

        // Interactions
        __account__.dangerousFund(token, address(this), amount); // Warning: external call to msg.sender.
        _settlement();
    }

    /// @inheritdoc IPortfolioActions
    function changeParameters(
        uint64 poolId,
        uint16 priorityFee,
        uint16 fee,
        uint16 jit
    ) external lock {
        PortfolioPool storage pool = pools[poolId];
        if (pool.controller != msg.sender) revert NotController();

        PortfolioCurve memory modified;
        modified = pool.params;
        if (jit != 0) modified.jit = jit;
        if (fee != 0) modified.fee = fee;
        if (priorityFee != 0) modified.priorityFee = priorityFee;

        pool.changePoolParameters(modified);

        emit ChangeParameters(poolId, priorityFee, fee, jit);
    }

    // ===== Internal ===== //

    /**
     * @dev Re-assigns the tokens owed of a position to the `msg.sender`'s internal balance.
     * @param deltaAsset Quantity of asset tokens in native token decimals to re-assign.
     * @param deltaQuote Quantity of quote tokens in native token decimals to re-assign.
     */
    function _claim(
        uint64 poolId,
        uint128 deltaAsset,
        uint128 deltaQuote
    ) internal {
        PortfolioPosition storage pos = positions[msg.sender][poolId];
        if (pos.lastTimestamp == 0) {
            revert NonExistentPosition(msg.sender, poolId);
        }

        PortfolioPool storage pool = pools[poolId];
        (
            uint256 growthAsset,
            uint256 growthQuote,
            uint256 growthInvariant,
            address asset,
            address quote
        ) = (
            pool.feeGrowthGlobalAsset,
            pool.feeGrowthGlobalQuote,
            pool.invariantGrowthGlobal,
            pool.pair.tokenAsset,
            pool.pair.tokenQuote
        );

        pos.syncPositionFees(growthAsset, growthQuote, growthInvariant);

        // 2^128 is a magic variable to claim the maximum amount of owed tokens after it has been synced.
        uint256 claimedAssets =
            deltaAsset == type(uint128).max ? pos.tokensOwedAsset : deltaAsset;
        uint256 claimedQuotes =
            deltaQuote == type(uint128).max ? pos.tokensOwedQuote : deltaQuote;

        pos.tokensOwedAsset -= claimedAssets.safeCastTo128();
        pos.tokensOwedQuote -= claimedQuotes.safeCastTo128();

        if (claimedAssets > 0) _applyCredit(msg.sender, asset, claimedAssets);
        if (claimedQuotes > 0) _applyCredit(msg.sender, quote, claimedQuotes);

        emit Collect(
            poolId, msg.sender, claimedAssets, asset, claimedQuotes, quote
            );
    }

    /**
     * @dev Increases virtual reserves and liquidity. Debits `msg.sender`.
     */
    function _allocate(
        bool useMax,
        uint64 poolId,
        uint128 deltaLiquidity
    ) internal returns (uint256 deltaAsset, uint256 deltaQuote) {
        if (!checkPool(poolId)) revert NonExistentPool(poolId);

        (address asset, address quote) =
            (pools[poolId].pair.tokenAsset, pools[poolId].pair.tokenQuote);

        if (useMax) {
            deltaLiquidity = getMaxLiquidity({
                poolId: poolId,
                amount0: getBalance(msg.sender, asset),
                amount1: getBalance(msg.sender, quote)
            });
        }

        if (deltaLiquidity == 0) revert ZeroLiquidity();
        (deltaAsset, deltaQuote) =
            getLiquidityDeltas(poolId, AssemblyLib.toInt128(deltaLiquidity)); // note: Rounds up.
        if (deltaAsset == 0 || deltaQuote == 0) revert ZeroAmounts();

        ChangeLiquidityParams memory args = ChangeLiquidityParams({
            owner: msg.sender,
            poolId: poolId,
            timestamp: block.timestamp,
            deltaAsset: deltaAsset,
            deltaQuote: deltaQuote,
            tokenAsset: asset,
            tokenQuote: quote,
            deltaLiquidity: AssemblyLib.toInt128(deltaLiquidity)
        });

        _changeLiquidity(args);

        emit Allocate(
            poolId, asset, quote, deltaAsset, deltaQuote, deltaLiquidity
            );
    }

    /**
     * @dev Reduces virtual reserves and liquidity. Credits `msg.sender`.
     */
    function _deallocate(
        bool useMax,
        uint64 poolId,
        uint128 deltaLiquidity
    ) internal returns (uint256 deltaAsset, uint256 deltaQuote) {
        if (!checkPool(poolId)) revert NonExistentPool(poolId);
        (address asset, address quote) =
            (pools[poolId].pair.tokenAsset, pools[poolId].pair.tokenQuote);

        if (useMax) {
            deltaLiquidity = positions[msg.sender][poolId].freeLiquidity;
        }

        if (deltaLiquidity == 0) revert ZeroLiquidity();
        (deltaAsset, deltaQuote) =
            getLiquidityDeltas(poolId, -AssemblyLib.toInt128(deltaLiquidity)); // note: Rounds down.

        ChangeLiquidityParams memory args = ChangeLiquidityParams({
            owner: msg.sender,
            poolId: poolId,
            timestamp: block.timestamp,
            deltaAsset: deltaAsset,
            deltaQuote: deltaQuote,
            tokenAsset: asset,
            tokenQuote: quote,
            deltaLiquidity: -AssemblyLib.toInt128(deltaLiquidity)
        });

        _changeLiquidity(args);

        emit Deallocate(
            poolId, asset, quote, deltaAsset, deltaQuote, deltaLiquidity
            );
    }

    /**
     * @dev Manipulates reserves depending on if liquidity is being allocated or deallocated.
     */
    function _changeLiquidity(ChangeLiquidityParams memory args)
        internal
        returns (uint256 feeAsset, uint256 feeQuote, uint256 invariantGrowth)
    {
        (PortfolioPool storage pool, PortfolioPosition storage position) =
            (pools[args.poolId], positions[args.owner][args.poolId]);

        (feeAsset, feeQuote, invariantGrowth) = position.syncPositionFees(
            pool.feeGrowthGlobalAsset,
            pool.feeGrowthGlobalQuote,
            pool.invariantGrowthGlobal
        );

        bool canUpdate =
            checkPosition(args.poolId, args.owner, args.deltaLiquidity);
        if (!canUpdate) revert JitLiquidity(pool.params.jit);

        position.changePositionLiquidity(args.timestamp, args.deltaLiquidity);
        pools[args.poolId].changePoolLiquidity(args.deltaLiquidity);

        (address asset, address quote) = (args.tokenAsset, args.tokenQuote);
        if (args.deltaLiquidity < 0) {
            _decreaseReserves(asset, args.deltaAsset);
            _decreaseReserves(quote, args.deltaQuote);
        } else {
            _increaseReserves(asset, args.deltaAsset);
            _increaseReserves(quote, args.deltaQuote);
        }
    }

    /**
     * @dev
     * Swaps in input of tokens (sellAsset == 1 = asset, sellAsset == 0 = quote)
     * for output of tokens (sellAsset == 1 = quote, sellAsset == 0 = asset).
     *
     * Fees can be saved into two buckets:
     * 1. Re-invested into the pool, increasing the value of liquidity.
     * 2. Pro-rata distribution to all liquidity position's `owed` tokens, via fee growth accumulator.
     *
     * These different fee buckets are applied using this logic:
     * 1. Add the input swap amount, fee included, to the per liquidity reserves in `syncPool`.
     * 2. Add the input swap amount less the fee, to the per liquidity reserves in `syncPool`
     *    and increase the `feeGrowthGlobal` value by the `feeAmount` divided by `pool.liquidity`.
     *
     * @custom:invariant MUST not change liquidity of a pool.
     */
    function _swap(Order memory args)
        internal
        returns (uint64 poolId, uint256 input, uint256 output)
    {
        // =---= Checks =---= //
        if (args.input == 0) revert ZeroInput();

        PortfolioPool storage pool = pools[args.poolId];
        if (!checkPool(args.poolId)) revert NonExistentPool(args.poolId);

        // -=- Load Fee & Token Info -=- //
        _state.sell = args.sellAsset == 1;
        _state.fee = msg.sender == pool.controller
            ? pool.params.priorityFee
            : pool.params.fee;

        if (_state.sell) {
            _state.feeGrowthGlobal = pool.feeGrowthGlobalAsset;
            _state.tokenInput = pool.pair.tokenAsset;
            _state.tokenOutput = pool.pair.tokenQuote;
        } else {
            _state.feeGrowthGlobal = pool.feeGrowthGlobalQuote;
            _state.tokenInput = pool.pair.tokenQuote;
            _state.tokenOutput = pool.pair.tokenAsset;
        }

        // -=- Load Swap Info -=- //
        Iteration memory iteration;
        {
            (bool success, int256 invariant) = _beforeSwapEffects(args.poolId);
            if (!success) revert PoolExpired();

            uint256 internalBalance = getBalance(
                msg.sender,
                _state.sell ? pool.pair.tokenAsset : pool.pair.tokenQuote
            );
            input = args.useMax == 1 ? internalBalance : args.input;
            input = input.scaleToWad(
                _state.sell ? pool.pair.decimalsAsset : pool.pair.decimalsQuote
            );
            output = args.output;
            output = output.scaleToWad(
                _state.sell ? pool.pair.decimalsQuote : pool.pair.decimalsAsset
            );

            iteration.prevInvariant = invariant;
            iteration.input = input;
            iteration.liquidity = pool.liquidity;
            iteration.output = output;
            (iteration.virtualX, iteration.virtualY) =
                (pool.virtualX, pool.virtualY);
        }

        if (iteration.output == 0) revert ZeroOutput();
        if (iteration.input == 0) revert ZeroInput();
        if (iteration.liquidity == 0) revert ZeroLiquidity();

        // These are WAD values per WAD of liquidity.
        uint256 liveIndependent;
        uint256 nextIndependent;
        uint256 nextIndependentLessFee;
        uint256 liveDependent;
        uint256 nextDependent;

        //  -=- Compute New Reserves -=- //
        {
            uint256 deltaInput;
            uint256 deltaInputLessFee;
            uint256 deltaOutput = iteration.output;

            // Virtual reserves
            if (_state.sell) {
                (liveIndependent, liveDependent) =
                    (iteration.virtualX, iteration.virtualY);
            } else {
                (liveDependent, liveIndependent) =
                    (iteration.virtualX, iteration.virtualY);
            }

            deltaInput = iteration.input;

            iteration.feeAmount = (deltaInput * _state.fee) / PERCENTAGE;
            if (_protocolFee != 0) {
                uint256 protocolFeeAmount = iteration.feeAmount / _protocolFee;
                iteration.feeAmount -= protocolFeeAmount;
                _applyCredit(REGISTRY, _state.tokenInput, protocolFeeAmount);
            }

            deltaInputLessFee = deltaInput - iteration.feeAmount;

            // This value should be used in `syncPool` if fees are re-invested into the pool.
            nextIndependent =
                liveIndependent + deltaInput.divWadDown(iteration.liquidity);

            // This is a very critical piece of code!
            // This value should be used in `syncPool` if fees are not re-invested.
            // The next independent amount is computed with the fee amount applied.
            // This means the lesser next independent reserve and dependent reserve
            // will pass the invariant.
            // The fee amount has to be added to the reserve to re-invest it in the pool.
            // This will mean the independent reserve has more tokens than expected,
            // leading to a larger invariant.
            nextIndependentLessFee = liveIndependent
                + deltaInputLessFee.divWadDown(iteration.liquidity);
            nextDependent =
                liveDependent - deltaOutput.divWadDown(iteration.liquidity);
        }

        // -=- Assert Invariant Passes -=- //
        {
            bool validInvariant;
            int256 nextInvariantWad;

            // This is revisited depending on if fees are saved in claimable balances.
            if (_state.sell) {
                (iteration.virtualX, iteration.virtualY) =
                    (nextIndependentLessFee, nextDependent);
            } else {
                (iteration.virtualX, iteration.virtualY) =
                    (nextDependent, nextIndependentLessFee);
            }

            (validInvariant, nextInvariantWad) = checkInvariant(
                args.poolId,
                iteration.prevInvariant,
                iteration.virtualX,
                iteration.virtualY,
                block.timestamp
            );

            if (!validInvariant) {
                revert InvalidInvariant(
                    iteration.prevInvariant, nextInvariantWad
                );
            }
            iteration.nextInvariant = int128(nextInvariantWad);
        }

        // -=- Apply Fee Saving Method -=- //
        {
            // Fees are saved by incrementing the fee growth accumulator.
            bool saved = _feeSavingEffects(args.poolId, iteration);

            // If the fees are not saved,
            // apply the full next independent amount with fee amount included.
            // Fees were not saved in the claimable balances,
            // so this will re-invest the fees into the pool.
            if (!saved) {
                if (_state.sell) {
                    iteration.virtualX = nextIndependent;
                } else {
                    iteration.virtualY = nextIndependent;
                }
            }
        }

        // =---= Effects =---= //

        _syncPool(
            args.poolId,
            iteration.virtualX,
            iteration.virtualY,
            iteration.liquidity,
            _state.sell ? _state.feeGrowthGlobal : 0,
            _state.sell ? 0 : _state.feeGrowthGlobal,
            _state.invariantGrowthGlobal
        );

        // -=- Scale Amounts to Native Token Decimals -=- //
        {
            uint256 inputDec;
            uint256 outputDec;
            if (_state.sell) {
                inputDec = pool.pair.decimalsAsset;
                outputDec = pool.pair.decimalsQuote;
            } else {
                inputDec = pool.pair.decimalsQuote;
                outputDec = pool.pair.decimalsAsset;
            }

            // Scaling the input here is important. ALl the math was done using WAD units.
            // But all the token related amounts must be in their native token decimals.
            iteration.input = iteration.input.scaleFromWadDown(inputDec);
            iteration.output = iteration.output.scaleFromWadDown(outputDec);
        }

        // Increasing reserves expects a debit from `msg.sender`,
        // a gifted surplus of tokens that is not synced (e.g. tokens transferred to Portfolio),
        // or tokens sent into Portfolio via `transferFrom` in the `_settlement` function.
        // Decreasing reserves credits the `msg.sender`'s account.
        _increaseReserves(_state.tokenInput, iteration.input);
        _decreaseReserves(_state.tokenOutput, iteration.output);

        emit Swap(
            args.poolId,
            getVirtualPrice(args.poolId),
            _state.tokenInput,
            iteration.input,
            _state.tokenOutput,
            iteration.output,
            iteration.feeAmount,
            iteration.nextInvariant
            );

        delete _state;
        return (args.poolId, iteration.input, iteration.output);
    }

    function _syncFeeGrowthAccumulator(uint256 feeGrowthGlobal) internal {
        _state.feeGrowthGlobal = feeGrowthGlobal;
    }

    function _syncInvariantGrowthAccumulator(uint256 invariantGrowthGlobal)
        internal
    {
        _state.invariantGrowthGlobal = invariantGrowthGlobal;
    }

    /**
     * @dev Effects on a `pool` after a successful swap.
     */
    function _syncPool(
        uint64 poolId,
        uint256 nextVirtualX,
        uint256 nextVirtualY,
        uint256 liquidity,
        uint256 feeGrowthGlobalAsset,
        uint256 feeGrowthGlobalQuote,
        uint256 invariantGrowthGlobal
    ) internal {
        PortfolioPool storage pool = pools[poolId];

        pool.virtualX = nextVirtualX.safeCastTo128();
        pool.virtualY = nextVirtualY.safeCastTo128();
        pool.liquidity = liquidity.safeCastTo128();
        pool.syncPoolTimestamp(block.timestamp);
        pool.feeGrowthGlobalAsset = AssemblyLib.computeCheckpoint(
            pool.feeGrowthGlobalAsset, feeGrowthGlobalAsset
        );
        pool.feeGrowthGlobalQuote = AssemblyLib.computeCheckpoint(
            pool.feeGrowthGlobalQuote, feeGrowthGlobalQuote
        );
        pool.invariantGrowthGlobal = AssemblyLib.computeCheckpoint(
            pool.invariantGrowthGlobal, invariantGrowthGlobal
        );
    }

    function _createPair(
        address asset,
        address quote
    ) internal returns (uint24 pairId) {
        if (asset == quote) revert SameTokenError();

        pairId = getPairId[asset][quote];
        if (pairId != 0) revert PairExists(pairId);

        (uint8 decimalsAsset, uint8 decimalsQuote) =
            (IERC20(asset).decimals(), IERC20(quote).decimals());
        if (!decimalsAsset.isBetween(MIN_DECIMALS, MAX_DECIMALS)) {
            revert InvalidDecimals(decimalsAsset);
        }
        if (!decimalsQuote.isBetween(MIN_DECIMALS, MAX_DECIMALS)) {
            revert InvalidDecimals(decimalsQuote);
        }

        pairId = ++getPairNonce;

        getPairId[asset][quote] = pairId; // note: Order of tokens matters!
        pairs[pairId] = PortfolioPair({
            tokenAsset: asset,
            decimalsAsset: decimalsAsset,
            tokenQuote: quote,
            decimalsQuote: decimalsQuote
        });

        emit CreatePair(pairId, asset, quote, decimalsAsset, decimalsQuote);
    }

    /**
     * @param pairId Nonce of the target pair. A `0` is a magic variable to use the state variable `getPairNonce` instead.
     * @param controller An address that can change the `fee`, `priorityFee`, and `jit` parameters of the created pool.
     * @param duration Sets the quantity of days (in units of days) until the pool "expires". Uses `type(uint16).max` as a magic variable to set `perpetual = true`.
     */
    function _createPool(
        uint24 pairId,
        address controller,
        uint16 priorityFee,
        uint16 fee,
        uint16 volatility,
        uint16 duration,
        uint16 jit,
        uint128 maxPrice,
        uint128 price
    ) internal returns (uint64 poolId) {
        if (price == 0) revert ZeroPrice();
        uint24 pairNonce = pairId == 0 ? getPairNonce : pairId; // magic variable
        if (pairNonce == 0) revert InvalidPair();

        bool hasController = controller != address(0);
        {
            uint32 poolNonce = ++getPoolNonce[pairNonce];
            poolId = FVM.encodePoolId(pairNonce, hasController, poolNonce);
        }

        PortfolioPool storage pool = pools[poolId];
        pool.controller = controller;
        if (hasController && priorityFee == 0) revert InvalidFee(priorityFee); // Cannot set priority to 0.

        uint32 timestamp = block.timestamp.safeCastTo32();
        pool.lastTimestamp = timestamp;
        pool.pair = pairs[pairNonce];

        bool isPerpetual = duration == type(uint16).max ? true : false; // type(uint16).max is a magic variable
        PortfolioCurve memory params = PortfolioCurve({
            maxPrice: maxPrice,
            jit: hasController ? jit : uint8(_liquidityPolicy),
            fee: fee,
            duration: isPerpetual ? uint16(MAX_DURATION) : duration, // Set duration to the max if perpetual.
            volatility: volatility,
            priorityFee: hasController ? priorityFee : 0,
            createdAt: timestamp,
            perpetual: isPerpetual
        });
        pool.changePoolParameters(params);

        (uint256 x, uint256 y) = computeReservesFromPrice(poolId, price);
        (pool.virtualY, pool.virtualX) = (y.safeCastTo128(), x.safeCastTo128());

        emit CreatePool(
            poolId,
            hasController,
            pool.pair.tokenAsset,
            pool.pair.tokenQuote,
            price
            );
    }

    // ===== Accounting System ===== //

    /**
     * @dev Wraps address(this).balance of ether but does not credit to `msg.sender`.
     * Received WETH will remain in the contract as a surplus, i.e. `getNetBalance(WETH)` will be positive.
     * The `settlement` function handles how to apply the surplus,
     * by either using it to pay a debit or by gifting the `msg.sender`.
     */
    function _deposit() internal {
        if (msg.value > 0) {
            __account__.__wrapEther__(WETH);
            emit Deposit(msg.sender, msg.value);
        }
    }

    /**
     * @dev Reserves are an internally tracked amount of tokens that should match the return value of `balanceOf`.
     *
     * @custom:security Directly manipulates reserves.
     */
    function _increaseReserves(address token, uint256 amount) internal {
        __account__.increase(token, amount);
        emit IncreaseReserveBalance(token, amount);
    }

    /**
     * @dev Reserves are an internally tracked amount of tokens that should match the return value of `balanceOf`.
     *
     * @custom:security Directly manipulates reserves.
     * @custom:reverts With `InsufficientReserve` if current reserve balance for `token` iss less than `amount`.
     */
    function _decreaseReserves(address token, uint256 amount) internal {
        __account__.decrease(token, amount);
        emit DecreaseReserveBalance(token, amount);
    }

    /**
     * @dev A positive credit is a receivable paid to the `msg.sender` internal balance.
     *      Positive credits are only applied to the internal balance of the account.
     *      Therefore, it does not require a state change for the global reserves.
     *
     * @custom:security Directly manipulates internal balances.
     */
    function _applyCredit(address to, address token, uint256 amount) internal {
        __account__.credit(to, token, amount);
        emit IncreaseUserBalance(to, token, amount);
    }

    /**
     * @dev A positive debit is a cost that must be paid for a transaction to be processed.
     *      If a balance exists for the token for the internal balance of `msg.sender`,
     *      it will be used to pay the debit. Else, the contract expects tokens to be transferred in.
     *
     * @custom:security Directly manipulates internal balances.
     */
    function _applyDebit(address token, uint256 amount) internal {
        __account__.debit(msg.sender, token, amount);
        emit DecreaseUserBalance(msg.sender, token, amount);
    }

    /**
     * @dev Use `multiprocess` to enter this function to process instructions.
     * @param data Custom encoded FVM data. First byte must be an FVM instruction.
     */
    function _process(bytes calldata data) internal {
        (, bytes1 instruction) = AssemblyLib.separate(data[0]); // Upper byte is useMax, lower byte is instruction.

        if (instruction == FVM.ALLOCATE) {
            (uint8 useMax, uint64 poolId, uint128 deltaLiquidity) =
                FVM.decodeAllocateOrDeallocate(data);
            _allocate(useMax == 1, poolId, deltaLiquidity);
        } else if (instruction == FVM.DEALLOCATE) {
            (uint8 useMax, uint64 poolId, uint128 deltaLiquidity) =
                FVM.decodeAllocateOrDeallocate(data);
            _deallocate(useMax == 1, poolId, deltaLiquidity);
        } else if (
            instruction == FVM.SWAP_ASSET || instruction == FVM.SWAP_QUOTE
        ) {
            Order memory args;
            (args.useMax, args.poolId, args.input, args.output, args.sellAsset)
            = FVM.decodeSwap(data);
            _swap(args);
        } else if (instruction == FVM.CREATE_POOL) {
            (
                uint24 pairId,
                address controller,
                uint16 priorityFee,
                uint16 fee,
                uint16 vol,
                uint16 dur,
                uint16 jit,
                uint128 maxPrice,
                uint128 price
            ) = FVM.decodeCreatePool(data);
            _createPool(
                pairId,
                controller,
                priorityFee,
                fee,
                vol,
                dur,
                jit,
                maxPrice,
                price
            );
        } else if (instruction == FVM.CREATE_PAIR) {
            (address asset, address quote) = FVM.decodeCreatePair(data);
            _createPair(asset, quote);
        } else if (instruction == FVM.CLAIM) {
            (uint64 poolId, uint128 deltaAsset, uint128 deltaQuote) =
                FVM.decodeClaim(data);
            _claim(poolId, deltaAsset, deltaQuote);
        } else {
            revert InvalidInstruction();
        }
    }

    /**
     * Be aware of these settlement invariants:
     * =
     *     Invariant 1. Every token that is interacted with is cached and exists.
     *     Invariant 2. Tokens are removed from cache and cache is empty by end of settlement.
     *     Invariant 3. Cached tokens cannot be carried over from previous transactions.
     *     Invariant 4. Execution does not exit during the loops prematurely.
     *     Invariant 5. Account `settled` bool is set to true at end of `settlement`.
     *     Invariant 6. Debits reduce `reserves` of `token`.
     */
    function _settlement() internal {
        address[] memory tokens = __account__.warm;
        uint256 loops = tokens.length;

        if (loops == 0) return __account__.reset(); // Exit early.

        // Compute all the payments that must be paid to this contract.
        uint256 i = loops;
        do {
            // Loop backwards to pop tokens off.
            address token = tokens[i - 1];
            // Apply credits or debits to net balance.
            (uint256 credited, uint256 debited, uint256 remainder) =
                __account__.settle(token, address(this));

            // Reserves were not tracking some tokens, increase the reserves to account for them.
            if (credited > 0) {
                emit IncreaseUserBalance(msg.sender, token, credited);
                emit IncreaseReserveBalance(token, credited);
            } else {
                // Users are never simultaneously credited and debited so we must only check this when not credited.

                // Reserves were increased, we paid a debit, therefore need to decrease reserves by `debited` amount.
                if (debited > 0) {
                    emit DecreaseUserBalance(msg.sender, token, debited);
                    emit DecreaseReserveBalance(token, debited);
                }

                // Outstanding amount must be transferred in.
                if (remainder > 0) {
                    _payments.push(
                        Payment({
                            token: token,
                            amount: remainder,
                            balance: Account.__balanceOf__(token, address(this))
                        })
                    );
                }
            }

            // Token considered fully accounted for.
            __account__.warm.pop();
            unchecked {
                --i; // Cannot underflow because loop exits at 0!
            }
        } while (i != 0);

        // Uses `token.transferFrom(msg.sender, amount)` to pay for the outstanding debits.
        Payment[] memory payments = _payments;
        uint256 px = payments.length;
        while (px != 0) {
            uint256 index = px - 1;
            Account.__dangerousTransferFrom__(
                payments[index].token, address(this), payments[index].amount
            );
            unchecked {
                --px; // Cannot underflow because loop exits at 0!
            }
        }

        // Sanity check the payment amounts.
        px = payments.length;
        while (px != 0) {
            uint256 index = px - 1;
            address token = payments[index].token;
            uint256 prevBalance = payments[index].balance;
            uint256 nextBalance = Account.__balanceOf__(token, address(this));
            uint256 expectedBalance = payments[index].amount + prevBalance;
            if (nextBalance < expectedBalance) {
                revert NegativeBalance(
                    token, int256(nextBalance) - int256(expectedBalance)
                );
            }

            unchecked {
                --px; // Cannot underflow because loop exits at 0!
            }
        }

        __account__.reset(); // Clears token cache and sets `settled` to `true`.
        delete _payments;
    }

    function setProtocolFee(uint256 fee) external override lock {
        if (msg.sender != IPortfolioRegistry(REGISTRY).controller()) {
            revert NotController();
        }
        if (fee > 20 || fee < 4) revert InvalidFee(uint16(fee));

        uint256 prevFee = _protocolFee;
        _protocolFee = fee;

        emit UpdateProtocolFee(prevFee, fee);
    }

    // ===== Public View ===== //

    /// @inheritdoc IPortfolioGetters
    function getLiquidityDeltas(
        uint64 poolId,
        int128 deltaLiquidity
    ) public view returns (uint128 deltaAsset, uint128 deltaQuote) {
        return pools[poolId].getPoolLiquidityDeltas(deltaLiquidity);
    }

    /// @inheritdoc IPortfolioGetters
    function getMaxLiquidity(
        uint64 poolId,
        uint256 amount0,
        uint256 amount1
    ) public view returns (uint128 deltaLiquidity) {
        return pools[poolId].getPoolMaxLiquidity(amount0, amount1);
    }

    /// @inheritdoc IPortfolioGetters
    function getPoolReserves(uint64 poolId)
        public
        view
        override
        returns (uint256 deltaAsset, uint256 deltaQuote)
    {
        return pools[poolId].getPoolReserves();
    }

    /// @inheritdoc IPortfolioGetters
    function getVirtualReservesPerLiquidity(uint64 poolId)
        public
        view
        override
        returns (uint128 deltaAsset, uint128 deltaQuote)
    {
        return pools[poolId].getVirtualReservesPerLiquidity();
    }
}
