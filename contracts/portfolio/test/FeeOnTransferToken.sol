pragma solidity ^0.8.4;

import "solmate/test/utils/mocks/MockERC20.sol";

contract FeeOnTransferToken is MockERC20 {
    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals
    ) MockERC20(name_, symbol_, decimals) { }

    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) public override returns (bool) {
        super.transferFrom(from, to, amount);
        // DO NOT CHANGE THIS FEE
        // ALL THE FEE TRANSFER RELATED TESTS ASSUME THE FEE IS 1%
        uint256 fee = amount * 1 / 100;
        balanceOf[to] -= fee;
        return true;
    }
}
