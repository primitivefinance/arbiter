pragma solidity ^0.8.0;

contract BlockInfo {
    function getBlockNumber() public view returns (uint256) {
        return block.number;
    }

    function getBlockTimestamp() public view returns (uint256) {
        return block.timestamp;
    }
}