// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.4;

import "portfolio/contracts/libraries/FVMLib.sol";

contract EncoderTarget {
    function createPair(address asset, address quote) external pure returns (bytes memory) {
        return encodeCreatePair(asset, quote);
    }

    function createPool(
        uint24 pairId,
        address controller,
        uint16 priorityFee,
        uint16 fee,
        uint16 vol,
        uint16 dur,
        uint16 jit,
        uint128 maxPrice,
        uint128 price
    ) external pure returns (bytes memory data) {
        return encodeCreatePool(
            pairId,
            controller,
            priorityFee,
            fee,
            vol,
            dur,
            jit,
            maxPrice,
            price
        );
    }

    function allocateOrDeallocate(
        bool shouldAllocate,
        uint8 useMax,
        uint64 poolId,
        uint128 deltaLiquidity,
        uint128 amount0,
        uint128 amount1
    ) external pure returns (bytes memory data) {
        return encodeAllocateOrDeallocate(
            shouldAllocate,
            useMax,
            poolId,
            deltaLiquidity,
            amount0,
            amount1
        );
    }

    function swap(
        uint8 useMax,
        uint64 poolId,
        uint128 amount0,
        uint128 amount1,
        uint8 sellAsset
    ) external pure returns (bytes memory data) {
        return encodeSwap(useMax, poolId, amount0, amount1, sellAsset);
    }
}
