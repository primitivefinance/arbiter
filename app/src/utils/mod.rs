use ethers::prelude::*;
use num_bigfloat::BigFloat;

pub fn convert(q64_96: U256) -> BigFloat {
    let least_sig = q64_96.0[0];
    let second_sig = q64_96.0[1];
    let third_sig = q64_96.0[2];
    let most_sig = q64_96.0[3];

    let bf2 = BigFloat::from(2);
    let bf64 = BigFloat::from(64);
    let bf128 = BigFloat::from(128);
    let bf192 = BigFloat::from(192);
    let bf96 = BigFloat::from(96);

    ((BigFloat::from(most_sig) * bf2.pow(&bf192))
        + (BigFloat::from(third_sig) * bf2.pow(&bf128))
        + (BigFloat::from(second_sig) * bf2.pow(&bf64))
        + BigFloat::from(least_sig))
        / bf2.pow(&bf96)
}
