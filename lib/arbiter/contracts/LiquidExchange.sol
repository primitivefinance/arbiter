// SPDX-License-Identifier: MIT
// compiler version must be greater than or equal to 0.8.17 and less than 0.9.0
pragma solidity ^0.8.17;
// import "solmate/utils/FixedPointMathLib.sol"; // This import is correct given Arbiter's foundry.toml
// import "solmate/utils/FixedPointMathLib.sol"; // This import goes directly to the contract
// import "solmate/tokens/ERC20.sol";
import "../../portfolio/lib/solmate/src/tokens/ERC20.sol";
import "../../portfolio/lib/solmate/src/utils/FixedPointMathLib.sol";
import "./ArbiterToken.sol";

/**
 * @dev Implementation of the test interface for Arbiter writing contracts.
 */
contract LiquidExchange {
    using FixedPointMathLib for int256;
    using FixedPointMathLib for uint256;
    address public admin;
    address public arbiterTokenX;
    address public arbiterTokenY;
    uint256 public price;
    uint256 public constant WAD = 10**18;

    // Each LiquidExchange contract will be deployed with a pair of token addresses and an initial price
    constructor(address arbiterTokenX_, address arbiterTokenY_, uint256 price_) {
        admin = msg.sender; // Set the contract deployer as the initial admin
        arbiterTokenX = arbiterTokenX_;
        arbiterTokenY = arbiterTokenY_;
        price = price_;
    }

    // Our admin lock
    modifier onlyAdmin() {
        require(msg.sender == admin, "Only admin can call this function");
        _;
    }

    event PriceChange(uint256 price);
    event Swap(address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, address to);

    // Admin only function to set the price of x in terms of y
    function setPrice(uint256 _price) public onlyAdmin {
        price = _price;
        emit PriceChange(price);
    }

    function swap(address tokenIn, uint256 amountIn) public returns (uint256 amountOut){
        uint256 amountOut;
        address tokenOut;
        if (tokenIn == arbiterTokenX) {
            tokenOut = arbiterTokenY;
            amountOut = FixedPointMathLib.mulWadDown(amountIn, price);
        } else if (tokenIn == arbiterTokenY) {
            tokenOut = arbiterTokenX;
            amountOut = FixedPointMathLib.divWadDown(amountIn, price);
        } else {
            revert("Invalid token");
        }
        // require(ArbiterToken(tokenIn).approve(msg.sender, amountIn), "Approval failed");
        // require(ArbiterToken(tokenOut).approve(admin, amountOut), "Approval failed");
        require(ERC20(tokenIn).transferFrom(msg.sender, admin, amountIn), "Transfer failed");
        require(ERC20(tokenOut).transferFrom(admin, msg.sender, amountOut), "Transfer failed");
        emit Swap(tokenIn, tokenOut, amountIn, amountOut, msg.sender);    
        return amountOut;
    //         require(ArbiterToken(tokenIn).approve(address(this), amountIn), "Approval failed");
    // require(ERC20(tokenIn).transferFrom(msg.sender, admin, amountIn), "Transfer failed");
    // ArbiterToken(tokenOut).mint(msg.sender, amountOut);
    // emit Swap(tokenIn, tokenOut, amountIn, amountOut, msg.sender);    
    // return amountOut;
    }
}
