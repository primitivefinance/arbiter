// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.13;

import "solstat/Invariant.sol";
import "../libraries/RMM01Lib.sol";

library RMM01ExtendedLib {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    struct RMM {
        uint256 strike;
        uint256 sigma;
        uint256 tau;
    }

    /**
     * @dev Computes change in price given a change in time in seconds.
     * @param stk WAD
     * @param vol percentage
     * @param prc WAD
     * @param tau seconds
     * @param epsilon seconds
     * @custom:math P(τ - ε) = ( ( P(τ) / K ) ^ ( √(1 - ε/τ) )) (K) (e^( (1/2) (o^2) ( √(τ) √(τ- ε) - (τ - ε) ) ))
     */
    function computePriceWithChangeInTau(
        uint256 stk,
        uint256 vol,
        uint256 prc,
        uint256 tau,
        uint256 epsilon
    ) internal pure returns (uint256) {
        if (epsilon == 0) return prc;
        if (epsilon > tau) return stk;

        RMM memory params = RMM(stk, vol, tau);

        uint256 tauYears;
        assembly {
            tauYears := sdiv(mul(tau, WAD), YEAR) // tau * WAD / year = time in years scaled to WAD
        }

        uint256 epsilonYears;
        assembly {
            epsilonYears := sdiv(mul(epsilon, WAD), YEAR) // epsilon * WAD / year = epsilon in years scaled to WAD
        }

        uint256 term_0 = WAD - (epsilonYears.divWadUp(tauYears)); // 1 - ε/τ, WAD - ((epsilon * WAD) / tau rounded down), units are WAD - WAD, time units cancel out
        uint256 term_1 = term_0.sqrt(); // √(1 - ε/τ)), this sqrts WAD, so we end up with SQRT_WAD units

        uint256 term_2 = prc.divWadUp(params.strike); // P(t) / K, both units are already WAD
        uint256 term_3 =
            uint256(int256(term_2).powWad(int256(term_1 * SQRT_WAD))); // ( P(τ) / K ) ^ ( √(1 - ε/τ) ))

        uint256 term_7;
        {
            uint256 currentTau = tauYears - epsilonYears; // (τ- ε), WAD - WAD = WAD
            uint256 tausSqrt = tauYears.sqrt() * (currentTau).sqrt(); // ( √(τ) √(τ- ε) ), sqrt(1e18) = 1e9, so 1e9 * 1e9 = 1e18
            uint256 term_4 = tausSqrt - currentTau; // ( √(τ) √(τ- ε) - (τ - ε) ), WAD - WAD = WAD

            uint256 sigmaWad =
                RMM01Lib.convertPercentageToWad(uint256(params.sigma));

            uint256 term_5 = (sigmaWad * sigmaWad) / Gaussian.DOUBLE_WAD; // ( 1 / 2 )(o^2), 1e4 * 1e4 * 1e17 / 1e4 = 1e17, which is half WAD
            uint256 term_6 =
                uint256((int256(term_5.mulWadDown(term_4))).expWad()); // (e^( (1/2) (o^2) ( √(τ) √(τ- ε) - (τ - ε) ) )), exp(WAD * WAD / WAD)
            term_7 = uint256(params.strike).mulWadDown(term_6); // (K) (e^( (1/2) (o^2) ( √(τ) √(τ- ε) - (τ - ε) ) ), WAD * WAD / WAD
        }

        uint256 price = term_3.mulWadDown(term_7); // WAD * WAD / WAD = WAD
        return price;
    }

    /**
     * @custom:math
     * y(τ - ε) = K phi( 1 / (o * sqrt(t)) * ln(P(t) / K) + 1/2*o^2t - o*sqrt(t-e))
     */
    function computeYWithChangeInTau(
        uint256 stk,
        uint256 vol,
        uint256 prc,
        uint256 tau,
        uint256 epsilon
    ) internal pure returns (uint256 R_y) {
        RMM memory params = RMM(stk, vol, tau);

        uint256 tauYears;
        assembly {
            tauYears := sdiv(mul(tau, WAD), YEAR) // tau * WAD / year = time in years scaled to WAD
        }

        uint256 epsilonYears;
        assembly {
            epsilonYears := sdiv(mul(epsilon, WAD), YEAR) // epsilon * WAD / year = epsilon in years scaled to WAD
        }

        uint256 sigmaWad =
            RMM01Lib.convertPercentageToWad(uint256(params.sigma));
        uint256 part0 =
            WAD.divWadDown(sigmaWad.mulWadDown(tauYears.sqrt() * 1e9));
        part0 = part0.mulWadDown(
            uint256(int256(prc.divWadDown(params.strike)).lnWad())
        );

        uint256 part1 = (sigmaWad * sigmaWad) / Gaussian.DOUBLE_WAD;
        part1 = part1.mulWadDown(tauYears);

        uint256 part2 =
            sigmaWad.mulWadDown((tauYears - epsilonYears).sqrt() * 1e9);

        R_y = params.strike.mulWadDown(
            uint256(Gaussian.cdf(int256(part0 + part1 - part2)))
        );
    }

    /**
     * @dev Computes the next price, invariant, and y reserves of a curve after a change in time.
     */
    function computeCurveChanges(
        uint256 stk,
        uint256 vol,
        uint256 tau,
        uint256 prc,
        int256 invariant,
        uint256 epsilon
    ) internal pure returns (uint256 p_t, int128 i_t, uint256 t_e) {
        RMM memory curve = RMM(stk, vol, tau);
        p_t = computePriceWithChangeInTau(stk, vol, prc, tau, epsilon);

        uint256 x_1 = RMM01Lib.getXWithPrice(prc, stk, vol, tau);
        curve.tau -= epsilon;
        uint256 y_2 = Invariant.getY({
            R_x: x_1,
            stk: stk,
            vol: vol,
            tau: tau,
            inv: invariant
        });

        i_t = int128(
            Invariant.invariant({
                R_y: y_2,
                R_x: x_1,
                stk: stk,
                vol: RMM01Lib.convertPercentageToWad(vol),
                tau: tau
            })
        );
        t_e = tau - epsilon;
    }
}
