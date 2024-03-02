use alloy_sol_macro::sol;

pub mod math {
    use super::*;
    sol!("contracts/ArbiterMath.sol");
    sol!(ArbiterMathInterface, "out/ArbiterMath.sol/ArbiterMath.json");

    sol!(ArbiterToken, "out/ArbiterToken.sol/ArbiterToken.json");

    fn testing_out() {
        let thing = ArbiterToken::constructorCall {};
    }
}
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod solstat_bindings;
