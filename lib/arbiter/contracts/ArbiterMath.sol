pragma solidity ^0.8.17;
import "solmate/utils/FixedPointMathLib.sol";
import "portfolio/lib/solstat/src/Gaussian.sol";


contract ArbiterMath {
    using FixedPointMathLib for int256;
    using FixedPointMathLib for uint256;

    function cdf(int256 input) public pure returns (int256 output) {
        output = Gaussian.cdf(input);
    }

    function pdf(int256 input) public pure returns (int256 output) {
        output = Gaussian.pdf(input);
    }

    function ppf(int256 input) public pure returns (int256 output) {
        output = Gaussian.ppf(input);
    }

    function mulWadDown(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.mulWadDown(x, y);
    }

    function mulWadUp(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.mulWadUp(x, y);
    }

    function divWadDown(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.divWadDown(x, y);
    }

    function divWadUp(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.divWadUp(x, y);
    }
}