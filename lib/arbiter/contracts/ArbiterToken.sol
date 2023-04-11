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

    function mint(address receiver, uint256 amount) public onlyAdmin {
        _mint(receiver, amount);
    }

    function mintMax(address receiver) public onlyAdmin {
        _mint(receiver, type(uint256).max);
    }

    function approve(address spender, uint256 amount) public virtual override returns (bool) {
        super.approve(spender, amount);
        return true;
    }
}