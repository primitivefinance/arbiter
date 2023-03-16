// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.13;

/**
 * -------------
 *
 *   This is called the FVM, it's an alternative ABI.
 *   Originally, it was designed to compress calldata and therefore
 *   save gas on optimistic rollup networks.
 *
 *   There are levels to the optimizations that can be made for it,
 *   but this one focuses on the alternative multicall: jump process.
 *
 *   Multicalls will pad all calls to a full bytes32.
 *   This means two calls are at least 64 bytes.
 *   This alternative multicall can process over 10 calls in the same 64 bytes.
 *   The smallest bytes provided by a call is for allocate and deallocate, at 11 bytes.
 *
 *   Multicalls also process transactions sequentially.
 *   State cannot be carried over transiently between transactions.
 *   With FVM, we can transiently set state (only specific state),
 *   and use it across "instructions".
 *
 *   Without jump instruction, this alternative encoding is overkill.
 *
 *   Data is delivered via the `multiprocess` function in Portfolio.
 *
 *   -------------
 *
 *   Primitive™
 */

import "./AssemblyLib.sol";

uint8 constant JUMP_PROCESS_START_POINTER = 2;
bytes1 constant UNKNOWN = 0x00;
bytes1 constant ALLOCATE = 0x01;
bytes1 constant UNSET02 = 0x02;
bytes1 constant DEALLOCATE = 0x03;
bytes1 constant CLAIM = 0x04;
bytes1 constant SWAP_QUOTE = 0x05;
bytes1 constant SWAP_ASSET = 0x06;
bytes1 constant UNSET07 = 0x07;
bytes1 constant UNSET08 = 0x08;
bytes1 constant UNSET09 = 0x09;
bytes1 constant CREATE_POOL = 0x0B;
bytes1 constant CREATE_PAIR = 0x0C;
bytes1 constant UNSET0D = 0x0D;
bytes1 constant INSTRUCTION_JUMP = 0xAA;

error InvalidJump(uint256 pointer); // 0x80f63bd1
error InvalidBytesLength(uint256 expected, uint256 length); // 0xe19dc95e

/**
 * @dev Expects a serialized encoding of instructions.
 *      Serialized byte array -> [Jump Instruction Opcode,Total Amount of Instructions, Length of instruction[0], Data of instruction[0], Length of instruction[1],...]
 *
 * Motivation
 *      This serialization is intentional because it enables the use of a dynamic array for instructions.
 *      A fixed instruction array would pad unfilled array data with zeroes, wasting potentially a lot of bytes.
 *      On optimistic rollups, these bytes are the most expensive (in gas) bytes!
 *
 * Simple Guide
 *      First, information is added about the set of instructions that will be processed.
 *          - The jump instruction code, to signal we want to process multiple instructions.
 *          - The amount of instructions we want to process.
 *          - The length of the next instruction.
 *          - The instruction data.
 *          - The length of the next instruction.
 *          - Etc...
 *      Since we want to process multiple instructions that are in one big string,
 *      the encoding has to put information at the beginning of the instruction to say
 *      "this instruction is 22 bytes long".
 *      Then when it's decoded using the assumption "so the next instruction starts after 22 bytes".
 *
 * Glossary
 * | Term | Description | Size |
 * ---------------------------------
 * | Pointer | Index of the jump calldata that holds the length of an instruction. | 1 byte |
 * | Instruction Code | FVM "op code" to signal which operation to execute | 1 byte |
 * | Total Instructions | Amount of instructions to be executed | 1 byte |
 *
 * Conclusion
 *      To summarize, the calldata can be sliced to get the length of the instruction, e.g. `data[3:4]`.
 *      The `pointer` is initialized as this value. The pointer acts as an accumulator that moves across the bytes string.
 *      This accumulated value is the byte index of the last byte of the instruction.
 *
 * Example
 * | Byte Index                 | Data               |
 * ----------------------------------------------------------
 * | bytes[0]                   | 0xAA Instruction code     |
 * | bytes[1]                   | Amount of Instructions    |
 * | bytes[2]                   | ptr[0] := Length of instruction[0]
 * | bytes[2:ptr[0] + 1]        | Data of instruction[0]. Calldata slice does not include end index.   |
 * | bytes[ptr[0] + 1]          | ptr[1] := Length of instruction[1] |
 * | ...                        | Repeats in a loop for each instruction. |
 */
function _jumpProcess(bytes calldata data, function(bytes calldata) _process) {
    // Encoded `data`:| 0x | opcode | amount instructions | instruction length | instruction |
    uint8 totalInstructions = uint8(data[1]);
    // The "pointer" is pointing to the first byte of an instruction,
    // which holds the data for the instruction's length in bytes.
    uint256 idxPtr = JUMP_PROCESS_START_POINTER;
    // As the instructions are processed,
    // the pointer moves from the end to the start.
    uint256 idxInstructionStart;
    uint256 idxInstructionEnd;
    // For each instruction set...
    for (uint256 i; i != totalInstructions; ++i) {
        // Start the instruction where the pointer is.
        idxInstructionStart = idxPtr;
        // Compute the index of the next pointer by summing
        // the current pointer value, the length of the instruction,
        // and the amount of bytes the instruction length takes (which is 1 byte).
        idxInstructionEnd =
            idxInstructionStart + uint8(bytes1(data[idxInstructionStart])) + 1;
        // Make sure the pointer is not out of bounds.
        if (idxInstructionEnd > data.length) {
            revert InvalidJump(idxInstructionEnd);
        }
        // Calldata slicing EXCLUDES the `idxInstructionEnd` byte.
        bytes calldata instruction = data[idxInstructionStart:idxInstructionEnd];
        // Move the pointer to the EXCLUDED `idxInstructionEnd` byte.
        // This byte holds the data for the index of byte with the next instruction's length.
        idxPtr = idxInstructionEnd;
        // Process the instruction after removing the instruction length,
        // so only instruction data is passed to `_process`.
        _process(instruction[1:]);
    }
}

/**
 * @dev For debugging jump instructions, pass the jump instruction calldata to this function to get the individual instructions.
 */
function decodeJumpInstructions(bytes calldata data)
    pure
    returns (bytes[] memory)
{
    uint8 totalInstructions = uint8(data[1]);
    uint256 idxPtr = JUMP_PROCESS_START_POINTER;
    uint256 idxInstructionStart;
    uint256 idxInstructionEnd;
    bytes[] memory instructions = new bytes[](totalInstructions);
    for (uint256 i; i != totalInstructions; ++i) {
        idxInstructionStart = idxPtr;
        idxInstructionEnd =
            idxInstructionStart + uint8(bytes1(data[idxInstructionStart])) + 1;
        if (idxInstructionEnd > data.length) {
            revert InvalidJump(idxInstructionEnd);
        }
        bytes calldata instruction = data[idxInstructionStart:idxInstructionEnd];
        idxPtr = idxInstructionEnd;

        instructions[i] = instruction[1:];
    }

    return instructions;
}

/**
 * @dev Serializes an array of instructions by appending the length of the instruction to each instruction packet.
 * Adds the INSTRUCTION_JUMP opcode and total instructions quantity to the front of the `bytes` array.
 * @param instructions Dynamically sized array of FVM encoded instructions.
 */
function encodeJumpInstruction(bytes[] memory instructions)
    pure
    returns (bytes memory)
{
    uint8 totalInstructions = uint8(instructions.length);
    bytes memory payload =
        bytes.concat(INSTRUCTION_JUMP, bytes1(totalInstructions));

    // for each instruction set...
    for (uint256 i; i != totalInstructions; ++i) {
        bytes memory instruction = instructions[i];
        // Amount of bytes of data for this instruction.
        uint8 instructionLength = uint8(instruction.length);
        // Appends pointer to next instruction to the beginning of this instruction.
        bytes memory edited =
            bytes.concat(bytes1(instructionLength), instruction);
        // Concats the serialized bytes data with this edited instruction.
        payload = bytes.concat(payload, edited);
    }

    return payload;
}

/**
 * @dev Decodes the `pair id` from a `pool id`.
 * @param poolId Pool id to use for the decoding
 * @return pairId Corresponding pair id
 * @custom:example
 * ```
 * uint24 pairId = decodePairIdFromPoolId(46183783333895);
 * ```
 */
function decodePairIdFromPoolId(uint64 poolId) pure returns (uint24 pairId) {
    assembly {
        pairId := shr(40, poolId)
    }
}

/**
 * @dev Returns an encoded pool id given specific pool parameters.
 * @param pairId Id of the pair of asset / quote tokens
 * @param isMutable True if the pool is mutable
 * @param poolNonce Current pool nonce of the Portfolio contract
 * @return poolId Corresponding encoded pool id
 * @custom:example
 * ```
 * uint64 poolId = encodePoolId(7, true, 42);
 * ```
 */
function encodePoolId(
    uint24 pairId,
    bool isMutable,
    uint32 poolNonce
) pure returns (uint64 poolId) {
    assembly {
        poolId := shl(0, or(or(shl(40, pairId), shl(32, isMutable)), poolNonce))
    }
}

/**
 * @dev Decodes the parameters of a pool given its id.
 * The pool id is expected to be encoded using the following format:\
 * `0x | pairId (24 bits) | isMutable (8 bits) | poolNonce (32 bits)`
 * @param data Encoded pool id
 * @return poolId Pool id converted from bytes to uint64
 * @return pairId Pair id of the pool
 * @return isMutable True if the pool is mutable
 * @return poolNonce Pool nonce of the pool
 * @custom:example
 * ```
 * (uint64 poolId, uint24 pairId, uint8 isMutable, uint32 poolNonce)
 *     = decodePoolId(0x000007010000002a);
 * ```
 */
function decodePoolId(bytes calldata data)
    pure
    returns (uint64 poolId, uint24 pairId, uint8 isMutable, uint32 poolNonce)
{
    // Using Solidity here doesn't impact the gas cost.
    if (data.length != 8) revert InvalidBytesLength(8, data.length);

    assembly {
        // For some reason not using calldataload all the time helps reducing
        // the gas cost. I think it might be linked to going too deep into the
        // stack.
        let value := calldataload(data.offset)
        poolId := shr(192, calldataload(data.offset))
        pairId := shr(232, calldataload(data.offset))
        isMutable := shr(248, calldataload(add(3, data.offset)))
        poolNonce := shr(224, shl(32, value))
    }
}

function encodeCreatePair(
    address token0,
    address token1
) pure returns (bytes memory data) {
    data = abi.encodePacked(CREATE_PAIR, token0, token1);
}

/**
 * @dev Decodes the paramters of a `CREATE_PAIR` operation.
 * The data is expected to be encoded using the following format:\
 * `0x | CREATE_PAIR (8 bits) | tokenAsset (20 bytes) | tokenQuote (20 bytes)`
 * @param data Encoded `CREATE_PAIR` operation following the format above
 * @return tokenAsset Address of the asset token
 * @return tokenQuote Address of the quote token
 */
function decodeCreatePair(bytes calldata data)
    pure
    returns (address tokenAsset, address tokenQuote)
{
    if (data.length != 41) revert InvalidBytesLength(41, data.length);

    assembly {
        tokenAsset := shr(96, calldataload(add(1, data.offset)))
        tokenQuote := shr(96, calldataload(add(21, data.offset)))
    }
}

/**
 * @dev Encodes a claim operation.
 * FIXME: This function is not optimized! Using `encodePacked` is not ideal
 * because it preserves all the trailing zeros for each type. An improved version
 * should be made to reduce the calldata size by removing the extra zeros.
 */
function encodeClaim(
    uint64 poolId,
    uint128 fee0,
    uint128 fee1
) pure returns (bytes memory data) {
    (uint8 powerFee0, uint128 baseFee0) = AssemblyLib.fromAmount(fee0);
    (uint8 powerFee1, uint128 baseFee1) = AssemblyLib.fromAmount(fee1);

    return abi.encodePacked(
        CLAIM,
        poolId,
        uint8(27), // pointer to fee1
        powerFee0,
        baseFee0,
        powerFee1,
        baseFee1
    );
}

/**
 * @dev Decodes a claim operation
 */
function decodeClaim(bytes calldata data)
    pure
    returns (uint64 poolId, uint128 fee0, uint128 fee1)
{
    assembly {
        let value := calldataload(data.offset)
        poolId := shr(192, shl(8, value))
        let pointer := byte(9, value)
        let power := byte(10, value)
        let length := sub(pointer, 11)
        fee0 :=
            mul(
                shr(sub(256, mul(8, length)), calldataload(add(data.offset, 11))),
                exp(10, power)
            )

        power := byte(pointer, value)
        length := sub(data.length, add(1, pointer))
        fee1 :=
            mul(
                shr(
                    sub(256, mul(8, length)),
                    calldataload(add(data.offset, add(1, pointer)))
                ),
                exp(10, power)
            )
    }
}

/**
 * @dev Encodes a create pool operation.
 */
function encodeCreatePool(
    uint24 pairId,
    address controller,
    uint16 priorityFee,
    uint16 fee,
    uint16 vol,
    uint16 dur,
    uint16 jit,
    uint128 maxPrice,
    uint128 price
) pure returns (bytes memory data) {
    (uint8 power0, uint128 base0) = AssemblyLib.fromAmount(maxPrice);
    (uint8 power1, uint128 base1) = AssemblyLib.fromAmount(price);

    data = abi.encodePacked(
        CREATE_POOL,
        pairId,
        controller,
        priorityFee,
        fee,
        vol,
        dur,
        jit,
        uint8(36 + 16),
        power0,
        base0,
        power1,
        base1
    );
}

function decodeCreatePool(bytes calldata data)
    pure
    returns (
        uint24 pairId,
        address controller,
        uint16 priorityFee,
        uint16 fee,
        uint16 vol,
        uint16 dur,
        uint16 jit,
        uint128 maxPrice,
        uint128 price
    )
{
    assembly {
        pairId := shr(232, calldataload(add(1, data.offset)))
        controller := shr(96, calldataload(add(4, data.offset)))
        priorityFee := shr(240, calldataload(add(24, data.offset)))
        fee := shr(240, calldataload(add(26, data.offset)))
        vol := shr(240, calldataload(add(28, data.offset)))
        dur := shr(240, calldataload(add(30, data.offset)))
        jit := shr(240, calldataload(add(32, data.offset)))
        let pointer := byte(0, calldataload(add(34, data.offset)))
        let power0 := byte(0, calldataload(add(35, data.offset)))
        let length0 := sub(pointer, 36)
        let base0 :=
            shr(sub(256, mul(8, length0)), calldataload(add(data.offset, 36)))
        maxPrice := mul(base0, exp(10, power0))
        let power1 := byte(0, calldataload(add(pointer, data.offset)))
        let length1 := sub(data.length, add(1, pointer))
        let base1 :=
            shr(
                sub(256, mul(8, length1)),
                calldataload(add(data.offset, add(1, pointer)))
            )
        price := mul(base1, exp(10, power1))
    }
}

/**
 * @dev Encodes a allocate or deallocate operation.
 * FIXME: Same issue as `encodeClaim`... This function is not optimized!
 */
function encodeAllocateOrDeallocate(
    bool shouldAllocate,
    uint8 useMax,
    uint64 poolId,
    uint128 deltaLiquidity
) pure returns (bytes memory data) {
    (uint8 power, uint128 base) = AssemblyLib.fromAmount(deltaLiquidity);
    data = abi.encodePacked(
        AssemblyLib.pack(bytes1(useMax), shouldAllocate ? ALLOCATE : DEALLOCATE),
        poolId,
        power,
        base
    );
}

function decodeAllocateOrDeallocate(bytes calldata data)
    pure
    returns (uint8 useMax, uint64 poolId, uint128 deltaLiquidity)
{
    // Looks like using Solidity or Assembly is the same in terms of gas cost.
    if (data.length < 11) revert InvalidBytesLength(11, data.length);

    assembly {
        let value := calldataload(data.offset)
        useMax := shr(252, value)
        poolId := shr(192, shl(8, value))
        let power := shr(248, shl(72, value))
        let base := shr(sub(256, mul(8, sub(data.length, 10))), shl(80, value))
        deltaLiquidity := mul(base, exp(10, power))
    }
}

/**
 * @dev Encodes a swap operation
 * FIXME: Same issue as `encodeClaim`... This function is not optimized!
 */
function encodeSwap(
    uint8 useMax,
    uint64 poolId,
    uint128 amount0,
    uint128 amount1,
    uint8 sellAsset
) pure returns (bytes memory data) {
    (uint8 power0, uint128 base0) = AssemblyLib.fromAmount(amount0);
    (uint8 power1, uint128 base1) = AssemblyLib.fromAmount(amount1);

    data = abi.encodePacked(
        AssemblyLib.pack(
            bytes1(useMax), sellAsset == 1 ? SWAP_ASSET : SWAP_QUOTE
        ),
        poolId,
        uint8(27),
        power0,
        base0,
        power1,
        base1
    );
}

/**
 * @dev Decodes a swap operation.
 */
function decodeSwap(bytes calldata data)
    pure
    returns (
        uint8 useMax,
        uint64 poolId,
        uint128 input,
        uint128 output,
        uint8 sellAsset
    )
{
    assembly {
        let value := calldataload(data.offset)
        useMax := shr(4, byte(0, value))
        sellAsset := eq(6, and(0x0F, byte(0, value)))
        poolId := shr(192, calldataload(add(1, data.offset)))
        let pointer := byte(0, calldataload(add(9, data.offset)))
        let power0 := byte(0, calldataload(add(10, data.offset)))
        let length0 := sub(pointer, 11)
        let base0 :=
            shr(sub(256, mul(8, length0)), calldataload(add(data.offset, 11)))
        input := mul(base0, exp(10, power0))
        let power1 := byte(0, calldataload(add(pointer, data.offset)))
        let length1 := sub(data.length, add(1, pointer))
        let base1 :=
            shr(
                sub(256, mul(8, length1)),
                calldataload(add(data.offset, add(1, pointer)))
            )
        output := mul(base1, exp(10, power1))
    }
}
