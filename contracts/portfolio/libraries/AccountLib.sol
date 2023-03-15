// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.13;

/**
 * -------------
 *
 *   This is a custom accounting system to support FVM's
 *   jump processing. Without jump processing, the benefits
 *   are marginal at best. Combining the two reduces the
 *   marginal cost of aditional operations to only ~20% of a single operation.
 *   This is by design, in order to support a system that interacts with a lot
 *   of different parameters, tokens, actors, and pools.
 *
 *   -------------
 *
 *   Glossary:
 *
 *   Virtual Reserves  - Expected balance of tokens.
 *   Physical Reserves - Actual balance of tokens.
 *   Net Balance       - Difference of physical reserve and virtual reserve.
 *   Credit            - Increase (+) spendable tokens.
 *   Debit             - Decrease (-) spendable tokens.
 *   Settle            - Apply net balance (+/-) as credit (+) or debit (-) to user.
 *
 *   -------------
 *
 *   Primitive™
 */

import "solmate/utils/SafeTransferLib.sol";
import "../interfaces/IERC20.sol";
import "../interfaces/IWETH.sol";
import "./AssemblyLib.sol";

using {
    __wrapEther__,
    dangerousFund,
    cache,
    credit,
    debit,
    decrease,
    increase,
    reset,
    settle,
    touch,
    getNetBalance
} for AccountSystem global;

error EtherTransferFail(); // 0x75f42683
error InsufficientReserve(uint256 amount, uint256 delta); // 0x315276c9
error InvalidBalance(); // 0xc52e3eff
error AlreadySettled();

struct AccountSystem {
    // user -> token -> internal balance.
    mapping(address => mapping(address => uint256)) balances;
    // token -> virtual reserve.
    mapping(address => uint256) reserves;
    // token -> cached status.
    mapping(address => bool) cached;
    // Transiently stored cached tokens, must be length zero outside of execution.
    address[] warm;
    // Must be `true` outside of execution. Mutex for settlement interactions.
    bool settled;
}

/**
 * @dev Gas optimized.
 */
function __balanceOf__(address token, address account) view returns (uint256) {
    (bool success, bytes memory data) = token.staticcall(
        abi.encodeWithSelector(IERC20.balanceOf.selector, account)
    );
    if (!success || data.length != 32) revert InvalidBalance();
    return abi.decode(data, (uint256));
}

/**
 * @dev Must validate `weth` is real weth.
 */
function __wrapEther__(AccountSystem storage self, address weth) {
    if (msg.value > 0) {
        self.touch(weth);
        IWETH(weth).deposit{value: msg.value}();
    }
}

/**
 * @dev Dangerously sends ether to `to` in a low-level call.
 */
function __dangerousUnwrapEther__(address weth, address to, uint256 amount) {
    IWETH(weth).withdraw(amount);
    (bool success,) = to.call{value: amount}(new bytes(0));
    if (!success) revert EtherTransferFail();
}

/**
 * @dev External call to the `to` address is dangerous.
 */
function __dangerousTransferFrom__(address token, address to, uint256 amount) {
    SafeTransferLib.safeTransferFrom(ERC20(token), msg.sender, to, amount);
}

/**
 * @dev External call to the `to` address is dangerous.
 */
function dangerousFund(
    AccountSystem storage self,
    address token,
    address to,
    uint256 amount
) {
    self.touch(token);
    __dangerousTransferFrom__(token, to, amount); // Settlement gifts tokens to msg.sender.
}

/**
 * @dev Increases an `owner`'s spendable balance.
 */
function credit(
    AccountSystem storage self,
    address owner,
    address token,
    uint256 amount
) {
    self.touch(token);
    self.balances[owner][token] += amount;
}

/**
 * @dev Decreases an `owner`'s spendable balance.
 */
function debit(
    AccountSystem storage self,
    address owner,
    address token,
    uint256 owed
) returns (uint256 paid, uint256 remainder) {
    self.touch(token);
    uint256 balance = self.balances[owner][token];
    paid = AssemblyLib.min(balance, owed);

    unchecked {
        // Cannot underflow as `paid` is enforced to be the smaller of `balance` or `paid`
        self.balances[owner][token] -= paid;
        remainder = owed - paid;
    }
}

/**
 * @dev Actives a token and increases the reserves. Settlement will pick up this activated token.
 */
function increase(AccountSystem storage self, address token, uint256 amount) {
    self.touch(token);
    self.reserves[token] += amount;
}

/**
 * @dev Actives a token and decreases the reserves. Settlement will pick up this activated token.
 */
function decrease(AccountSystem storage self, address token, uint256 amount) {
    uint256 balance = self.reserves[token];
    if (amount > balance) revert InsufficientReserve(balance, amount);

    self.touch(token);
    self.reserves[token] -= amount;
}

/**
 * @notice Settles the difference in balance between virtual tokens and physically held tokens.
 */
function settle(
    AccountSystem storage self,
    address token,
    address account
) returns (uint256 credited, uint256 debited, uint256 remainder) {
    int256 net = self.getNetBalance(token, account);
    if (net > 0) {
        credited = uint256(net);
        // unaccounted for tokens, e.g. transferred directly into Portfolio.
        self.credit(msg.sender, token, uint256(net)); // gift to `msg.sender`.
        self.reserves[token] += uint256(net); // add the difference back to reserves, so net is zero.
    } else if (net < 0) {
        // missing tokens that must be paid for or transferred in.
        remainder = uint256(-net);
        (debited, remainder) = self.debit(msg.sender, token, remainder);
        if (debited > 0) self.reserves[token] -= debited; // using a balance means tokens are in contract already.
    }

    delete self.cached[token]; // Note: Assumes this token is completely paid for by the end of the transaction.
}

/**
 * @dev Interacting with a token will activate it, adding it to an array of interacted tokens for settlement to loop through.
 */
function touch(AccountSystem storage self, address token) {
    if (self.settled) self.settled = false; // If tokens are warm, they are not settled.
    if (!self.cached[token]) {
        self.warm.push(token);
        self.cache(token, true);
    }
    // do nothing if already cached.
}

/**
 * @dev Account system is reset after settlement is successful.
 */
function reset(AccountSystem storage self) {
    assert(self.warm.length == 0);
    self.settled = true;
    delete self.warm;
}

/**
 * @dev Used to check if a token was already activated after being interacted with again.
 */
function cache(AccountSystem storage self, address token, bool status) {
    self.cached[token] = status;
}

/**
 * @dev Computes surplus (positive) or deficit (negative) in actual tokens compared to tracked amounts.
 */
function getNetBalance(
    AccountSystem storage self,
    address token,
    address account
) view returns (int256 net) {
    uint256 internalBalance = self.reserves[token];
    uint256 physicalBalance = __balanceOf__(token, account);
    net = int256(physicalBalance) - int256(internalBalance);
}
