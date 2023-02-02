pragma solidity ^0.8.10;

interface Vault {
    event AuthorizerChanged(address indexed newAuthorizer);
    event ExternalBalanceTransfer(address indexed token, address indexed sender, address recipient, uint256 amount);
    event FlashLoan(address indexed recipient, address indexed token, uint256 amount, uint256 feeAmount);
    event InternalBalanceChanged(address indexed user, address indexed token, int256 delta);
    event PausedStateChanged(bool paused);
    event PoolBalanceChanged(bytes32 indexed poolId, address indexed liquidityProvider, address[] tokens, int256[] deltas, uint256[] protocolFeeAmounts);
    event PoolBalanceManaged(bytes32 indexed poolId, address indexed assetManager, address indexed token, int256 cashDelta, int256 managedDelta);
    event PoolRegistered(bytes32 indexed poolId, address indexed poolAddress, uint8 specialization);
    event RelayerApprovalChanged(address indexed relayer, address indexed sender, bool approved);
    event Swap(bytes32 indexed poolId, address indexed tokenIn, address indexed tokenOut, uint256 amountIn, uint256 amountOut);
    event TokensDeregistered(bytes32 indexed poolId, address[] tokens);
    event TokensRegistered(bytes32 indexed poolId, address[] tokens, address[] assetManagers);

    struct ExitPoolRequest { address[] a; uint256[] b; bytes c; bool d; }
    struct FundManagement { address a; bool b; address c; bool d; }
    struct SingleSwap { bytes32 a; uint8 b; address c; address d; uint256 e; bytes f; }
    struct JoinPoolRequest { address[] a; uint256[] b; bytes c; bool d; }

    function WETH() view external returns (address);
    // function batchSwap(uint8 kind, (bytes32,uint256,uint256,uint256,bytes)[] memory swaps, address[] memory assets, FundManagement memory funds, int256[] memory limits, uint256 deadline) payable external returns (int256[] memory assetDeltas);
    function deregisterTokens(bytes32 poolId, address[] memory tokens) external;
    function exitPool(bytes32 poolId, address sender, address recipient, ExitPoolRequest memory request) external;
    function flashLoan(address recipient, address[] memory tokens, uint256[] memory amounts, bytes memory userData) external;
    function getActionId(bytes4 selector) view external returns (bytes32);
    function getAuthorizer() view external returns (address);
    function getDomainSeparator() view external returns (bytes32);
    function getInternalBalance(address user, address[] memory tokens) view external returns (uint256[] memory balances);
    function getNextNonce(address user) view external returns (uint256);
    function getPausedState() view external returns (bool paused, uint256 pauseWindowEndTime, uint256 bufferPeriodEndTime);
    function getPool(bytes32 poolId) view external returns (address, uint8);
    function getPoolTokenInfo(bytes32 poolId, address token) view external returns (uint256 cash, uint256 managed, uint256 lastChangeBlock, address assetManager);
    function getPoolTokens(bytes32 poolId) view external returns (address[] memory tokens, uint256[] memory balances, uint256 lastChangeBlock);
    function getProtocolFeesCollector() view external returns (address);
    function hasApprovedRelayer(address user, address relayer) view external returns (bool);
    function joinPool(bytes32 poolId, address sender, address recipient, JoinPoolRequest memory request) payable external;
    // function managePoolBalance((uint8,bytes32,address,uint256)[] memory ops) external;
    // function manageUserBalance((uint8,address,uint256,address,address)[] memory ops) payable external;
    // function queryBatchSwap(uint8 kind, (bytes32,uint256,uint256,uint256,bytes)[] memory swaps, address[] memory assets, FundManagement memory funds) external returns (int256[] memory);
    function registerPool(uint8 specialization) external returns (bytes32);
    function registerTokens(bytes32 poolId, address[] memory tokens, address[] memory assetManagers) external;
    function setAuthorizer(address newAuthorizer) external;
    function setPaused(bool paused) external;
    function setRelayerApproval(address sender, address relayer, bool approved) external;
    function swap(SingleSwap memory singleSwap, FundManagement memory funds, uint256 limit, uint256 deadline) payable external returns (uint256 amountCalculated);
}

