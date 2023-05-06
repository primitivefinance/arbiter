pub use portfolio_virtual::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod portfolio_virtual {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"EtherTransferFail\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FindTheRevert\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientLiquidity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delta\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InsufficientReserve\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidBalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidDecimals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"prev\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"next\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidInvariant\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidNegativeLiquidity\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidPair\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidReentrancy\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSettlement\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"distance\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"JitLiquidity\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MaxDeltaReached\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MinDeltaUnmatched\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"net\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NegativeBalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"type\":\"error\",\"name\":\"NonExistentPool\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotController\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[]}],\"type\":\"error\",\"name\":\"PairExists\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PoolExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SameTokenError\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAmounts\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroInput\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroLiquidity\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroOutput\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroPrice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaLiquidity\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Allocate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ChangeParameters\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ClaimFees\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"decimalsAsset\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"decimalsQuote\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CreatePair\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"controller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"maxPrice\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"duration\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"volatility\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CreatePool\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaLiquidity\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deallocate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DecreaseReserveBalance\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deposit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IncreaseReserveBalance\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"input\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"output\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feeAmountDec\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"invariantWad\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Swap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prevFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdateProtocolFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REGISTRY\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"VERSION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"__account__\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"settled\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"useMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaLiquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"maxDeltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"maxDeltaQuote\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"allocate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeParameters\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"invariant\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveX\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkInvariant\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"nextInvariant\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkPool\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"delta\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkPosition\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"sellAsset\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"computeMaxInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"computeReservesFromPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"reserveX\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveY\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"createPair\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"controller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"volatility\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"duration\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"maxPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"price\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"createPool\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"useMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaLiquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"minDeltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"minDeltaQuote\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deallocate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"sellAsset\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"swapper\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOut\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"output\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"deltaLiquidity\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLiquidityDeltas\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"deltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaQuote\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxLiquidity\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"deltaLiquidity\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNetBalance\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPairId\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPairNonce\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVirtualPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVirtualReservesDec\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"deltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaQuote\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pairs\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"tokenAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsAsset\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenQuote\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsQuote\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pools\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"virtualX\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"virtualY\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"lastTimestamp\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"controller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct PortfolioCurve\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"maxPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"duration\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"volatility\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"createdAt\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"perpetual\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"struct PortfolioPair\",\"name\":\"pair\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsAsset\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenQuote\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsQuote\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"positions\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"freeLiquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"lastTimestamp\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"protocolFees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setProtocolFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct Order\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bool\",\"name\":\"useMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"input\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"output\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"sellAsset\",\"type\":\"bool\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"input\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"output\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PORTFOLIOVIRTUAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct PortfolioVirtual<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PortfolioVirtual<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PortfolioVirtual<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PortfolioVirtual<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PortfolioVirtual<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PortfolioVirtual))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PortfolioVirtual<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PORTFOLIOVIRTUAL_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `REGISTRY` (0x06433b1b) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([6, 67, 59, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `__account__` (0xda31ee54) function
        pub fn account(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([218, 49, 238, 84], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocate` (0x94c32b4a) function
        pub fn allocate(
            &self,
            use_max: bool,
            pool_id: u64,
            delta_liquidity: u128,
            max_delta_asset: u128,
            max_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [148, 195, 43, 74],
                    (
                        use_max,
                        pool_id,
                        delta_liquidity,
                        max_delta_asset,
                        max_delta_quote,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeParameters` (0xaf777855) function
        pub fn change_parameters(
            &self,
            pool_id: u64,
            priority_fee: u16,
            fee: u16,
            jit: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 119, 120, 85], (pool_id, priority_fee, fee, jit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkInvariant` (0x2f337da5) function
        pub fn check_invariant(
            &self,
            pool_id: u64,
            invariant: ::ethers::core::types::I256,
            reserve_x: ::ethers::core::types::U256,
            reserve_y: ::ethers::core::types::U256,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, ::ethers::core::types::I256)>
        {
            self.0
                .method_hash(
                    [47, 51, 125, 165],
                    (pool_id, invariant, reserve_x, reserve_y, timestamp),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkPool` (0xa68aaa41) function
        pub fn check_pool(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 138, 170, 65], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkPosition` (0x2cc6641e) function
        pub fn check_position(
            &self,
            pool_id: u64,
            owner: ::ethers::core::types::Address,
            delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([44, 198, 100, 30], (pool_id, owner, delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimFee` (0xdda40797) function
        pub fn claim_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 164, 7, 151], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeMaxInput` (0x989bafba) function
        pub fn compute_max_input(
            &self,
            pool_id: u64,
            sell_asset: bool,
            reserve_in: ::ethers::core::types::U256,
            liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [152, 155, 175, 186],
                    (pool_id, sell_asset, reserve_in, liquidity),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeReservesFromPrice` (0xc48d887a) function
        pub fn compute_reserves_from_price(
            &self,
            pool_id: u64,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([196, 141, 136, 122], (pool_id, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPair` (0xc9c65396) function
        pub fn create_pair(
            &self,
            asset: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([201, 198, 83, 150], (asset, quote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPool` (0x40b54e8e) function
        pub fn create_pool(
            &self,
            pair_id: u32,
            controller: ::ethers::core::types::Address,
            priority_fee: u16,
            fee: u16,
            volatility: u16,
            duration: u16,
            jit: u16,
            max_price: u128,
            price: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash(
                    [64, 181, 78, 142],
                    (
                        pair_id,
                        controller,
                        priority_fee,
                        fee,
                        volatility,
                        duration,
                        jit,
                        max_price,
                        price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0x5bc55464) function
        pub fn deallocate(
            &self,
            use_max: bool,
            pool_id: u64,
            delta_liquidity: u128,
            min_delta_asset: u128,
            min_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [91, 197, 84, 100],
                    (
                        use_max,
                        pool_id,
                        delta_liquidity,
                        min_delta_asset,
                        min_delta_quote,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x19057807) function
        pub fn get_amount_out(
            &self,
            pool_id: u64,
            sell_asset: bool,
            amount_in: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 5, 120, 7], (pool_id, sell_asset, amount_in, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityDeltas` (0x8992f20a) function
        pub fn get_liquidity_deltas(
            &self,
            pool_id: u64,
            delta_liquidity: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([137, 146, 242, 10], (pool_id, delta_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxLiquidity` (0xd6b7dec5) function
        pub fn get_max_liquidity(
            &self,
            pool_id: u64,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([214, 183, 222, 197], (pool_id, amount_0, amount_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNetBalance` (0x4dc68a90) function
        pub fn get_net_balance(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([77, 198, 138, 144], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairId` (0x3f92a339) function
        pub fn get_pair_id(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([63, 146, 163, 57], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairNonce` (0x078888d6) function
        pub fn get_pair_nonce(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([7, 136, 136, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolNonce` (0xa5cd8a49) function
        pub fn get_pool_nonce(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([165, 205, 138, 73], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolReserves` (0x2afb9df8) function
        pub fn get_pool_reserves(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([42, 251, 157, 248], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserve` (0xc9a396e9) function
        pub fn get_reserve(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 163, 150, 233], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVirtualPrice` (0x61b7ea6a) function
        pub fn get_virtual_price(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 183, 234, 106], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVirtualReservesDec` (0xa21b9ba0) function
        pub fn get_virtual_reserves_dec(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([162, 27, 155, 160], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairs` (0x5e47663c) function
        pub fn pairs(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                u8,
                ::ethers::core::types::Address,
                u8,
            ),
        > {
            self.0
                .method_hash([94, 71, 102, 60], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0x89a5f084) function
        pub fn pools(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                u128,
                u128,
                u32,
                ::ethers::core::types::Address,
                PortfolioCurve,
                PortfolioPair,
            ),
        > {
            self.0
                .method_hash([137, 165, 240, 132], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positions` (0xb68513ea) function
        pub fn positions(
            &self,
            p0: ::ethers::core::types::Address,
            p1: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u32)> {
            self.0
                .method_hash([182, 133, 19, 234], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFees` (0xdcf844a7) function
        pub fn protocol_fees(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 248, 68, 167], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFee` (0x787dce3d) function
        pub fn set_protocol_fee(
            &self,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 125, 206, 61], fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x64f14ef2) function
        pub fn swap(
            &self,
            args: Order,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u64,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([100, 241, 78, 242], (args,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AllocateFilter> {
            self.0.event()
        }
        ///Gets the contract's `ChangeParameters` event
        pub fn change_parameters_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeParametersFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ClaimFees` event
        pub fn claim_fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClaimFeesFilter> {
            self.0.event()
        }
        ///Gets the contract's `CreatePair` event
        pub fn create_pair_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CreatePairFilter> {
            self.0.event()
        }
        ///Gets the contract's `CreatePool` event
        pub fn create_pool_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CreatePoolFilter> {
            self.0.event()
        }
        ///Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeallocateFilter> {
            self.0.event()
        }
        ///Gets the contract's `DecreaseReserveBalance` event
        pub fn decrease_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DecreaseReserveBalanceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncreaseReserveBalance` event
        pub fn increase_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IncreaseReserveBalanceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `UpdateProtocolFee` event
        pub fn update_protocol_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpdateProtocolFeeFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PortfolioVirtualEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PortfolioVirtual<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EtherTransferFail` with signature `EtherTransferFail()` and selector `0x75f42683`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EtherTransferFail", abi = "EtherTransferFail()")]
    pub struct EtherTransferFail;
    ///Custom Error type `FindTheRevert` with signature `FindTheRevert()` and selector `0x1ca82207`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FindTheRevert", abi = "FindTheRevert()")]
    pub struct FindTheRevert;
    ///Custom Error type `InsufficientLiquidity` with signature `InsufficientLiquidity()` and selector `0xbb55fd27`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InsufficientLiquidity", abi = "InsufficientLiquidity()")]
    pub struct InsufficientLiquidity;
    ///Custom Error type `InsufficientReserve` with signature `InsufficientReserve(uint256,uint256)` and selector `0x315276c9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "InsufficientReserve",
        abi = "InsufficientReserve(uint256,uint256)"
    )]
    pub struct InsufficientReserve {
        pub amount: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidBalance` with signature `InvalidBalance()` and selector `0xc52e3eff`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidBalance", abi = "InvalidBalance()")]
    pub struct InvalidBalance;
    ///Custom Error type `InvalidDecimals` with signature `InvalidDecimals(uint8)` and selector `0xca950391`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidDecimals", abi = "InvalidDecimals(uint8)")]
    pub struct InvalidDecimals {
        pub decimals: u8,
    }
    ///Custom Error type `InvalidFee` with signature `InvalidFee(uint16)` and selector `0xf6f4a38f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidFee", abi = "InvalidFee(uint16)")]
    pub struct InvalidFee {
        pub fee: u16,
    }
    ///Custom Error type `InvalidInvariant` with signature `InvalidInvariant(int256,int256)` and selector `0x2125a168`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidInvariant", abi = "InvalidInvariant(int256,int256)")]
    pub struct InvalidInvariant {
        pub prev: ::ethers::core::types::I256,
        pub next: ::ethers::core::types::I256,
    }
    ///Custom Error type `InvalidNegativeLiquidity` with signature `InvalidNegativeLiquidity()` and selector `0x25c776da`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidNegativeLiquidity", abi = "InvalidNegativeLiquidity()")]
    pub struct InvalidNegativeLiquidity;
    ///Custom Error type `InvalidPair` with signature `InvalidPair()` and selector `0x1e4f7d8c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidPair", abi = "InvalidPair()")]
    pub struct InvalidPair;
    ///Custom Error type `InvalidReentrancy` with signature `InvalidReentrancy()` and selector `0xffc72209`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidReentrancy", abi = "InvalidReentrancy()")]
    pub struct InvalidReentrancy;
    ///Custom Error type `InvalidSettlement` with signature `InvalidSettlement()` and selector `0x115931c4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidSettlement", abi = "InvalidSettlement()")]
    pub struct InvalidSettlement;
    ///Custom Error type `JitLiquidity` with signature `JitLiquidity(uint256)` and selector `0x9a231b2c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "JitLiquidity", abi = "JitLiquidity(uint256)")]
    pub struct JitLiquidity {
        pub distance: ::ethers::core::types::U256,
    }
    ///Custom Error type `MaxDeltaReached` with signature `MaxDeltaReached()` and selector `0xdd4c0942`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MaxDeltaReached", abi = "MaxDeltaReached()")]
    pub struct MaxDeltaReached;
    ///Custom Error type `MinDeltaUnmatched` with signature `MinDeltaUnmatched()` and selector `0x95160bab`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MinDeltaUnmatched", abi = "MinDeltaUnmatched()")]
    pub struct MinDeltaUnmatched;
    ///Custom Error type `NegativeBalance` with signature `NegativeBalance(address,int256)` and selector `0xfe239baa`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NegativeBalance", abi = "NegativeBalance(address,int256)")]
    pub struct NegativeBalance {
        pub token: ::ethers::core::types::Address,
        pub net: ::ethers::core::types::I256,
    }
    ///Custom Error type `NonExistentPool` with signature `NonExistentPool(uint64)` and selector `0xd4480d46`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NonExistentPool", abi = "NonExistentPool(uint64)")]
    pub struct NonExistentPool {
        pub pool_id: u64,
    }
    ///Custom Error type `NotController` with signature `NotController()` and selector `0x23019e67`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotController", abi = "NotController()")]
    pub struct NotController;
    ///Custom Error type `PairExists` with signature `PairExists(uint24)` and selector `0x3325fa77`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PairExists", abi = "PairExists(uint24)")]
    pub struct PairExists {
        pub pair_id: u32,
    }
    ///Custom Error type `PoolExpired` with signature `PoolExpired()` and selector `0x398b36db`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PoolExpired", abi = "PoolExpired()")]
    pub struct PoolExpired;
    ///Custom Error type `SameTokenError` with signature `SameTokenError()` and selector `0xec38b794`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SameTokenError", abi = "SameTokenError()")]
    pub struct SameTokenError;
    ///Custom Error type `ZeroAmounts` with signature `ZeroAmounts()` and selector `0x213c7cc5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroAmounts", abi = "ZeroAmounts()")]
    pub struct ZeroAmounts;
    ///Custom Error type `ZeroInput` with signature `ZeroInput()` and selector `0xaf458c07`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroInput", abi = "ZeroInput()")]
    pub struct ZeroInput;
    ///Custom Error type `ZeroLiquidity` with signature `ZeroLiquidity()` and selector `0x10074548`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroLiquidity", abi = "ZeroLiquidity()")]
    pub struct ZeroLiquidity;
    ///Custom Error type `ZeroOutput` with signature `ZeroOutput()` and selector `0xe618637e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroOutput", abi = "ZeroOutput()")]
    pub struct ZeroOutput;
    ///Custom Error type `ZeroPrice` with signature `ZeroPrice()` and selector `0x4dfba023`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroPrice", abi = "ZeroPrice()")]
    pub struct ZeroPrice;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PortfolioVirtualErrors {
        EtherTransferFail(EtherTransferFail),
        FindTheRevert(FindTheRevert),
        InsufficientLiquidity(InsufficientLiquidity),
        InsufficientReserve(InsufficientReserve),
        InvalidBalance(InvalidBalance),
        InvalidDecimals(InvalidDecimals),
        InvalidFee(InvalidFee),
        InvalidInvariant(InvalidInvariant),
        InvalidNegativeLiquidity(InvalidNegativeLiquidity),
        InvalidPair(InvalidPair),
        InvalidReentrancy(InvalidReentrancy),
        InvalidSettlement(InvalidSettlement),
        JitLiquidity(JitLiquidity),
        MaxDeltaReached(MaxDeltaReached),
        MinDeltaUnmatched(MinDeltaUnmatched),
        NegativeBalance(NegativeBalance),
        NonExistentPool(NonExistentPool),
        NotController(NotController),
        PairExists(PairExists),
        PoolExpired(PoolExpired),
        SameTokenError(SameTokenError),
        ZeroAmounts(ZeroAmounts),
        ZeroInput(ZeroInput),
        ZeroLiquidity(ZeroLiquidity),
        ZeroOutput(ZeroOutput),
        ZeroPrice(ZeroPrice),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PortfolioVirtualErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <EtherTransferFail as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EtherTransferFail(decoded));
            }
            if let Ok(decoded) = <FindTheRevert as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FindTheRevert(decoded));
            }
            if let Ok(decoded) =
                <InsufficientLiquidity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientLiquidity(decoded));
            }
            if let Ok(decoded) =
                <InsufficientReserve as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientReserve(decoded));
            }
            if let Ok(decoded) = <InvalidBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidBalance(decoded));
            }
            if let Ok(decoded) = <InvalidDecimals as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidDecimals(decoded));
            }
            if let Ok(decoded) = <InvalidFee as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidFee(decoded));
            }
            if let Ok(decoded) = <InvalidInvariant as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInvariant(decoded));
            }
            if let Ok(decoded) =
                <InvalidNegativeLiquidity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidNegativeLiquidity(decoded));
            }
            if let Ok(decoded) = <InvalidPair as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidPair(decoded));
            }
            if let Ok(decoded) = <InvalidReentrancy as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidReentrancy(decoded));
            }
            if let Ok(decoded) = <InvalidSettlement as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidSettlement(decoded));
            }
            if let Ok(decoded) = <JitLiquidity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::JitLiquidity(decoded));
            }
            if let Ok(decoded) = <MaxDeltaReached as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxDeltaReached(decoded));
            }
            if let Ok(decoded) = <MinDeltaUnmatched as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinDeltaUnmatched(decoded));
            }
            if let Ok(decoded) = <NegativeBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NegativeBalance(decoded));
            }
            if let Ok(decoded) = <NonExistentPool as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonExistentPool(decoded));
            }
            if let Ok(decoded) = <NotController as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotController(decoded));
            }
            if let Ok(decoded) = <PairExists as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PairExists(decoded));
            }
            if let Ok(decoded) = <PoolExpired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolExpired(decoded));
            }
            if let Ok(decoded) = <SameTokenError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SameTokenError(decoded));
            }
            if let Ok(decoded) = <ZeroAmounts as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAmounts(decoded));
            }
            if let Ok(decoded) = <ZeroInput as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroInput(decoded));
            }
            if let Ok(decoded) = <ZeroLiquidity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroLiquidity(decoded));
            }
            if let Ok(decoded) = <ZeroOutput as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroOutput(decoded));
            }
            if let Ok(decoded) = <ZeroPrice as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PortfolioVirtualErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EtherTransferFail(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindTheRevert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidDecimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidInvariant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidNegativeLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPair(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidReentrancy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSettlement(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::JitLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxDeltaReached(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinDeltaUnmatched(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NonExistentPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotController(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairExists(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolExpired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SameTokenError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroAmounts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroInput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroOutput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PortfolioVirtualErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EtherTransferFail as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FindTheRevert as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidity as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientReserve as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidFee as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidInvariant as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidNegativeLiquidity as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidPair as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidReentrancy as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidSettlement as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <JitLiquidity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <MaxDeltaReached as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MinDeltaUnmatched as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NegativeBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NonExistentPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <PairExists as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <PoolExpired as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SameTokenError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <ZeroAmounts as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ZeroInput as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ZeroLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <ZeroOutput as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ZeroPrice as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PortfolioVirtualErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EtherTransferFail(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindTheRevert(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNegativeLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPair(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReentrancy(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSettlement(element) => ::core::fmt::Display::fmt(element, f),
                Self::JitLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDeltaReached(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinDeltaUnmatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonExistentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotController(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::SameTokenError(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PortfolioVirtualErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EtherTransferFail> for PortfolioVirtualErrors {
        fn from(value: EtherTransferFail) -> Self {
            Self::EtherTransferFail(value)
        }
    }
    impl ::core::convert::From<FindTheRevert> for PortfolioVirtualErrors {
        fn from(value: FindTheRevert) -> Self {
            Self::FindTheRevert(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for PortfolioVirtualErrors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InsufficientReserve> for PortfolioVirtualErrors {
        fn from(value: InsufficientReserve) -> Self {
            Self::InsufficientReserve(value)
        }
    }
    impl ::core::convert::From<InvalidBalance> for PortfolioVirtualErrors {
        fn from(value: InvalidBalance) -> Self {
            Self::InvalidBalance(value)
        }
    }
    impl ::core::convert::From<InvalidDecimals> for PortfolioVirtualErrors {
        fn from(value: InvalidDecimals) -> Self {
            Self::InvalidDecimals(value)
        }
    }
    impl ::core::convert::From<InvalidFee> for PortfolioVirtualErrors {
        fn from(value: InvalidFee) -> Self {
            Self::InvalidFee(value)
        }
    }
    impl ::core::convert::From<InvalidInvariant> for PortfolioVirtualErrors {
        fn from(value: InvalidInvariant) -> Self {
            Self::InvalidInvariant(value)
        }
    }
    impl ::core::convert::From<InvalidNegativeLiquidity> for PortfolioVirtualErrors {
        fn from(value: InvalidNegativeLiquidity) -> Self {
            Self::InvalidNegativeLiquidity(value)
        }
    }
    impl ::core::convert::From<InvalidPair> for PortfolioVirtualErrors {
        fn from(value: InvalidPair) -> Self {
            Self::InvalidPair(value)
        }
    }
    impl ::core::convert::From<InvalidReentrancy> for PortfolioVirtualErrors {
        fn from(value: InvalidReentrancy) -> Self {
            Self::InvalidReentrancy(value)
        }
    }
    impl ::core::convert::From<InvalidSettlement> for PortfolioVirtualErrors {
        fn from(value: InvalidSettlement) -> Self {
            Self::InvalidSettlement(value)
        }
    }
    impl ::core::convert::From<JitLiquidity> for PortfolioVirtualErrors {
        fn from(value: JitLiquidity) -> Self {
            Self::JitLiquidity(value)
        }
    }
    impl ::core::convert::From<MaxDeltaReached> for PortfolioVirtualErrors {
        fn from(value: MaxDeltaReached) -> Self {
            Self::MaxDeltaReached(value)
        }
    }
    impl ::core::convert::From<MinDeltaUnmatched> for PortfolioVirtualErrors {
        fn from(value: MinDeltaUnmatched) -> Self {
            Self::MinDeltaUnmatched(value)
        }
    }
    impl ::core::convert::From<NegativeBalance> for PortfolioVirtualErrors {
        fn from(value: NegativeBalance) -> Self {
            Self::NegativeBalance(value)
        }
    }
    impl ::core::convert::From<NonExistentPool> for PortfolioVirtualErrors {
        fn from(value: NonExistentPool) -> Self {
            Self::NonExistentPool(value)
        }
    }
    impl ::core::convert::From<NotController> for PortfolioVirtualErrors {
        fn from(value: NotController) -> Self {
            Self::NotController(value)
        }
    }
    impl ::core::convert::From<PairExists> for PortfolioVirtualErrors {
        fn from(value: PairExists) -> Self {
            Self::PairExists(value)
        }
    }
    impl ::core::convert::From<PoolExpired> for PortfolioVirtualErrors {
        fn from(value: PoolExpired) -> Self {
            Self::PoolExpired(value)
        }
    }
    impl ::core::convert::From<SameTokenError> for PortfolioVirtualErrors {
        fn from(value: SameTokenError) -> Self {
            Self::SameTokenError(value)
        }
    }
    impl ::core::convert::From<ZeroAmounts> for PortfolioVirtualErrors {
        fn from(value: ZeroAmounts) -> Self {
            Self::ZeroAmounts(value)
        }
    }
    impl ::core::convert::From<ZeroInput> for PortfolioVirtualErrors {
        fn from(value: ZeroInput) -> Self {
            Self::ZeroInput(value)
        }
    }
    impl ::core::convert::From<ZeroLiquidity> for PortfolioVirtualErrors {
        fn from(value: ZeroLiquidity) -> Self {
            Self::ZeroLiquidity(value)
        }
    }
    impl ::core::convert::From<ZeroOutput> for PortfolioVirtualErrors {
        fn from(value: ZeroOutput) -> Self {
            Self::ZeroOutput(value)
        }
    }
    impl ::core::convert::From<ZeroPrice> for PortfolioVirtualErrors {
        fn from(value: ZeroPrice) -> Self {
            Self::ZeroPrice(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Allocate",
        abi = "Allocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ChangeParameters",
        abi = "ChangeParameters(uint64,uint16,uint16,uint16)"
    )]
    pub struct ChangeParametersFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub priority_fee: u16,
        #[ethevent(indexed)]
        pub fee: u16,
        pub jit: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ClaimFees", abi = "ClaimFees(address,uint256)")]
    pub struct ClaimFeesFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "CreatePair",
        abi = "CreatePair(uint24,address,address,uint8,uint8)"
    )]
    pub struct CreatePairFilter {
        #[ethevent(indexed)]
        pub pair_id: u32,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub decimals_quote: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "CreatePool",
        abi = "CreatePool(uint64,address,address,address,uint128,uint16,uint16,uint16,uint16,uint16)"
    )]
    pub struct CreatePoolFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub controller: ::ethers::core::types::Address,
        pub max_price: u128,
        pub jit: u16,
        pub fee: u16,
        pub duration: u16,
        pub volatility: u16,
        pub priority_fee: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Deallocate",
        abi = "Deallocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "DecreaseReserveBalance",
        abi = "DecreaseReserveBalance(address,uint256)"
    )]
    pub struct DecreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "IncreaseReserveBalance",
        abi = "IncreaseReserveBalance(address,uint256)"
    )]
    pub struct IncreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(uint64,uint256,address,uint256,address,uint256,uint256,int256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        pub price: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        pub input: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub output: ::ethers::core::types::U256,
        pub fee_amount_dec: ::ethers::core::types::U256,
        pub invariant_wad: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "UpdateProtocolFee", abi = "UpdateProtocolFee(uint256,uint256)")]
    pub struct UpdateProtocolFeeFilter {
        pub prev_fee: ::ethers::core::types::U256,
        pub next_fee: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PortfolioVirtualEvents {
        AllocateFilter(AllocateFilter),
        ChangeParametersFilter(ChangeParametersFilter),
        ClaimFeesFilter(ClaimFeesFilter),
        CreatePairFilter(CreatePairFilter),
        CreatePoolFilter(CreatePoolFilter),
        DeallocateFilter(DeallocateFilter),
        DecreaseReserveBalanceFilter(DecreaseReserveBalanceFilter),
        DepositFilter(DepositFilter),
        IncreaseReserveBalanceFilter(IncreaseReserveBalanceFilter),
        SwapFilter(SwapFilter),
        UpdateProtocolFeeFilter(UpdateProtocolFeeFilter),
    }
    impl ::ethers::contract::EthLogDecode for PortfolioVirtualEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = ChangeParametersFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::ChangeParametersFilter(decoded));
            }
            if let Ok(decoded) = ClaimFeesFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::ClaimFeesFilter(decoded));
            }
            if let Ok(decoded) = CreatePairFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::CreatePairFilter(decoded));
            }
            if let Ok(decoded) = CreatePoolFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::CreatePoolFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = DecreaseReserveBalanceFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::DecreaseReserveBalanceFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = IncreaseReserveBalanceFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::IncreaseReserveBalanceFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = UpdateProtocolFeeFilter::decode_log(log) {
                return Ok(PortfolioVirtualEvents::UpdateProtocolFeeFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PortfolioVirtualEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeParametersFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePairFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePoolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProtocolFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for PortfolioVirtualEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<ChangeParametersFilter> for PortfolioVirtualEvents {
        fn from(value: ChangeParametersFilter) -> Self {
            Self::ChangeParametersFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFeesFilter> for PortfolioVirtualEvents {
        fn from(value: ClaimFeesFilter) -> Self {
            Self::ClaimFeesFilter(value)
        }
    }
    impl ::core::convert::From<CreatePairFilter> for PortfolioVirtualEvents {
        fn from(value: CreatePairFilter) -> Self {
            Self::CreatePairFilter(value)
        }
    }
    impl ::core::convert::From<CreatePoolFilter> for PortfolioVirtualEvents {
        fn from(value: CreatePoolFilter) -> Self {
            Self::CreatePoolFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for PortfolioVirtualEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseReserveBalanceFilter> for PortfolioVirtualEvents {
        fn from(value: DecreaseReserveBalanceFilter) -> Self {
            Self::DecreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for PortfolioVirtualEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseReserveBalanceFilter> for PortfolioVirtualEvents {
        fn from(value: IncreaseReserveBalanceFilter) -> Self {
            Self::IncreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for PortfolioVirtualEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<UpdateProtocolFeeFilter> for PortfolioVirtualEvents {
        fn from(value: UpdateProtocolFeeFilter) -> Self {
            Self::UpdateProtocolFeeFilter(value)
        }
    }
    ///Container type for all input parameters for the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "REGISTRY", abi = "REGISTRY()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `__account__` function with signature `__account__()` and selector `0xda31ee54`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "__account__", abi = "__account__()")]
    pub struct AccountCall;
    ///Container type for all input parameters for the `allocate` function with signature `allocate(bool,uint64,uint128,uint128,uint128)` and selector `0x94c32b4a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "allocate",
        abi = "allocate(bool,uint64,uint128,uint128,uint128)"
    )]
    pub struct AllocateCall {
        pub use_max: bool,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub max_delta_asset: u128,
        pub max_delta_quote: u128,
    }
    ///Container type for all input parameters for the `changeParameters` function with signature `changeParameters(uint64,uint16,uint16,uint16)` and selector `0xaf777855`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "changeParameters",
        abi = "changeParameters(uint64,uint16,uint16,uint16)"
    )]
    pub struct ChangeParametersCall {
        pub pool_id: u64,
        pub priority_fee: u16,
        pub fee: u16,
        pub jit: u16,
    }
    ///Container type for all input parameters for the `checkInvariant` function with signature `checkInvariant(uint64,int256,uint256,uint256,uint256)` and selector `0x2f337da5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "checkInvariant",
        abi = "checkInvariant(uint64,int256,uint256,uint256,uint256)"
    )]
    pub struct CheckInvariantCall {
        pub pool_id: u64,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkPool` function with signature `checkPool(uint64)` and selector `0xa68aaa41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "checkPool", abi = "checkPool(uint64)")]
    pub struct CheckPoolCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `checkPosition` function with signature `checkPosition(uint64,address,int256)` and selector `0x2cc6641e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "checkPosition", abi = "checkPosition(uint64,address,int256)")]
    pub struct CheckPositionCall {
        pub pool_id: u64,
        pub owner: ::ethers::core::types::Address,
        pub delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `claimFee` function with signature `claimFee(address,uint256)` and selector `0xdda40797`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "claimFee", abi = "claimFee(address,uint256)")]
    pub struct ClaimFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeMaxInput` function with signature `computeMaxInput(uint64,bool,uint256,uint256)` and selector `0x989bafba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "computeMaxInput",
        abi = "computeMaxInput(uint64,bool,uint256,uint256)"
    )]
    pub struct ComputeMaxInputCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub reserve_in: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeReservesFromPrice` function with signature `computeReservesFromPrice(uint64,uint256)` and selector `0xc48d887a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "computeReservesFromPrice",
        abi = "computeReservesFromPrice(uint64,uint256)"
    )]
    pub struct ComputeReservesFromPriceCall {
        pub pool_id: u64,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub asset: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(uint24,address,uint16,uint16,uint16,uint16,uint16,uint128,uint128)` and selector `0x40b54e8e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "createPool",
        abi = "createPool(uint24,address,uint16,uint16,uint16,uint16,uint16,uint128,uint128)"
    )]
    pub struct CreatePoolCall {
        pub pair_id: u32,
        pub controller: ::ethers::core::types::Address,
        pub priority_fee: u16,
        pub fee: u16,
        pub volatility: u16,
        pub duration: u16,
        pub jit: u16,
        pub max_price: u128,
        pub price: u128,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "deallocate",
        abi = "deallocate(bool,uint64,uint128,uint128,uint128)"
    )]
    pub struct DeallocateCall {
        pub use_max: bool,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub min_delta_asset: u128,
        pub min_delta_quote: u128,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getAmountOut",
        abi = "getAmountOut(uint64,bool,uint256,address)"
    )]
    pub struct GetAmountOutCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLiquidityDeltas", abi = "getLiquidityDeltas(uint64,int128)")]
    pub struct GetLiquidityDeltasCall {
        pub pool_id: u64,
        pub delta_liquidity: i128,
    }
    ///Container type for all input parameters for the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getMaxLiquidity",
        abi = "getMaxLiquidity(uint64,uint256,uint256)"
    )]
    pub struct GetMaxLiquidityCall {
        pub pool_id: u64,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNetBalance", abi = "getNetBalance(address)")]
    pub struct GetNetBalanceCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPairId", abi = "getPairId(address,address)")]
    pub struct GetPairIdCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPairNonce", abi = "getPairNonce()")]
    pub struct GetPairNonceCall;
    ///Container type for all input parameters for the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPoolNonce", abi = "getPoolNonce(uint24)")]
    pub struct GetPoolNonceCall(pub u32);
    ///Container type for all input parameters for the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPoolReserves", abi = "getPoolReserves(uint64)")]
    pub struct GetPoolReservesCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getReserve", abi = "getReserve(address)")]
    pub struct GetReserveCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getVirtualPrice` function with signature `getVirtualPrice(uint64)` and selector `0x61b7ea6a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getVirtualPrice", abi = "getVirtualPrice(uint64)")]
    pub struct GetVirtualPriceCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getVirtualReservesDec` function with signature `getVirtualReservesDec(uint64)` and selector `0xa21b9ba0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getVirtualReservesDec", abi = "getVirtualReservesDec(uint64)")]
    pub struct GetVirtualReservesDecCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairs", abi = "pairs(uint24)")]
    pub struct PairsCall(pub u32);
    ///Container type for all input parameters for the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pools", abi = "pools(uint64)")]
    pub struct PoolsCall(pub u64);
    ///Container type for all input parameters for the `positions` function with signature `positions(address,uint64)` and selector `0xb68513ea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "positions", abi = "positions(address,uint64)")]
    pub struct PositionsCall(pub ::ethers::core::types::Address, pub u64);
    ///Container type for all input parameters for the `protocolFees` function with signature `protocolFees(address)` and selector `0xdcf844a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "protocolFees", abi = "protocolFees(address)")]
    pub struct ProtocolFeesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `setProtocolFee` function with signature `setProtocolFee(uint256)` and selector `0x787dce3d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setProtocolFee", abi = "setProtocolFee(uint256)")]
    pub struct SetProtocolFeeCall {
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap((bool,uint64,uint128,uint128,bool))` and selector `0x64f14ef2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "swap", abi = "swap((bool,uint64,uint128,uint128,bool))")]
    pub struct SwapCall {
        pub args: Order,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PortfolioVirtualCalls {
        Registry(RegistryCall),
        Version(VersionCall),
        Weth(WethCall),
        Account(AccountCall),
        Allocate(AllocateCall),
        ChangeParameters(ChangeParametersCall),
        CheckInvariant(CheckInvariantCall),
        CheckPool(CheckPoolCall),
        CheckPosition(CheckPositionCall),
        ClaimFee(ClaimFeeCall),
        ComputeMaxInput(ComputeMaxInputCall),
        ComputeReservesFromPrice(ComputeReservesFromPriceCall),
        CreatePair(CreatePairCall),
        CreatePool(CreatePoolCall),
        Deallocate(DeallocateCall),
        GetAmountOut(GetAmountOutCall),
        GetLiquidityDeltas(GetLiquidityDeltasCall),
        GetMaxLiquidity(GetMaxLiquidityCall),
        GetNetBalance(GetNetBalanceCall),
        GetPairId(GetPairIdCall),
        GetPairNonce(GetPairNonceCall),
        GetPoolNonce(GetPoolNonceCall),
        GetPoolReserves(GetPoolReservesCall),
        GetReserve(GetReserveCall),
        GetVirtualPrice(GetVirtualPriceCall),
        GetVirtualReservesDec(GetVirtualReservesDecCall),
        Multicall(MulticallCall),
        Pairs(PairsCall),
        Pools(PoolsCall),
        Positions(PositionsCall),
        ProtocolFees(ProtocolFeesCall),
        SetProtocolFee(SetProtocolFeeCall),
        Swap(SwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for PortfolioVirtualCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Registry(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded) = <AccountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Account(decoded));
            }
            if let Ok(decoded) = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) =
                <ChangeParametersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChangeParameters(decoded));
            }
            if let Ok(decoded) =
                <CheckInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckInvariant(decoded));
            }
            if let Ok(decoded) = <CheckPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckPool(decoded));
            }
            if let Ok(decoded) = <CheckPositionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckPosition(decoded));
            }
            if let Ok(decoded) = <ClaimFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimFee(decoded));
            }
            if let Ok(decoded) =
                <ComputeMaxInputCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeMaxInput(decoded));
            }
            if let Ok(decoded) =
                <ComputeReservesFromPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeReservesFromPrice(decoded));
            }
            if let Ok(decoded) = <CreatePairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePair(decoded));
            }
            if let Ok(decoded) = <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePool(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidityDeltasCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidityDeltas(decoded));
            }
            if let Ok(decoded) =
                <GetMaxLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMaxLiquidity(decoded));
            }
            if let Ok(decoded) = <GetNetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNetBalance(decoded));
            }
            if let Ok(decoded) = <GetPairIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPairId(decoded));
            }
            if let Ok(decoded) = <GetPairNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPairNonce(decoded));
            }
            if let Ok(decoded) = <GetPoolNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolNonce(decoded));
            }
            if let Ok(decoded) =
                <GetPoolReservesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolReserves(decoded));
            }
            if let Ok(decoded) = <GetReserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserve(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetVirtualPrice(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualReservesDecCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetVirtualReservesDec(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <PairsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pairs(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <PositionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Positions(decoded));
            }
            if let Ok(decoded) = <ProtocolFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProtocolFees(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProtocolFee(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PortfolioVirtualCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Registry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Account(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChangeParameters(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckInvariant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckPosition(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeMaxInput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeReservesFromPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePair(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreatePool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deallocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLiquidityDeltas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPairId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPairNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolReserves(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReserve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVirtualPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVirtualReservesDec(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pairs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProtocolFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProtocolFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PortfolioVirtualCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Registry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Account(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeParameters(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeMaxInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeReservesFromPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePair(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidityDeltas(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVirtualPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVirtualReservesDec(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RegistryCall> for PortfolioVirtualCalls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<VersionCall> for PortfolioVirtualCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WethCall> for PortfolioVirtualCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<AccountCall> for PortfolioVirtualCalls {
        fn from(value: AccountCall) -> Self {
            Self::Account(value)
        }
    }
    impl ::core::convert::From<AllocateCall> for PortfolioVirtualCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<ChangeParametersCall> for PortfolioVirtualCalls {
        fn from(value: ChangeParametersCall) -> Self {
            Self::ChangeParameters(value)
        }
    }
    impl ::core::convert::From<CheckInvariantCall> for PortfolioVirtualCalls {
        fn from(value: CheckInvariantCall) -> Self {
            Self::CheckInvariant(value)
        }
    }
    impl ::core::convert::From<CheckPoolCall> for PortfolioVirtualCalls {
        fn from(value: CheckPoolCall) -> Self {
            Self::CheckPool(value)
        }
    }
    impl ::core::convert::From<CheckPositionCall> for PortfolioVirtualCalls {
        fn from(value: CheckPositionCall) -> Self {
            Self::CheckPosition(value)
        }
    }
    impl ::core::convert::From<ClaimFeeCall> for PortfolioVirtualCalls {
        fn from(value: ClaimFeeCall) -> Self {
            Self::ClaimFee(value)
        }
    }
    impl ::core::convert::From<ComputeMaxInputCall> for PortfolioVirtualCalls {
        fn from(value: ComputeMaxInputCall) -> Self {
            Self::ComputeMaxInput(value)
        }
    }
    impl ::core::convert::From<ComputeReservesFromPriceCall> for PortfolioVirtualCalls {
        fn from(value: ComputeReservesFromPriceCall) -> Self {
            Self::ComputeReservesFromPrice(value)
        }
    }
    impl ::core::convert::From<CreatePairCall> for PortfolioVirtualCalls {
        fn from(value: CreatePairCall) -> Self {
            Self::CreatePair(value)
        }
    }
    impl ::core::convert::From<CreatePoolCall> for PortfolioVirtualCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for PortfolioVirtualCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for PortfolioVirtualCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetLiquidityDeltasCall> for PortfolioVirtualCalls {
        fn from(value: GetLiquidityDeltasCall) -> Self {
            Self::GetLiquidityDeltas(value)
        }
    }
    impl ::core::convert::From<GetMaxLiquidityCall> for PortfolioVirtualCalls {
        fn from(value: GetMaxLiquidityCall) -> Self {
            Self::GetMaxLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNetBalanceCall> for PortfolioVirtualCalls {
        fn from(value: GetNetBalanceCall) -> Self {
            Self::GetNetBalance(value)
        }
    }
    impl ::core::convert::From<GetPairIdCall> for PortfolioVirtualCalls {
        fn from(value: GetPairIdCall) -> Self {
            Self::GetPairId(value)
        }
    }
    impl ::core::convert::From<GetPairNonceCall> for PortfolioVirtualCalls {
        fn from(value: GetPairNonceCall) -> Self {
            Self::GetPairNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolNonceCall> for PortfolioVirtualCalls {
        fn from(value: GetPoolNonceCall) -> Self {
            Self::GetPoolNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolReservesCall> for PortfolioVirtualCalls {
        fn from(value: GetPoolReservesCall) -> Self {
            Self::GetPoolReserves(value)
        }
    }
    impl ::core::convert::From<GetReserveCall> for PortfolioVirtualCalls {
        fn from(value: GetReserveCall) -> Self {
            Self::GetReserve(value)
        }
    }
    impl ::core::convert::From<GetVirtualPriceCall> for PortfolioVirtualCalls {
        fn from(value: GetVirtualPriceCall) -> Self {
            Self::GetVirtualPrice(value)
        }
    }
    impl ::core::convert::From<GetVirtualReservesDecCall> for PortfolioVirtualCalls {
        fn from(value: GetVirtualReservesDecCall) -> Self {
            Self::GetVirtualReservesDec(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for PortfolioVirtualCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<PairsCall> for PortfolioVirtualCalls {
        fn from(value: PairsCall) -> Self {
            Self::Pairs(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for PortfolioVirtualCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for PortfolioVirtualCalls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<ProtocolFeesCall> for PortfolioVirtualCalls {
        fn from(value: ProtocolFeesCall) -> Self {
            Self::ProtocolFees(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeCall> for PortfolioVirtualCalls {
        fn from(value: SetProtocolFeeCall) -> Self {
            Self::SetProtocolFee(value)
        }
    }
    impl ::core::convert::From<SwapCall> for PortfolioVirtualCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    ///Container type for all return fields from the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `__account__` function with signature `__account__()` and selector `0xda31ee54`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AccountReturn {
        pub settled: bool,
    }
    ///Container type for all return fields from the `allocate` function with signature `allocate(bool,uint64,uint128,uint128,uint128)` and selector `0x94c32b4a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AllocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `checkInvariant` function with signature `checkInvariant(uint64,int256,uint256,uint256,uint256)` and selector `0x2f337da5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckInvariantReturn {
        pub success: bool,
        pub next_invariant: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `checkPool` function with signature `checkPool(uint64)` and selector `0xa68aaa41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckPoolReturn(pub bool);
    ///Container type for all return fields from the `checkPosition` function with signature `checkPosition(uint64,address,int256)` and selector `0x2cc6641e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckPositionReturn(pub bool);
    ///Container type for all return fields from the `computeMaxInput` function with signature `computeMaxInput(uint64,bool,uint256,uint256)` and selector `0x989bafba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ComputeMaxInputReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computeReservesFromPrice` function with signature `computeReservesFromPrice(uint64,uint256)` and selector `0xc48d887a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ComputeReservesFromPriceReturn {
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreatePairReturn {
        pub pair_id: u32,
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(uint24,address,uint16,uint16,uint16,uint16,uint16,uint128,uint128)` and selector `0x40b54e8e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreatePoolReturn {
        pub pool_id: u64,
    }
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DeallocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAmountOutReturn {
        pub output: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLiquidityDeltasReturn {
        pub delta_asset: u128,
        pub delta_quote: u128,
    }
    ///Container type for all return fields from the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetMaxLiquidityReturn {
        pub delta_liquidity: u128,
    }
    ///Container type for all return fields from the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNetBalanceReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPairIdReturn(pub u32);
    ///Container type for all return fields from the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPairNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPoolNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPoolReservesReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetReserveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVirtualPrice` function with signature `getVirtualPrice(uint64)` and selector `0x61b7ea6a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetVirtualPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getVirtualReservesDec` function with signature `getVirtualReservesDec(uint64)` and selector `0xa21b9ba0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetVirtualReservesDecReturn {
        pub delta_asset: u128,
        pub delta_quote: u128,
    }
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairsReturn {
        pub token_asset: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub token_quote: ::ethers::core::types::Address,
        pub decimals_quote: u8,
    }
    ///Container type for all return fields from the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolsReturn {
        pub virtual_x: u128,
        pub virtual_y: u128,
        pub liquidity: u128,
        pub last_timestamp: u32,
        pub controller: ::ethers::core::types::Address,
        pub params: PortfolioCurve,
        pub pair: PortfolioPair,
    }
    ///Container type for all return fields from the `positions` function with signature `positions(address,uint64)` and selector `0xb68513ea`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PositionsReturn {
        pub free_liquidity: u128,
        pub last_timestamp: u32,
    }
    ///Container type for all return fields from the `protocolFees` function with signature `protocolFees(address)` and selector `0xdcf844a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProtocolFeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `swap` function with signature `swap((bool,uint64,uint128,uint128,bool))` and selector `0x64f14ef2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapReturn {
        pub pool_id: u64,
        pub input: ::ethers::core::types::U256,
        pub output: ::ethers::core::types::U256,
    }
}
