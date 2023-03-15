// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.13;

error InvalidLiquidity();
error InvalidDays();

uint256 constant SECONDS_PER_YEAR = 31556953 seconds;
uint256 constant SECONDS_PER_DAY = 86_400 seconds;
uint8 constant MIN_DECIMALS = 6;
uint8 constant MAX_DECIMALS = 18;

/**
 * @title   AssemblyLib
 * @author  Primitive™
 * @notice  Yul implementations of the most used functions in Portfolio.
 * @dev     Free functions are nice for user-defined types since `using for` can utilize the `global` keyword.
 *          Since this library only implements functions for native solidity types, it's better as a `library`.
 */
library AssemblyLib {
    /**
     * @dev Returns true if `value` is between a range defined by `lower` and `upper` bounds.
     * @param value Value to compare.
     * @param lower Inclusive lower bound of the range.
     * @param upper Inclusive upper bound of the range.
     * @return valid True if the value is between the range.
     * @custom:example
     * ```
     * bool valid = isBetween(50, 0, 100);
     * assertTrue(value);
     * ```
     */
    function isBetween(
        uint256 value,
        uint256 lower,
        uint256 upper
    ) internal pure returns (bool valid) {
        assembly {
            valid :=
                and(
                    or(eq(value, lower), gt(value, lower)),
                    or(eq(value, upper), lt(value, upper))
                )
        }
    }

    /**
     * @dev Returns the smaller of two numbers.
     */
    function min(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // Equivalent to `result = (a < b) ? a : b`
        assembly {
            result := sub(a, mul(sub(a, b), gt(a, b)))
        }
    }

    /**
     * @dev Returns the larger of two numbers.
     */
    function max(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // Equivalent to: `result = (a < b) ? b : a`
        assembly {
            result := sub(a, mul(sub(a, b), lt(a, b)))
        }
    }

    /**
     * @dev Adds a signed `delta` to an unsigned `input` by using the sign-agnostic 256-bit type used in yul.
     * Checks for overflow manually and reverts with `InvalidLiquidity()`, because this function is used to
     * change liquidity in a position or pool.
     * @custom:example
     * ```
     * uint128 output = addSignedDelta(uint128(15), -int128(5));
     * assertEq(output, uint128(10));
     * ```
     */
    function addSignedDelta(
        uint128 input,
        int128 delta
    ) internal pure returns (uint128 output) {
        bytes memory revertData =
            abi.encodeWithSelector(InvalidLiquidity.selector);
        assembly {
            output := add(input, delta)
            // Reverts on overflow.
            if gt(output, 0xffffffffffffffffffffffffffffffff) {
                revert(add(32, revertData), mload(revertData))
            } // 0x1fff9681
        }
    }

    /**
     * @dev Adds a `delta` to a cumulative value and returns the result. Each result is used as a
     * fee `checkpoint`. Positions compute fees earned using checkpoints by measuring the difference between
     * when positions were entered and when they were exited. Since these are cumulative values,
     * computing the difference with a checkpoint that overflows is no different from computing the difference
     * between checkpoints that have not overflowed.
     * @custom:example
     * ```
     * uint256 checkpoint = computeCheckpoint(100, 22);
     * assertEq(checkpoint, 122);
     * ```
     */
    function computeCheckpoint(
        uint256 present,
        uint256 delta
    ) internal pure returns (uint256 checkpoint) {
        // Overflow by design, as these are checkpoints, which can measure the distance even if overflowed.
        assembly {
            checkpoint := add(present, delta)
        }
    }

    /**
     * @notice Computes the difference between two checkpoints.
     * @dev Underflows.
     * @custom:example
     * ```
     * uint256 distance = computeCheckpointDistance(50, 25);
     * assertEq(distance, 25);
     * ```
     */
    function computeCheckpointDistance(
        uint256 present,
        uint256 past
    ) internal pure returns (uint256 distance) {
        // Underflow by design, as these are checkpoints which can measure the distance even if underflowed.
        assembly {
            distance := sub(present, past)
        }
    }

    /**
     * @notice Days units are used in Portfolio because they fit into an unsigned 16-bit integer and they
     * are human readable.
     * @dev Reverts on overflow.
     */
    function convertDaysToSeconds(uint256 amountDays)
        internal
        pure
        returns (uint256 amountSeconds)
    {
        bytes memory revertData = abi.encodeWithSelector(InvalidDays.selector);
        assembly {
            amountSeconds := mul(amountDays, SECONDS_PER_DAY)
            // Reverts on overflow.
            if gt(amountSeconds, 0xffffffffffffffffffffffffffffffff) {
                revert(add(32, revertData), mload(revertData))
            }
        }
    }

    /**
     * @dev There's no explict casting from dynamic to fixed sized bytes, this function
     * handles it for us.
     */
    function toBytes16(bytes memory raw) internal pure returns (bytes16 data) {
        assembly {
            data := mload(add(raw, 32))
            let shift := mul(sub(16, mload(raw)), 8)
            data := shr(shift, data)
        }
    }

    /**
     * @dev There's no explict casting from dynamic to fixed sized bytes, this function
     * handles it for us.
     */
    function toBytes8(bytes memory raw) internal pure returns (bytes8 data) {
        assembly {
            data := mload(add(raw, 32))
            let shift := mul(sub(8, mload(raw)), 8)
            data := shr(shift, data)
        }
    }

    /**
     * @dev Safely casts an unsigned 128-bit integer into a signed 128-bit integer.
     * Reverts on overflow.
     */
    function toInt128(uint128 a) internal pure returns (int128 b) {
        assembly {
            // Reverts on overflow.
            if gt(a, 0x7fffffffffffffffffffffffffffffff) { revert(0, 0) }

            b := a
        }
    }

    /**
     * @dev Separates the upper and lower bits of a byte.
     * @param data Byte to separate.
     * @return upper Upper bit of the byte.
     * @return lower Lower bit of the byte.
     * @custom:example
     * ```
     * (bytes1 upper, bytes1 lower) = separate(0x12);
     * ```
     */
    function separate(bytes1 data)
        internal
        pure
        returns (bytes1 upper, bytes1 lower)
    {
        upper = data >> 4;
        lower = data & 0x0f;
    }

    /**
     * @dev Packs the upper and lower bits of a byte.
     * @param upper Upper bit of the byte.
     * @param lower Lower bit of the byte.
     * @return data Packed byte.
     * @custom:example
     * ```
     * bytes1 0x21 = pack(0x20, 0x01);
     * ```
     */
    function pack(
        bytes1 upper,
        bytes1 lower
    ) internal pure returns (bytes1 data) {
        data = (upper << 4) | lower;
    }

    /**
     * @dev Converts an array of bytes into an uint128, the array must adhere
     * to the the following format:
     * - First byte: Amount of trailing zeros.
     * - Rest of the array: A hexadecimal number.
     */
    function toAmount(bytes calldata raw)
        internal
        pure
        returns (uint128 amount)
    {
        uint8 power = uint8(raw[0]);
        amount = uint128(toBytes16(raw[1:raw.length]));
        if (power != 0) amount = amount * uint128(10 ** power);
    }

    function fromAmount(uint128 amount)
        internal
        pure
        returns (uint8 power, uint128 base)
    {
        if (amount == 0) return (0, 0);
        base = amount;

        while (base % 10 == 0) {
            ++power;
            base /= 10;
        }
    }

    /**
     * @dev Scalars are used to convert token amounts between the token's decimal units
     * and WAD (10^18) units.
     */
    function computeScalar(uint256 decimals)
        internal
        pure
        returns (uint256 scalar)
    {
        return 10 ** (MAX_DECIMALS - decimals); // can revert on underflow
    }

    function scaleToWad(
        uint256 amountDec,
        uint256 decimals
    ) internal pure returns (uint256 outputWad) {
        uint256 factor = computeScalar(decimals);
        assembly {
            outputWad := mul(amountDec, factor)
        }
    }

    function scaleFromWadUp(
        uint256 amountWad,
        uint256 decimals
    ) internal pure returns (uint256 outputDec) {
        uint256 factor = computeScalar(decimals);
        assembly {
            outputDec := add(div(sub(amountWad, 1), factor), 1) // ((a-1) / b) + 1
        }
    }

    function scaleFromWadDown(
        uint256 amountWad,
        uint256 decimals
    ) internal pure returns (uint256 outputDec) {
        uint256 factor = computeScalar(decimals);
        assembly {
            outputDec := div(amountWad, factor)
        }
    }

    function scaleFromWadUpSigned(
        int256 amountWad,
        uint256 decimals
    ) internal pure returns (int256 outputDec) {
        int256 factor = int256(computeScalar(decimals));
        assembly {
            outputDec := add(sdiv(sub(amountWad, 1), factor), 1) // ((a-1) / b) + 1
        }
    }

    function scaleFromWadDownSigned(
        int256 amountWad,
        uint256 decimals
    ) internal pure returns (int256 outputDec) {
        int256 factor = int256(computeScalar(decimals));
        assembly {
            outputDec := sdiv(amountWad, factor)
        }
    }

    /*

    /!\ Do not use this function, it's completely cursed:
    Using this function twice in a row will result in the first output being
    assigned the value of the second output...

    function trimBytes(bytes memory input) internal pure returns (bytes memory output) {
        assembly {
            let length := mload(input)
            let value := mload(add(0x20, input))

            for {} 1 {} {
                let s := byte(0, value)

                switch iszero(s)
                    case 0 { break }
                    case 1 {
                        value := shl(8, value)
                        length := sub(length, 1)
                    }
            }

            mstore(output, length)
            mstore(add(0x20, output), value)
        }
    }
    */
}
