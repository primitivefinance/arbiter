// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity >=0.8.0;

interface IPortfolioRegistry {
    /**
     * @dev Controller of the PortfolioRegistry contract.
     */
    function controller() external view returns (address);
}
