// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.13;

import "solmate/utils/SafeCastLib.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "./libraries/AssemblyLib.sol";
import "./libraries/FVMLib.sol" as FVM;
import "./libraries/AccountLib.sol" as Account;

using AssemblyLib for uint256;
using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;
using SafeCastLib for uint256;

using {
    checkParameters, maturity, validateParameters
} for PortfolioCurve global;
using {
    changePositionLiquidity,
    syncPositionFees,
    getTimeSinceChanged
} for PortfolioPosition global;
using {
    changePoolLiquidity,
    changePoolParameters,
    exists,
    getVirtualReservesPerLiquidity,
    getVirtualPoolReservesPerLiquidityInWad,
    getPoolLiquidityDeltas,
    getPoolMaxLiquidity,
    getPoolReserves,
    isMutable,
    syncPoolTimestamp,
    lastTau,
    computeTau
} for PortfolioPool global;

uint256 constant PERCENTAGE = 10_000;
uint256 constant MIN_MAX_PRICE = 1;
uint256 constant MAX_MAX_PRICE = type(uint128).max;
uint256 constant MIN_FEE = 1; // 0.01%
uint256 constant MAX_FEE = 1000; // 10%
uint256 constant MIN_VOLATILITY = 100; // 1%
uint256 constant MAX_VOLATILITY = 25_000; // 250%
uint256 constant MIN_DURATION = 1; // days, but without units
uint256 constant MAX_DURATION = 500; // days, but without units
uint256 constant JUST_IN_TIME_MAX = 600 seconds;
uint256 constant JUST_IN_TIME_LIQUIDITY_POLICY = 4 seconds;

error DrawBalance();
error InvalidDecimals(uint8 decimals);
error InvalidDuration(uint16);
error InvalidFee(uint16 fee);
error InvalidInstruction();
error InvalidInvariant(int256 prev, int256 next);
error InvalidJit(uint16);
error InvalidPair();
error InvalidReentrancy();
error InvalidSettlement();
error InvalidStrike(uint128 strike);
error InvalidTransfer();
error InvalidVolatility(uint16 sigma);
error JitLiquidity(uint256 distance);
error NegativeBalance(address token, int256 net);
error NotController();
error NonExistentPool(uint64 poolId);
error NonExistentPosition(address owner, uint64 poolId);
error PairExists(uint24 pairId);
error PoolExpired();
error SameTokenError();
error SwapInputTooSmall();
error ZeroAmounts();
error ZeroInput();
error ZeroLiquidity();
error ZeroOutput();
error ZeroPrice();
error ZeroValue();

struct PortfolioPair {
    address tokenAsset; // Base asset, referred to as "X" reserve.
    uint8 decimalsAsset;
    address tokenQuote; // Quote asset, referred to as "Y" reserve.
    uint8 decimalsQuote;
}

struct PortfolioCurve {
    // single slot
    uint128 maxPrice; // Can be used as a terminal price (max price that can be reached by maturity).
    uint16 jit; // Set to a default value in seconds for non-controlled pools.
    uint16 fee; // Can be manipulated by a controller of a pool, if there is one.
    uint16 duration; // Set to max duration for perpetual pools.
    uint16 volatility; // Effects the pool like an amplification factor, increasing price impact of swaps.
    uint16 priorityFee; // Only set for controlled pools, and can be changed by controller.
    uint32 createdAt; // Set to the `block.timestamp` on pool creation.
    bool perpetual; // Set to `true` if the `duration` variable in pool creation is the magic variable type(uint16).max.
}

struct PortfolioPool {
    uint128 virtualX; // WAD x per WAD liquidity.
    uint128 virtualY; // WAD y per WAD liquidity.
    uint128 liquidity; // Total supply of liquidity.
    uint32 lastTimestamp; // The block.timestamp of the last swap.
    address controller; // Address that can change fee, priorityFee, or jit params.
    uint256 invariantGrowthGlobal; // Cumulative sum of positive invariant growth.
    uint256 feeGrowthGlobalAsset; // Cumulative sum of fee's denominated in the `asset` with positive invariant.
    uint256 feeGrowthGlobalQuote; // Cumulative sum of fee's denominated in the `quote` with positive invariant.
    PortfolioCurve params; // Parameters of the objective's trading function.
    PortfolioPair pair; // Token pair data.
}

struct PortfolioPosition {
    uint128 freeLiquidity;
    uint32 lastTimestamp;
    uint256 invariantGrowthLast; // Increases when the invariant increases from a positive value.
    uint256 feeGrowthAssetLast;
    uint256 feeGrowthQuoteLast;
    uint128 tokensOwedAsset;
    uint128 tokensOwedQuote;
    uint128 invariantOwed; // Not used by Portfolio, but can be used by a pool controller.
}

struct ChangeLiquidityParams {
    address owner;
    uint64 poolId;
    uint256 timestamp;
    uint256 deltaAsset;
    uint256 deltaQuote;
    address tokenAsset;
    address tokenQuote;
    int128 deltaLiquidity;
}

struct Order {
    uint8 useMax;
    uint64 poolId;
    uint128 input;
    uint128 output;
    uint8 sellAsset;
}

struct Iteration {
    int256 prevInvariant;
    int256 nextInvariant;
    uint256 virtualX;
    uint256 virtualY;
    uint256 remainder;
    uint256 feeAmount;
    uint256 liquidity;
    uint256 input;
    uint256 output;
}

struct SwapState {
    bool sell;
    address tokenInput;
    uint16 fee;
    address tokenOutput;
    uint256 feeGrowthGlobal;
    uint256 invariantGrowthGlobal;
}

struct Payment {
    address token;
    uint256 amount;
    uint256 balance;
}

// ===== Effects ===== //

function changePoolLiquidity(
    PortfolioPool storage self,
    int128 liquidityDelta
) {
    self.liquidity = AssemblyLib.addSignedDelta(self.liquidity, liquidityDelta);
}

function syncPoolTimestamp(PortfolioPool storage self, uint256 timestamp) {
    self.lastTimestamp = SafeCastLib.safeCastTo32(timestamp);
}

function changePoolParameters(
    PortfolioPool storage self,
    PortfolioCurve memory updated
) {
    // Reverts on invalid parameters.
    updated.validateParameters();
    self.params = updated;
}

function changePositionLiquidity(
    PortfolioPosition storage self,
    uint256 timestamp,
    int128 liquidityDelta
) {
    self.lastTimestamp = uint32(timestamp);
    self.freeLiquidity =
        AssemblyLib.addSignedDelta(self.freeLiquidity, liquidityDelta);
}

/**
 * @dev Liquidity must be altered after syncing positions and not before.
 */
function syncPositionFees(
    PortfolioPosition storage self,
    uint256 feeGrowthAsset,
    uint256 feeGrowthQuote,
    uint256 invariantGrowth
)
    returns (
        uint256 feeAssetEarned,
        uint256 feeQuoteEarned,
        uint256 feeInvariantEarned
    )
{
    // fee growth current - position fee growth last
    uint256 differenceAsset = AssemblyLib.computeCheckpointDistance(
        feeGrowthAsset, self.feeGrowthAssetLast
    );
    uint256 differenceQuote = AssemblyLib.computeCheckpointDistance(
        feeGrowthQuote, self.feeGrowthQuoteLast
    );
    uint256 differenceInvariant = AssemblyLib.computeCheckpointDistance(
        invariantGrowth, self.invariantGrowthLast
    );

    // fee growth per liquidity * position liquidity
    feeAssetEarned =
        FixedPointMathLib.mulWadDown(differenceAsset, self.freeLiquidity);
    feeQuoteEarned =
        FixedPointMathLib.mulWadDown(differenceQuote, self.freeLiquidity);
    feeInvariantEarned =
        FixedPointMathLib.mulWadDown(differenceInvariant, self.freeLiquidity);

    self.feeGrowthAssetLast = feeGrowthAsset;
    self.feeGrowthQuoteLast = feeGrowthQuote;
    self.invariantGrowthLast = invariantGrowth;

    self.tokensOwedAsset += SafeCastLib.safeCastTo128(feeAssetEarned);
    self.tokensOwedQuote += SafeCastLib.safeCastTo128(feeQuoteEarned);
    self.invariantOwed += SafeCastLib.safeCastTo128(feeInvariantEarned);
}

// ===== View ===== //

/**
 * @dev Quantity of tokens in units of their native decimals deallocated if all liquidity was removed.
 */
function getPoolReserves(PortfolioPool memory self)
    pure
    returns (uint128 reserveAsset, uint128 reserveQuote)
{
    return self.getPoolLiquidityDeltas(-int128(self.liquidity)); // Rounds down.
}

/**
 * @dev Maximum amount of liquidity minted given amounts of each token.
 * @param deltaAsset Quantity of `asset` tokens denominated in their native decimals.
 * @param deltaQuote Quantity of `quote` tokens denominated in their native decimals.
 */
function getPoolMaxLiquidity(
    PortfolioPool memory self,
    uint256 deltaAsset,
    uint256 deltaQuote
) pure returns (uint128 deltaLiquidity) {
    deltaAsset = deltaAsset.scaleToWad(self.pair.decimalsAsset).safeCastTo128();
    deltaQuote = deltaQuote.scaleToWad(self.pair.decimalsQuote).safeCastTo128();

    (uint256 amountAssetWad, uint256 amountQuoteWad) =
        self.getVirtualPoolReservesPerLiquidityInWad();
    uint256 liquidity0 = deltaAsset.divWadDown(amountAssetWad); // L_0 = X / (X / L)
    uint256 liquidity1 = deltaQuote.divWadDown(amountQuoteWad); // L_1 = Y / (Y / L)
    deltaLiquidity = AssemblyLib.min(liquidity0, liquidity1).safeCastTo128();
}

/**
 * @dev Rounds positive deltas up. Rounds negative deltas down.
 */
function getPoolLiquidityDeltas(
    PortfolioPool memory self,
    int128 deltaLiquidity
) pure returns (uint128 deltaAsset, uint128 deltaQuote) {
    if (deltaLiquidity == 0) return (deltaAsset, deltaQuote);

    (uint256 amountAssetWad, uint256 amountQuoteWad) =
        self.getVirtualPoolReservesPerLiquidityInWad();
    uint256 scaleDownFactorAsset = AssemblyLib.computeScalar(
        self.pair.decimalsAsset
    ) * FixedPointMathLib.WAD;
    uint256 scaleDownFactorQuote = AssemblyLib.computeScalar(
        self.pair.decimalsQuote
    ) * FixedPointMathLib.WAD;

    uint256 delta;
    if (deltaLiquidity > 0) {
        delta = uint128(deltaLiquidity);
        deltaAsset =
            amountAssetWad.mulDivUp(delta, scaleDownFactorAsset).safeCastTo128();
        deltaQuote =
            amountQuoteWad.mulDivUp(delta, scaleDownFactorQuote).safeCastTo128();
    } else {
        delta = uint128(-deltaLiquidity);
        deltaAsset = amountAssetWad.mulDivDown(delta, scaleDownFactorAsset)
            .safeCastTo128();
        deltaQuote = amountQuoteWad.mulDivDown(delta, scaleDownFactorQuote)
            .safeCastTo128();
    }
}

/**
 * @dev Scales virtual reserves per liquidity from WAD to native token decimal units.
 * @return amountAssetDec Quantity of `asset` tokens in native decimal units per WAD unit of liquidity.
 * @return amountQuoteDec Quantity of `quote` tokens in native decimal units per WAD unit of liquidity.
 */
function getVirtualReservesPerLiquidity(PortfolioPool memory self)
    pure
    returns (uint128 amountAssetDec, uint128 amountQuoteDec)
{
    (uint256 amountAssetWad, uint256 amountQuoteWad) =
        self.getVirtualPoolReservesPerLiquidityInWad();
    amountAssetDec =
        amountAssetWad.scaleFromWadDown(self.pair.decimalsAsset).safeCastTo128();
    amountQuoteDec =
        amountQuoteWad.scaleFromWadDown(self.pair.decimalsQuote).safeCastTo128();
}

/**
 * @dev Virtual reserves of tokens in WAD units per WAD units of liquidity.
 * @return amountAssetWad Quantity of `asset` tokens in WAD units per WAD of liquidity.
 * @return amountQuoteWad Quantity of `quote` tokens in WAD units per WAD of liquidity.
 */
function getVirtualPoolReservesPerLiquidityInWad(PortfolioPool memory self)
    pure
    returns (uint128 amountAssetWad, uint128 amountQuoteWad)
{
    amountAssetWad = self.virtualX;
    amountQuoteWad = self.virtualY;
}

// ===== Derived ===== //

function getTimeSinceChanged(
    PortfolioPosition memory self,
    uint256 timestamp
) pure returns (uint256 distance) {
    return timestamp - self.lastTimestamp;
}

function exists(PortfolioPool memory self) pure returns (bool) {
    return self.lastTimestamp != 0;
}

function isMutable(PortfolioPool memory self) pure returns (bool) {
    return self.controller != address(0);
}

function lastTau(PortfolioPool memory self) pure returns (uint256) {
    return self.computeTau(self.lastTimestamp);
}

function computeTau(
    PortfolioPool memory self,
    uint256 timestamp
) pure returns (uint256) {
    if (self.params.perpetual) return SECONDS_PER_YEAR; // Default to 1 year for perpetual pools.

    uint256 end = self.params.maturity();
    unchecked {
        // Cannot underflow as LHS is either equal to `timestamp` or greater.
        return AssemblyLib.max(timestamp, end) - timestamp;
    }
}

function maturity(PortfolioCurve memory self)
    pure
    returns (uint32 endTimestamp)
{
    unchecked {
        // Portfolio duration is limited such that this addition will never overflow 256 bits.
        endTimestamp = (
            AssemblyLib.convertDaysToSeconds(self.duration) + self.createdAt
        ).safeCastTo32();
    }
}

function validateParameters(PortfolioCurve memory self) pure {
    (bool success, bytes memory reason) = self.checkParameters();
    if (!success) {
        assembly {
            revert(add(32, reason), mload(reason))
        }
    }
}

/**
 * @dev Invalid parameters should revert. Bound checks are inclusive.
 */
function checkParameters(PortfolioCurve memory self)
    pure
    returns (bool, bytes memory)
{
    if (self.jit > JUST_IN_TIME_MAX) {
        return (false, abi.encodeWithSelector(InvalidJit.selector, self.jit));
    }
    if (!AssemblyLib.isBetween(self.volatility, MIN_VOLATILITY, MAX_VOLATILITY))
    {
        return (
            false,
            abi.encodeWithSelector(InvalidVolatility.selector, self.volatility)
        );
    }
    if (!AssemblyLib.isBetween(self.duration, MIN_DURATION, MAX_DURATION)) {
        return (
            false,
            abi.encodeWithSelector(InvalidDuration.selector, self.duration)
        );
    }
    if (!AssemblyLib.isBetween(self.maxPrice, MIN_MAX_PRICE, MAX_MAX_PRICE)) {
        return (
            false, abi.encodeWithSelector(InvalidStrike.selector, self.maxPrice)
        );
    }
    if (!AssemblyLib.isBetween(self.fee, MIN_FEE, MAX_FEE)) {
        return (false, abi.encodeWithSelector(InvalidFee.selector, self.fee));
    }
    // 0 priority fee == no controller, impossible to set to zero unless default from non controlled pools.
    if (!AssemblyLib.isBetween(self.priorityFee, 0, self.fee)) {
        return (
            false, abi.encodeWithSelector(InvalidFee.selector, self.priorityFee)
        );
    }

    return (true, "");
}
