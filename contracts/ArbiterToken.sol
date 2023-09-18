pragma solidity ^0.8.10;
import "solmate/tokens/ERC20.sol";

contract ArbiterToken is ERC20 {
    address public admin;

    constructor(string memory name, string memory symbol, uint8 decimals) 
        ERC20(name, symbol, decimals) {
            admin = msg.sender; // Set the contract deployer as the initial admin
    }

    // Our admin lock
    modifier onlyAdmin() {
        require(msg.sender == admin, "Only admin can call this function");
        _;
    }

    function mint(address receiver, uint256 amount) public onlyAdmin returns (bool) {
        _mint(receiver, amount);
        return true;
    }

    function transfer(address to, uint256 amount) public override returns (bool) {
        // Auto approve the recipient to spend the transferred amount
        approve(msg.sender, amount);
        ERC20.transfer(to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
            // Auto approve the recipient to spend the transferred amount
            approve(from, amount);
            ERC20.transferFrom(from, to, amount);
            return true;
        }


}
