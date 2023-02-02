pragma solidity ^0.8.10;

interface MetaStablePool {
    event AmpUpdateStarted(uint256 startValue, uint256 endValue, uint256 startTime, uint256 endTime);
    event AmpUpdateStopped(uint256 currentValue);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event OracleEnabledChanged(bool enabled);
    event PausedStateChanged(bool paused);
    event PriceRateCacheUpdated(address indexed token, uint256 rate);
    event PriceRateProviderSet(address indexed token, address indexed provider, uint256 cacheDuration);
    event SwapFeePercentageChanged(uint256 swapFeePercentage);
    event Transfer(address indexed from, address indexed to, uint256 value);

    struct SwapRequest { uint8 a; address b; address c; uint256 d; bytes32 e; uint256 f; address g; address h; bytes i; }

    function DOMAIN_SEPARATOR() view external returns (bytes32);
    function allowance(address owner, address spender) view external returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function balanceOf(address account) view external returns (uint256);
    function decimals() view external returns (uint8);
    function decreaseAllowance(address spender, uint256 amount) external returns (bool);
    function enableOracle() external;
    function getActionId(bytes4 selector) view external returns (bytes32);
    function getAmplificationParameter() view external returns (uint256 value, bool isUpdating, uint256 precision);
    function getAuthorizer() view external returns (address);
    function getLargestSafeQueryWindow() pure external returns (uint256);
    function getLastInvariant() view external returns (uint256 lastInvariant, uint256 lastInvariantAmp);
    function getLatest(uint8 variable) view external returns (uint256);
    function getOracleMiscData() view external returns (int256 logInvariant, int256 logTotalSupply, uint256 oracleSampleCreationTimestamp, uint256 oracleIndex, bool oracleEnabled);
    function getOwner() view external returns (address);
    // function getPastAccumulators( (uint8, uint256)[] memory queries) view external returns (int256[] memory results);
    function getPausedState() view external returns (bool paused, uint256 pauseWindowEndTime, uint256 bufferPeriodEndTime);
    function getPoolId() view external returns (bytes32);
    function getPriceRateCache(address token) view external returns (uint256 rate, uint256 duration, uint256 expires);
    function getRate() view external returns (uint256);
    function getRateProviders() view external returns (address[] memory providers);
    function getSample(uint256 index) view external returns (int256 logPairPrice, int256 accLogPairPrice, int256 logBptPrice, int256 accLogBptPrice, int256 logInvariant, int256 accLogInvariant, uint256 timestamp);
    function getScalingFactors() view external returns (uint256[] memory);
    function getSwapFeePercentage() view external returns (uint256);
    // function getTimeWeightedAverage((uint8,uint256,uint256)[] memory queries) view external returns (uint256[] memory results);
    function getTotalSamples() pure external returns (uint256);
    function getVault() view external returns (address);
    function increaseAllowance(address spender, uint256 addedValue) external returns (bool);
    function name() view external returns (string memory);
    function nonces(address owner) view external returns (uint256);
    function onExitPool(bytes32 poolId, address sender, address recipient, uint256[] memory balances, uint256 lastChangeBlock, uint256 protocolSwapFeePercentage, bytes memory userData) external returns (uint256[] memory amountsOut, uint256[] memory dueProtocolFeeAmounts);
    function onJoinPool(bytes32 poolId, address sender, address recipient, uint256[] memory balances, uint256 lastChangeBlock, uint256 protocolSwapFeePercentage, bytes memory userData) external returns (uint256[] memory amountsIn, uint256[] memory dueProtocolFeeAmounts);
    function onSwap(SwapRequest memory request, uint256[] memory balances, uint256 indexIn, uint256 indexOut) external returns (uint256);
    function onSwap(SwapRequest memory request, uint256 balanceTokenIn, uint256 balanceTokenOut) external returns (uint256);
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function queryExit(bytes32 poolId, address sender, address recipient, uint256[] memory balances, uint256 lastChangeBlock, uint256 protocolSwapFeePercentage, bytes memory userData) external returns (uint256 bptIn, uint256[] memory amountsOut);
    function queryJoin(bytes32 poolId, address sender, address recipient, uint256[] memory balances, uint256 lastChangeBlock, uint256 protocolSwapFeePercentage, bytes memory userData) external returns (uint256 bptOut, uint256[] memory amountsIn);
    function setAssetManagerPoolConfig(address token, bytes memory poolConfig) external;
    function setPaused(bool paused) external;
    function setPriceRateCacheDuration(address token, uint256 duration) external;
    function setSwapFeePercentage(uint256 swapFeePercentage) external;
    function startAmplificationParameterUpdate(uint256 rawEndValue, uint256 endTime) external;
    function stopAmplificationParameterUpdate() external;
    function symbol() view external returns (string memory);
    function totalSupply() view external returns (uint256);
    function transfer(address recipient, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    function updatePriceRateCache(address token) external;
}

