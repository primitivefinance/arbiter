pub use simple_registry::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod simple_registry {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"portfolio\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFee\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"portfolio\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFee\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static SIMPLEREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        93,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        34,
        96,
        36,
        130,
        1,
        82,
        127,
        69,
        116,
        104,
        101,
        114,
        32,
        115,
        101,
        110,
        116,
        32,
        116,
        111,
        32,
        110,
        111,
        110,
        45,
        112,
        97,
        121,
        97,
        98,
        108,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        97,
        55,
        183,
        96,
        241,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        80,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        51,
        23,
        144,
        85,
        97,
        4,
        47,
        128,
        97,
        0,
        127,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        93,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        34,
        96,
        36,
        130,
        1,
        82,
        127,
        69,
        116,
        104,
        101,
        114,
        32,
        115,
        101,
        110,
        116,
        32,
        116,
        111,
        32,
        110,
        111,
        110,
        45,
        112,
        97,
        121,
        97,
        98,
        108,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        97,
        55,
        183,
        96,
        241,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        142,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        166,
        203,
        224,
        151,
        20,
        97,
        0,
        243,
        87,
        128,
        99,
        229,
        81,
        86,
        181,
        20,
        97,
        1,
        8,
        87,
        128,
        99,
        247,
        124,
        71,
        145,
        20,
        97,
        1,
        27,
        87,
        91,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        53,
        96,
        36,
        130,
        1,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        97,
        1,
        6,
        97,
        1,
        1,
        54,
        96,
        4,
        97,
        3,
        141,
        86,
        91,
        97,
        1,
        74,
        86,
        91,
        0,
        91,
        97,
        1,
        6,
        97,
        1,
        22,
        54,
        96,
        4,
        97,
        3,
        204,
        86,
        91,
        97,
        2,
        58,
        86,
        91,
        96,
        0,
        84,
        97,
        1,
        46,
        144,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        128,
        84,
        48,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        130,
        22,
        23,
        144,
        145,
        85,
        96,
        64,
        81,
        99,
        221,
        164,
        7,
        151,
        96,
        224,
        27,
        129,
        82,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        132,
        129,
        22,
        96,
        4,
        131,
        1,
        82,
        96,
        36,
        130,
        1,
        132,
        144,
        82,
        145,
        130,
        22,
        145,
        133,
        22,
        144,
        99,
        221,
        164,
        7,
        151,
        144,
        96,
        68,
        1,
        96,
        0,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        128,
        59,
        21,
        128,
        21,
        97,
        1,
        252,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        37,
        96,
        36,
        130,
        1,
        82,
        127,
        84,
        97,
        114,
        103,
        101,
        116,
        32,
        99,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        99,
        111,
        110,
        116,
        97,
        105,
        110,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        100,
        32,
        99,
        111,
        100,
        101,
        96,
        216,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        80,
        90,
        241,
        21,
        128,
        21,
        97,
        2,
        16,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        148,
        144,
        148,
        22,
        147,
        144,
        147,
        23,
        144,
        146,
        85,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        84,
        48,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        130,
        22,
        23,
        144,
        145,
        85,
        96,
        64,
        81,
        99,
        120,
        125,
        206,
        61,
        96,
        224,
        27,
        129,
        82,
        96,
        4,
        129,
        1,
        131,
        144,
        82,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        145,
        130,
        22,
        145,
        132,
        22,
        144,
        99,
        120,
        125,
        206,
        61,
        144,
        96,
        36,
        1,
        96,
        0,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        128,
        59,
        21,
        128,
        21,
        97,
        2,
        228,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        37,
        96,
        36,
        130,
        1,
        82,
        127,
        84,
        97,
        114,
        103,
        101,
        116,
        32,
        99,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        99,
        111,
        110,
        116,
        97,
        105,
        110,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        100,
        32,
        99,
        111,
        100,
        101,
        96,
        216,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        80,
        90,
        241,
        21,
        128,
        21,
        97,
        2,
        248,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        148,
        144,
        148,
        22,
        147,
        144,
        147,
        23,
        144,
        146,
        85,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        34,
        96,
        36,
        130,
        1,
        82,
        127,
        65,
        66,
        73,
        32,
        100,
        101,
        99,
        111,
        100,
        105,
        110,
        103,
        58,
        32,
        116,
        117,
        112,
        108,
        101,
        32,
        100,
        97,
        116,
        97,
        32,
        116,
        111,
        111,
        32,
        115,
        104,
        111,
        96,
        68,
        130,
        1,
        82,
        97,
        28,
        157,
        96,
        242,
        27,
        96,
        100,
        130,
        1,
        82,
        96,
        132,
        129,
        253,
        91,
        128,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        3,
        136,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        96,
        96,
        132,
        134,
        3,
        18,
        21,
        97,
        3,
        165,
        87,
        97,
        3,
        165,
        97,
        3,
        33,
        86,
        91,
        97,
        3,
        174,
        132,
        97,
        3,
        113,
        86,
        91,
        146,
        80,
        97,
        3,
        188,
        96,
        32,
        133,
        1,
        97,
        3,
        113,
        86,
        91,
        145,
        80,
        96,
        64,
        132,
        1,
        53,
        144,
        80,
        146,
        80,
        146,
        80,
        146,
        86,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        3,
        226,
        87,
        97,
        3,
        226,
        97,
        3,
        33,
        86,
        91,
        97,
        3,
        235,
        131,
        97,
        3,
        113,
        86,
        91,
        148,
        96,
        32,
        147,
        144,
        147,
        1,
        53,
        147,
        80,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        38,
        244,
        228,
        122,
        95,
        151,
        214,
        129,
        78,
        13,
        53,
        221,
        222,
        90,
        58,
        83,
        165,
        177,
        141,
        204,
        251,
        176,
        249,
        75,
        186,
        237,
        67,
        243,
        226,
        185,
        192,
        31,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static SIMPLEREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        93,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        34,
        96,
        36,
        130,
        1,
        82,
        127,
        69,
        116,
        104,
        101,
        114,
        32,
        115,
        101,
        110,
        116,
        32,
        116,
        111,
        32,
        110,
        111,
        110,
        45,
        112,
        97,
        121,
        97,
        98,
        108,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        97,
        55,
        183,
        96,
        241,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        142,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        166,
        203,
        224,
        151,
        20,
        97,
        0,
        243,
        87,
        128,
        99,
        229,
        81,
        86,
        181,
        20,
        97,
        1,
        8,
        87,
        128,
        99,
        247,
        124,
        71,
        145,
        20,
        97,
        1,
        27,
        87,
        91,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        53,
        96,
        36,
        130,
        1,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        97,
        1,
        6,
        97,
        1,
        1,
        54,
        96,
        4,
        97,
        3,
        141,
        86,
        91,
        97,
        1,
        74,
        86,
        91,
        0,
        91,
        97,
        1,
        6,
        97,
        1,
        22,
        54,
        96,
        4,
        97,
        3,
        204,
        86,
        91,
        97,
        2,
        58,
        86,
        91,
        96,
        0,
        84,
        97,
        1,
        46,
        144,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        128,
        84,
        48,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        130,
        22,
        23,
        144,
        145,
        85,
        96,
        64,
        81,
        99,
        221,
        164,
        7,
        151,
        96,
        224,
        27,
        129,
        82,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        132,
        129,
        22,
        96,
        4,
        131,
        1,
        82,
        96,
        36,
        130,
        1,
        132,
        144,
        82,
        145,
        130,
        22,
        145,
        133,
        22,
        144,
        99,
        221,
        164,
        7,
        151,
        144,
        96,
        68,
        1,
        96,
        0,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        128,
        59,
        21,
        128,
        21,
        97,
        1,
        252,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        37,
        96,
        36,
        130,
        1,
        82,
        127,
        84,
        97,
        114,
        103,
        101,
        116,
        32,
        99,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        99,
        111,
        110,
        116,
        97,
        105,
        110,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        100,
        32,
        99,
        111,
        100,
        101,
        96,
        216,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        80,
        90,
        241,
        21,
        128,
        21,
        97,
        2,
        16,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        148,
        144,
        148,
        22,
        147,
        144,
        147,
        23,
        144,
        146,
        85,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        84,
        48,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        130,
        22,
        23,
        144,
        145,
        85,
        96,
        64,
        81,
        99,
        120,
        125,
        206,
        61,
        96,
        224,
        27,
        129,
        82,
        96,
        4,
        129,
        1,
        131,
        144,
        82,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        145,
        130,
        22,
        145,
        132,
        22,
        144,
        99,
        120,
        125,
        206,
        61,
        144,
        96,
        36,
        1,
        96,
        0,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        128,
        59,
        21,
        128,
        21,
        97,
        2,
        228,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        37,
        96,
        36,
        130,
        1,
        82,
        127,
        84,
        97,
        114,
        103,
        101,
        116,
        32,
        99,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        99,
        111,
        110,
        116,
        97,
        105,
        110,
        96,
        68,
        130,
        1,
        144,
        129,
        82,
        100,
        32,
        99,
        111,
        100,
        101,
        96,
        216,
        27,
        96,
        100,
        131,
        1,
        82,
        96,
        132,
        130,
        253,
        91,
        80,
        90,
        241,
        21,
        128,
        21,
        97,
        2,
        248,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        148,
        144,
        148,
        22,
        147,
        144,
        147,
        23,
        144,
        146,
        85,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        34,
        96,
        36,
        130,
        1,
        82,
        127,
        65,
        66,
        73,
        32,
        100,
        101,
        99,
        111,
        100,
        105,
        110,
        103,
        58,
        32,
        116,
        117,
        112,
        108,
        101,
        32,
        100,
        97,
        116,
        97,
        32,
        116,
        111,
        111,
        32,
        115,
        104,
        111,
        96,
        68,
        130,
        1,
        82,
        97,
        28,
        157,
        96,
        242,
        27,
        96,
        100,
        130,
        1,
        82,
        96,
        132,
        129,
        253,
        91,
        128,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        3,
        136,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        96,
        96,
        132,
        134,
        3,
        18,
        21,
        97,
        3,
        165,
        87,
        97,
        3,
        165,
        97,
        3,
        33,
        86,
        91,
        97,
        3,
        174,
        132,
        97,
        3,
        113,
        86,
        91,
        146,
        80,
        97,
        3,
        188,
        96,
        32,
        133,
        1,
        97,
        3,
        113,
        86,
        91,
        145,
        80,
        96,
        64,
        132,
        1,
        53,
        144,
        80,
        146,
        80,
        146,
        80,
        146,
        86,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        3,
        226,
        87,
        97,
        3,
        226,
        97,
        3,
        33,
        86,
        91,
        97,
        3,
        235,
        131,
        97,
        3,
        113,
        86,
        91,
        148,
        96,
        32,
        147,
        144,
        147,
        1,
        53,
        147,
        80,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        38,
        244,
        228,
        122,
        95,
        151,
        214,
        129,
        78,
        13,
        53,
        221,
        222,
        90,
        58,
        83,
        165,
        177,
        141,
        204,
        251,
        176,
        249,
        75,
        186,
        237,
        67,
        243,
        226,
        185,
        192,
        31,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static SIMPLEREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SimpleRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SimpleRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SimpleRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SimpleRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SimpleRegistry)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SIMPLEREGISTRY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SIMPLEREGISTRY_ABI.clone(),
                SIMPLEREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `claimFee` (0xa6cbe097) function
        pub fn claim_fee(
            &self,
            portfolio: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 203, 224, 151], (portfolio, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `controller` (0xf77c4791) function
        pub fn controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([247, 124, 71, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFee` (0xe55156b5) function
        pub fn set_fee(
            &self,
            portfolio: ::ethers::core::types::Address,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 81, 86, 181], (portfolio, fee))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SimpleRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `claimFee` function with signature `claimFee(address,address,uint256)` and selector `0xa6cbe097`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claimFee", abi = "claimFee(address,address,uint256)")]
    pub struct ClaimFeeCall {
        pub portfolio: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `controller` function with signature `controller()` and selector `0xf77c4791`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "controller", abi = "controller()")]
    pub struct ControllerCall;
    ///Container type for all input parameters for the `setFee` function with signature `setFee(address,uint256)` and selector `0xe55156b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFee", abi = "setFee(address,uint256)")]
    pub struct SetFeeCall {
        pub portfolio: ::ethers::core::types::Address,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimpleRegistryCalls {
        ClaimFee(ClaimFeeCall),
        Controller(ControllerCall),
        SetFee(SetFeeCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ClaimFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimFee(decoded));
            }
            if let Ok(decoded)
                = <ControllerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Controller(decoded));
            }
            if let Ok(decoded)
                = <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFee(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Controller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SimpleRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Controller(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimFeeCall> for SimpleRegistryCalls {
        fn from(value: ClaimFeeCall) -> Self {
            Self::ClaimFee(value)
        }
    }
    impl ::core::convert::From<ControllerCall> for SimpleRegistryCalls {
        fn from(value: ControllerCall) -> Self {
            Self::Controller(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for SimpleRegistryCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    ///Container type for all return fields from the `controller` function with signature `controller()` and selector `0xf77c4791`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ControllerReturn(pub ::ethers::core::types::Address);
}
