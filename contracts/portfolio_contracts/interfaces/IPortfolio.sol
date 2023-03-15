// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity >=0.8.0;

import { PortfolioCurve, PortfolioPair } from "../PortfolioLib.sol";

interface IPortfolioEvents {
    /**
     * @dev Ether transfers into Portfolio via payable `deposit` function.
     */
    event Deposit(address indexed account, uint256 amount);

    /**
     * @notice Assigns `amount` of `token` to `account`.
     * @dev Emitted on `deallocate`, `swap`, or `fund`.
     * @param amount Quantity of token in token's native decimal units.
     */
    event IncreaseUserBalance(
        address indexed account, address indexed token, uint256 amount
    );

    /**
     * @notice Unassigns `amount` of `token` from `account`.
     * @dev Emitted on `allocate`, `swap`, or `draw`.
     * @param amount Quantity of token in token's native decimal units.
     */
    event DecreaseUserBalance(
        address indexed account, address indexed token, uint256 amount
    );

    /**
     * @notice Assigns an additional `amount` of `token` to Portfolio's internally tracked balance.
     * @dev Emitted on `swap`, `allocate`, and when a user is gifted surplus tokens that were sent to the contract.
     * @param amount Quantity of token in token's native decimal units.
     */
    event IncreaseReserveBalance(address indexed token, uint256 amount);

    /**
     * @notice Unassigns `amount` of `token` from Portfolio's internally tracked balance.
     * @dev Emitted on `swap`, `deallocate`, and when paying with an internal balance.
     * @param amount Quantity of token in token's native decimal units.
     */
    event DecreaseReserveBalance(address indexed token, uint256 amount);

    /**
     * @dev Assigns `input` amount of `sellAsset` to Portfolio's reserves.
     * Unassigns `output` amount of `tokenOut` from Portfolio's reserves.
     * @param price Post-swap approximated marginal price in wad units.
     * @param feeAmountDec Amount of `sellAsset` tokens paid as a fee.
     * @param invariantWad Post-swap invariant in wad units.
     */
    event Swap(
        uint64 indexed poolId,
        uint256 price,
        address indexed sellAsset,
        uint256 input,
        address indexed tokenOut,
        uint256 output,
        uint256 feeAmountDec,
        int256 invariantWad
    );

    /**
     * @dev Assigns amount `deltaAsset` of `asset` and `deltaQuote` of `quote` tokens to `poolId.
     * Units are in the respective tokens' native decimals. Units for `deltaLiquidity` are WAD.
     */
    event Allocate(
        uint64 indexed poolId,
        address indexed asset,
        address indexed quote,
        uint256 deltaAsset,
        uint256 deltaQuote,
        uint256 deltaLiquidity
    );

    /**
     * @dev Unassigns amount `deltaAsset` of `asset` and `deltaQuote` of `quote` tokens to `poolId.
     * Units are in the respective tokens' native decimals. Units for `deltaLiquidity` are WAD.
     */
    event Deallocate(
        uint64 indexed poolId,
        address indexed asset,
        address indexed quote,
        uint256 deltaAsset,
        uint256 deltaQuote,
        uint256 deltaLiquidity
    );

    /**
     * @dev Emits a `0` for unchanged parameters.
     */
    event ChangeParameters(
        uint64 indexed poolId,
        uint16 indexed priorityFee,
        uint16 indexed fee,
        uint16 jit
    );

    /**
     * @notice Reduces `feeAssetDec` amount of `asset` and `feeQuoteDec` amount of `quote` from the position's state.
     */
    event Collect(
        uint64 poolId,
        address indexed account,
        uint256 feeAssetDec,
        address indexed asset,
        uint256 feeQuoteDec,
        address indexed quote
    );

    /**
     * @notice Emitted on pair creation.
     */
    event CreatePair(
        uint24 indexed pairId,
        address indexed asset,
        address indexed quote,
        uint8 decimalsAsset,
        uint8 decimalsQuote
    );

    /**
     * @param price Estimated price of the initialized pool in WAD units.
     */
    event CreatePool(
        uint64 indexed poolId,
        bool isMutable,
        address indexed asset,
        address indexed quote,
        uint256 price
    );

    /**
     * @dev Emitted on updating the `protocolFee` state value.
     */
    event UpdateProtocolFee(uint256 prevFee, uint256 nextFee);
}

interface IPortfolioGetters {
    // ===== Account Getters ===== //
    /**
     * @dev Internally owned balance of `token` of `owner`.
     * @return Balance held, in native `token` decimal units.
     */
    function getBalance(
        address owner,
        address token
    ) external view returns (uint256);

    /**
     * @dev Internally tracked global balance of all `token`s assigned to an address or a pool.
     * @return Global balance held, in native `token` decimal units.
     */
    function getReserve(address token) external view returns (uint256);

    /**
     * @notice Difference of `token.balanceOf(this)` and internally tracked reserve balance.
     * @dev Critical system invariant. Must always return greater than or equal to zero.
     * @custom:example
     * ```
     * uint256 previousReserve = getReserve(token);
     * uint256 previousBalance = token.balanceOf(portfolio);
     * assertEq(previousReserve, 1);
     * assertEq(previousBalance, 1);
     * token.transfer(portfolio, 10);
     * uint256 netBalance = getNetBalance(token);
     * assertEq(netBalance, 10);
     * ```
     */
    function getNetBalance(address token) external view returns (int256);

    // ===== State Getters ===== //

    /**
     * @dev Current semantic version of the Portfolio.
     */
    function VERSION() external pure returns (string memory);

    /**
     * @dev Wrapped Ether address initialized on creating the Portfolio.
     */
    function WETH() external view returns (address);

    /**
     * @dev Contract for storing canonical Portfolio deployments.
     */
    function REGISTRY() external view returns (address);

    /**
     * @dev Incremented when a new pair of tokens is made and stored in the `pairs` mapping.
     */
    function getPairNonce() external view returns (uint24);

    /**
     * @dev Incremented when a pool is created.
     */
    function getPoolNonce(uint24 pairNonce) external view returns (uint32);

    /**
     * @dev Reverse lookup to find the `pairId` of a given `asset` and `quote`.
     * Order matters! There can be two pairs for every two tokens.
     */
    function getPairId(
        address asset,
        address quote
    ) external view returns (uint24 pairId);

    function pairs(uint24 pairId)
        external
        view
        returns (
            address tokenAsset,
            uint8 decimalsAsset,
            address tokenQuote,
            uint8 decimalsQuote
        );

    /**
     * @dev Structs in memory are returned as tuples, e.g. (foo, bar...).
     */
    function pools(uint64 poolId)
        external
        view
        returns (
            uint128 virtualX,
            uint128 virtualY,
            uint128 liquidity,
            uint32 lastTimestamp,
            address controller,
            uint256 invariantGrowthGlobal,
            uint256 feeGrowthGlobalAsset,
            uint256 feeGrowthGlobalQuote,
            PortfolioCurve memory,
            PortfolioPair memory
        );

    function positions(
        address owner,
        uint64 poolId
    )
        external
        view
        returns (
            uint128 freeLiquidity,
            uint32 lastTimestamp,
            uint256 invariantGrowthLast,
            uint256 feeGrowthAssetLast,
            uint256 feeGrowthQuoteLast,
            uint128 tokensOwedAsset,
            uint128 tokensOwedQuote,
            uint128 invariantOwed
        );

    // ===== Portfolio View ===== //

    /**
     * @dev Computes amount of `deltaAsset` and `deltaQuote` that must be paid for to
     * mint `deltaLiquidity`.
     */
    function getLiquidityDeltas(
        uint64 poolId,
        int128 deltaLiquidity
    ) external view returns (uint128 deltaAsset, uint128 deltaQuote);

    /**
     * @dev Computes the optimal and max amount of `deltaLiquidity` given two
     * amounts of `deltaAsset` and `deltaQuote`.
     * @param deltaAsset Quantity of `asset` tokens in native decimal units.
     * @param deltaQuote Quantity of `quote` tokens in native decimal units.
     * @return deltaLiquidity Quantity of liquidity minted in wad units.
     */
    function getMaxLiquidity(
        uint64 poolId,
        uint256 deltaAsset,
        uint256 deltaQuote
    ) external view returns (uint128 deltaLiquidity);

    /**
     * @dev Amount of tokens received if all `pool.liquidity` is removed.
     * @return deltaAsset Quantity of `asset` tokens in native decimal units.
     * @return deltaQuote Quantity of `quote` tokens in native decimal units.
     */
    function getPoolReserves(uint64 poolId)
        external
        view
        returns (uint256 deltaAsset, uint256 deltaQuote);

    /**
     * @dev Amount of tokens in native token decimals per WAD liquidity.
     * @return deltaAsset Quantity of `asset` tokens in wad units.
     * @return deltaQuote Quantity of `quote` tokens in wad units.
     */
    function getVirtualReservesPerLiquidity(uint64 poolId)
        external
        view
        returns (uint128 deltaAsset, uint128 deltaQuote);

    // ===== Objective View ===== //

    /**
     * @dev Computes an amount out of tokens given an `amountIn`.
     * @param sellAsset If true, swap `asset` for `quote` tokens.
     * @param amountIn Quantity of tokens to swap in, denominated in native token decimal units.
     * @return amountOut of tokens in native token decimal units.
     */
    function getAmountOut(
        uint64 poolId,
        bool sellAsset,
        uint256 amountIn
    ) external view returns (uint256);

    /**
     * @dev Computes an estimated on-chain price of the `poolId`.
     * @custom:mev Vulnerable to manipulation, do not rely on this function on-chain.
     * @return price Estimated price in wad units of `quote` tokens per `asset` token.
     */
    function getVirtualPrice(uint64 poolId)
        external
        view
        returns (uint256 price);
}

interface IPortfolioActions {
    /**
     * @notice Entrypoint to allocate, deallocate, or swap in Portfolio.
     * @dev Multiprocess expects custom encoded data that can be built off-chain
     * or on-chain using the `FVMLib` library. This function is similar to
     * multicall, which sends calldata to a target by looping over an array of
     * calldatas and targets.
     *
     * The difference is that the transactions in a multicall
     * must setttle token amounts in each call.
     * In multiprocess, token amounts are settled after all calls
     * have been processed.
     *
     * This means that token deficits can be carried over between calls
     * and paid by future ones (within the same multiprocess transaction)!
     *
     * todo: Update multiprocess to return data, or information that can help debugging.
     */
    function multiprocess(bytes calldata data) external payable;

    /**
     * @notice Assigns `amount` of `token` to `msg.sender` internal balance.
     * @dev Uses `IERC20.transferFrom`.
     */
    function fund(address token, uint256 amount) external;

    /**
     * @notice Unassigns `amount` of `token` from `msg.sender` and transfers it to the `to` address.
     * @dev Uses `IERC20.transfer`.
     */
    function draw(address token, uint256 amount, address to) external;

    /**
     * @notice Deposits ether into the `WETH` contract and credits `msg.sender` the received WETH.
     * @dev Amount of ether must be sent as `msg.value`, the ether will be wrapped.
     */
    function deposit() external payable;

    /**
     * @notice Updates the parameters of the pool `poolId`.
     * @dev The sender must be the pool controller, leaving a function parameter
     * as '0' will not change the pool parameter.
     * @param priorityFee New priority fee of the pool in basis points (1 = 0.01%).
     * @param fee New fee of the pool in basis points (1 = 0.01%).
     * @param jit New JIT policy of the pool in seconds (1 = 1 second).
     */
    function changeParameters(
        uint64 poolId,
        uint16 priorityFee,
        uint16 fee,
        uint16 jit
    ) external;

    /**
     * @dev Sets the `protocolFee` state value.
     * @param fee Must be within the range: 4 <= x <= 20.
     */
    function setProtocolFee(uint256 fee) external;
}

interface IPortfolio is
    IPortfolioActions,
    IPortfolioEvents,
    IPortfolioGetters
{ }
