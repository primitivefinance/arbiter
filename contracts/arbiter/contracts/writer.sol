// SPDX-License-Identifier: MIT
// compiler version must be greater than or equal to 0.8.17 and less than 0.9.0
pragma solidity ^0.8.17;


/**
 * @dev Implementation of the test interface for Arbiter writing contracts.
 */
contract Writer {
    string private _test_string;

    event WasWritten(string test_string);

    function echoString(string memory test_string) public returns (string memory) {
        _test_string = test_string;
        emit WasWritten(test_string);
        return _test_string;
    }
    
}
