pub use arbiter_math::*;
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
pub mod arbiter_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("cdf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cdf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divWadDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divWadUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invariant"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("R_y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("R_x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tau"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("k"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulWadDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulWadUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pdf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pdf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ppf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ppf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sqrt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sqrt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Infinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OOB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OOB"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ARBITERMATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x12|\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xAE\x97h\xA8\x11a\0\xB3W\x80c\xAE\x97h\xA8\x14a\x01\xC1W\x80c\xBD%-\x06\x14a\x01\xD4W\x80c\xD0\xB7\x1B\x1E\x14a\x01\xE7W\x80c\xD2L\xE6\xE5\x14a\x01\xFAW\x80c\xE5$\xF8I\x14a\x02\rWa\0\xEBV[\x80c-[l\xB9\x14a\x01PW\x80c/Yw:\x14a\x01uW\x80c6yr:\x14a\x01\x88W\x80c7\xC6\xA4J\x14a\x01\x9BW\x80cgsB\xCE\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x10\xA4V[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x10\xC0V[a\x021V[a\x01ca\x01\x966`\x04a\x10\xA4V[a\x02JV[a\x01ca\x01\xA96`\x04a\x10\xFEV[a\x02UV[a\x01ca\x01\xBC6`\x04a\x10\xA4V[a\x02hV[a\x01ca\x01\xCF6`\x04a\x10\xFEV[a\x02sV[a\x01ca\x01\xE26`\x04a\x10\xFEV[a\x02\x7FV[a\x01ca\x01\xF56`\x04a\x10\xA4V[a\x02\x8BV[a\x01ca\x02\x086`\x04a\x10\xA4V[a\x02\xCEV[a\x01ca\x02\x1B6`\x04a\x10\xFEV[a\x02\xD9V[`\0a\x02+\x82a\x02\xE5V[\x92\x91PPV[`\0a\x02@\x86\x86\x86\x86\x86a\x04\xC5V[\x96\x95PPPPPPV[`\0a\x02+\x82a\x04\xE2V[`\0a\x02a\x83\x83a\x05\x88V[\x93\x92PPPV[`\0a\x02+\x82a\x05\x9DV[`\0a\x02a\x83\x83a\x06AV[`\0a\x02a\x83\x83a\x06VV[`\0a\x02\x96\x82a\x06kV[\x90Pa\x02\xC9`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x02\xA3C)\x03{\xAB\xA3\x83\xAB\xA1\x03K\x99\xD1`\x8D\x1B\x81RP\x82a\x06\xD4V[\x91\x90PV[`\0a\x02+\x82a\x07\x1DV[`\0a\x02a\x83\x83a\x07yV[`\0\x80\x82\x13a\x03'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x034\x84a\x07\x8EV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80a\x04\xD5\x86\x86\x86\x86\x86a\x086V[\x90\x96\x03\x96\x95PPPPPPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x04\xFBWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x05#W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x05DW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05Q\x83`\x02a\x119V[\x90P`\0a\x05^\x82a\tQV[\x90P`\0a\x05tg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0B\xCDV[\x90Pa\x05\x7F\x81a\x11iV[\x95\x94PPPPPV[`\0a\x02a\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0B\xE2V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x05\xB6W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x05\xD2W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x05\xEAW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x06\0W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x02a\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x01V[`\0a\x02a\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\x01V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x06\x89g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x119V[a\x06\x93\x91\x90a\x11\x85V[\x90P`\0a\x06\xA0\x82a\x11iV[\x90P`\0a\x06\xAD\x82a\x0C/V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x06\xCAg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x119V[a\x05\x7F\x91\x90a\x11\x85V[a\x07\x19\x82\x82`@Q`$\x01a\x06\xEA\x92\x91\x90a\x11\xC1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra\x0E\x13V[PPV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\x073\x81a\x11iV[a\x07=\x91\x90a\x119V[a\x07G\x91\x90a\x11\x85V[\x90Pa\x07R\x81a\x0E\x1FV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\x07og\r\xE0\xB6\xB3\xA7d\0\0\x83a\x119V[a\x02a\x91\x90a\x11\x85V[`\0a\x02a\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xE2V[`\0\x80\x82\x11a\x07\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\x1EV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x86\x11\x15a\x08aW`@Qc\xAA\xF3\x95o`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x86\x03a\x08wWP\x80a\x05\x7FV[\x85`\0\x03a\x08\x90Wa\x08\x89\x82\x86a\x12\x16V[\x90Pa\x05\x7FV[\x82\x15a\t+W`\0a\x08\xA6\x84c\x01\xE1\x85Xa\x05\x88V[\x90P`\0a\x08\xB3\x82a\x05\x9DV[\x90Pa\x08\xC3c;\x9A\xCA\0\x82a\x12>V[\x90Pa\x08\xCF\x86\x82a\x07yV[\x90P`\0a\x08\xE5\x89g\r\xE0\xB6\xB3\xA7d\0\0a\x12UV[\x90Pa\x08\xF0\x81a\x04\xE2V[\x90P`\0a\x08\xFE\x83\x83a\x12UV[\x90Pa\t\t\x81a\x06kV[\x90P\x85a\t\x16\x8A\x83a\x0B\xCDV[a\t \x91\x90a\x12\x16V[\x94PPPPPa\x05\x7FV[\x81a\tG\x86a\tB\x89g\r\xE0\xB6\xB3\xA7d\0\0a\x12UV[a\x0B\xCDV[a\x02@\x91\x90a\x12\x16V[`\0\x80\x82\x12\x80a\thWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\t\x86W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\t\xA7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\t\xCFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\t\xDAW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\n\x02Wa\t\xFD\x83g\x1B\xC1mgN\xC8\0\0a\x12UV[a\n\x04V[\x82[\x90P`\0a\n\x1A\x82g\x1B\xC1mgN\xC8\0\0a\x0F\xC8V[\x90P\x80`\0\x03a\n=W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\nH\x82a\x02\xE5V[\x90P`\0c;\x9A\xCA\0a\nsa\nna\nhg\x1B\xC1mgN\xC8\0\0a\x11iV[\x85a\x0B\xCDV[a\x05\x9DV[a\n}\x91\x90a\x119V[\x90P`\0\x80a\n\x94\x83g\x03\xC1f\\z\xAB \0a\x0B\xCDV[a\n\xA6\x90g \x05\xFEO&\x8E\xA0\0a\x12\x16V[\x90P`\0a\n\xD1\x84a\n\xBF\x86f\x9F2u$b\xA0\0a\x0B\xCDV[a\tB\x90g\r\xC5R\x7Fd, \0a\x12\x16V[a\n\xE3\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x16V[\x90Pa\x0B\x07g\t\xD0(\xCCo _\xFF\x19\x85a\n\xFD\x85\x85a\x0F\xC8V[a\tB\x91\x90a\x12UV[\x92PPP`\0[`\x02\x81\x10\x15a\x0B\xA2W`\0\x86a\x0B#\x84a\x0C/V[a\x0B-\x91\x90a\x12UV[\x90P`\0a\x0B;\x84\x85a\x0B\xCDV[a\x0BD\x90a\x11iV[\x90P`\0a\x0BQ\x82a\x0E\x1FV[\x90P`\0a\x0B_\x86\x85a\x0B\xCDV[a\x0Bqg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0B\xCDV[a\x0B{\x91\x90a\x12UV[\x90Pa\x0B\x87\x84\x82a\x0F\xC8V[a\x0B\x91\x90\x87a\x12\x16V[\x95P\x84`\x01\x01\x94PPPPPa\x0B\x0EV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0B\xBFWa\x0B\xBA\x82a\x11iV[a\x0B\xC1V[\x81[\x98\x97PPPPPPPPV[`\0a\x02a\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xD9V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0B\xFAW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\x19W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x0CHWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0C_WP`\0\x91\x90PV[a\x0CpgV\x98\xEE\xF0fp\0\0a\x11iV[\x82\x13a\x0C\x85WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x0C\x90\x83a\x0F\xF8V[\x90P`\0a\x0C\xC9g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xB2\x84g\x1B\xC1mgN\xC8\0\0a\x05\x88V[a\x0C\xC4\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x16V[a\x0F\xC8V[\x90P`\0\x80\x82a\r%\x81a\r\x12\x81a\r\0\x81a\x0C\xED\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0B\xCDV[a\tB\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x12\x16V[a\tB\x90g\x14\xA8EL\x19\xE1\xAC\0a\x12\x16V[a\tB\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x12\x16V[a\r7\x90g\x03\xDE\xBD\x08;\x8C|\0a\x12\x16V[\x91P\x83\x90Pa\r\x9F\x81a\r\x8D\x81a\r{\x81a\ri\x81a\rV\x81\x8Ba\x0B\xCDV[a\tB\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x12\x16V[a\tB\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x12\x16V[a\tB\x90g\x051\n\xA7\xD5!0\0a\x12\x16V[a\tB\x90g\r\xE0\xCC=\x15a\0\0a\x12\x16V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\r\xB5\x87\x88a\x0B\xCDV[a\r\xC1\x90`\0\x19a\x119V[a\r\xCB\x91\x90a\x12UV[a\r\xD5\x91\x90a\x12\x16V[\x92PP`\0a\r\xE3\x83a\x0E\x1FV[\x90P`\0a\r\xF1\x85\x83a\x0B\xCDV[\x90P`\0\x88\x12a\x0E\x01W\x80a\x0B\xC1V[a\x0B\xC1\x81g\x1B\xC1mgN\xC8\0\0a\x12UV[a\x0E\x1C\x81a\x103V[PV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0E:WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0E\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03\x1EV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x02a\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x0F\xF1W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x10\x1EW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x10/WP\x19`\x01\x01\x90V[P\x90V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xB9Wa\x10\xB9a\x10TV[P5\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x10\xDBWa\x10\xDBa\x10TV[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11\x14Wa\x11\x14a\x10TV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x11UWa\x11Ua\x11#V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02+Wa\x02+a\x11#V[`\0`\x01`\xFF\x1B\x82\x01a\x11~Wa\x11~a\x11#V[P`\0\x03\x90V[`\0\x82a\x11\xA2WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x11\xBCWa\x11\xBCa\x11#V[P\x05\x90V[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x11\xEFW` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x11\xD2V[P`\0``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x126Wa\x126a\x11#V[PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02+Wa\x02+a\x11#V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x12uWa\x12ua\x11#V[P\x92\x91PPV";
    /// The bytecode of the contract.
    pub static ARBITERMATH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xEBW`\x005`\xE0\x1C\x80c\xAE\x97h\xA8\x11a\0\xB3W\x80c\xAE\x97h\xA8\x14a\x01\xC1W\x80c\xBD%-\x06\x14a\x01\xD4W\x80c\xD0\xB7\x1B\x1E\x14a\x01\xE7W\x80c\xD2L\xE6\xE5\x14a\x01\xFAW\x80c\xE5$\xF8I\x14a\x02\rWa\0\xEBV[\x80c-[l\xB9\x14a\x01PW\x80c/Yw:\x14a\x01uW\x80c6yr:\x14a\x01\x88W\x80c7\xC6\xA4J\x14a\x01\x9BW\x80cgsB\xCE\x14a\x01\xAEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01ca\x01^6`\x04a\x10\xA4V[a\x02 V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01ca\x01\x836`\x04a\x10\xC0V[a\x021V[a\x01ca\x01\x966`\x04a\x10\xA4V[a\x02JV[a\x01ca\x01\xA96`\x04a\x10\xFEV[a\x02UV[a\x01ca\x01\xBC6`\x04a\x10\xA4V[a\x02hV[a\x01ca\x01\xCF6`\x04a\x10\xFEV[a\x02sV[a\x01ca\x01\xE26`\x04a\x10\xFEV[a\x02\x7FV[a\x01ca\x01\xF56`\x04a\x10\xA4V[a\x02\x8BV[a\x01ca\x02\x086`\x04a\x10\xA4V[a\x02\xCEV[a\x01ca\x02\x1B6`\x04a\x10\xFEV[a\x02\xD9V[`\0a\x02+\x82a\x02\xE5V[\x92\x91PPV[`\0a\x02@\x86\x86\x86\x86\x86a\x04\xC5V[\x96\x95PPPPPPV[`\0a\x02+\x82a\x04\xE2V[`\0a\x02a\x83\x83a\x05\x88V[\x93\x92PPPV[`\0a\x02+\x82a\x05\x9DV[`\0a\x02a\x83\x83a\x06AV[`\0a\x02a\x83\x83a\x06VV[`\0a\x02\x96\x82a\x06kV[\x90Pa\x02\xC9`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x02\xA3C)\x03{\xAB\xA3\x83\xAB\xA1\x03K\x99\xD1`\x8D\x1B\x81RP\x82a\x06\xD4V[\x91\x90PV[`\0a\x02+\x82a\x07\x1DV[`\0a\x02a\x83\x83a\x07yV[`\0\x80\x82\x13a\x03'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x034\x84a\x07\x8EV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80a\x04\xD5\x86\x86\x86\x86\x86a\x086V[\x90\x96\x03\x96\x95PPPPPPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x04\xFBWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x05#W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x05DW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05Q\x83`\x02a\x119V[\x90P`\0a\x05^\x82a\tQV[\x90P`\0a\x05tg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x0B\xCDV[\x90Pa\x05\x7F\x81a\x11iV[\x95\x94PPPPPV[`\0a\x02a\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0B\xE2V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x05\xB6W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x05\xD2W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x05\xEAW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x06\0W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x02a\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x01V[`\0a\x02a\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C\x01V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x06\x89g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x119V[a\x06\x93\x91\x90a\x11\x85V[\x90P`\0a\x06\xA0\x82a\x11iV[\x90P`\0a\x06\xAD\x82a\x0C/V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x06\xCAg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x119V[a\x05\x7F\x91\x90a\x11\x85V[a\x07\x19\x82\x82`@Q`$\x01a\x06\xEA\x92\x91\x90a\x11\xC1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x1ES\x13G`\xE1\x1B\x17\x90Ra\x0E\x13V[PPV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\x073\x81a\x11iV[a\x07=\x91\x90a\x119V[a\x07G\x91\x90a\x11\x85V[\x90Pa\x07R\x81a\x0E\x1FV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\x07og\r\xE0\xB6\xB3\xA7d\0\0\x83a\x119V[a\x02a\x91\x90a\x11\x85V[`\0a\x02a\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xE2V[`\0\x80\x82\x11a\x07\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\x1EV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x86\x11\x15a\x08aW`@Qc\xAA\xF3\x95o`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x86\x03a\x08wWP\x80a\x05\x7FV[\x85`\0\x03a\x08\x90Wa\x08\x89\x82\x86a\x12\x16V[\x90Pa\x05\x7FV[\x82\x15a\t+W`\0a\x08\xA6\x84c\x01\xE1\x85Xa\x05\x88V[\x90P`\0a\x08\xB3\x82a\x05\x9DV[\x90Pa\x08\xC3c;\x9A\xCA\0\x82a\x12>V[\x90Pa\x08\xCF\x86\x82a\x07yV[\x90P`\0a\x08\xE5\x89g\r\xE0\xB6\xB3\xA7d\0\0a\x12UV[\x90Pa\x08\xF0\x81a\x04\xE2V[\x90P`\0a\x08\xFE\x83\x83a\x12UV[\x90Pa\t\t\x81a\x06kV[\x90P\x85a\t\x16\x8A\x83a\x0B\xCDV[a\t \x91\x90a\x12\x16V[\x94PPPPPa\x05\x7FV[\x81a\tG\x86a\tB\x89g\r\xE0\xB6\xB3\xA7d\0\0a\x12UV[a\x0B\xCDV[a\x02@\x91\x90a\x12\x16V[`\0\x80\x82\x12\x80a\thWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\t\x86W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\t\xA7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\t\xCFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\t\xDAW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\n\x02Wa\t\xFD\x83g\x1B\xC1mgN\xC8\0\0a\x12UV[a\n\x04V[\x82[\x90P`\0a\n\x1A\x82g\x1B\xC1mgN\xC8\0\0a\x0F\xC8V[\x90P\x80`\0\x03a\n=W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\nH\x82a\x02\xE5V[\x90P`\0c;\x9A\xCA\0a\nsa\nna\nhg\x1B\xC1mgN\xC8\0\0a\x11iV[\x85a\x0B\xCDV[a\x05\x9DV[a\n}\x91\x90a\x119V[\x90P`\0\x80a\n\x94\x83g\x03\xC1f\\z\xAB \0a\x0B\xCDV[a\n\xA6\x90g \x05\xFEO&\x8E\xA0\0a\x12\x16V[\x90P`\0a\n\xD1\x84a\n\xBF\x86f\x9F2u$b\xA0\0a\x0B\xCDV[a\tB\x90g\r\xC5R\x7Fd, \0a\x12\x16V[a\n\xE3\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x16V[\x90Pa\x0B\x07g\t\xD0(\xCCo _\xFF\x19\x85a\n\xFD\x85\x85a\x0F\xC8V[a\tB\x91\x90a\x12UV[\x92PPP`\0[`\x02\x81\x10\x15a\x0B\xA2W`\0\x86a\x0B#\x84a\x0C/V[a\x0B-\x91\x90a\x12UV[\x90P`\0a\x0B;\x84\x85a\x0B\xCDV[a\x0BD\x90a\x11iV[\x90P`\0a\x0BQ\x82a\x0E\x1FV[\x90P`\0a\x0B_\x86\x85a\x0B\xCDV[a\x0Bqg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x0B\xCDV[a\x0B{\x91\x90a\x12UV[\x90Pa\x0B\x87\x84\x82a\x0F\xC8V[a\x0B\x91\x90\x87a\x12\x16V[\x95P\x84`\x01\x01\x94PPPPPa\x0B\x0EV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x0B\xBFWa\x0B\xBA\x82a\x11iV[a\x0B\xC1V[\x81[\x98\x97PPPPPPPPV[`\0a\x02a\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xD9V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0B\xFAW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\x19W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x0CHWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x0C_WP`\0\x91\x90PV[a\x0CpgV\x98\xEE\xF0fp\0\0a\x11iV[\x82\x13a\x0C\x85WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x0C\x90\x83a\x0F\xF8V[\x90P`\0a\x0C\xC9g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xB2\x84g\x1B\xC1mgN\xC8\0\0a\x05\x88V[a\x0C\xC4\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x16V[a\x0F\xC8V[\x90P`\0\x80\x82a\r%\x81a\r\x12\x81a\r\0\x81a\x0C\xED\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x0B\xCDV[a\tB\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x12\x16V[a\tB\x90g\x14\xA8EL\x19\xE1\xAC\0a\x12\x16V[a\tB\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x12\x16V[a\r7\x90g\x03\xDE\xBD\x08;\x8C|\0a\x12\x16V[\x91P\x83\x90Pa\r\x9F\x81a\r\x8D\x81a\r{\x81a\ri\x81a\rV\x81\x8Ba\x0B\xCDV[a\tB\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x12\x16V[a\tB\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x12\x16V[a\tB\x90g\x051\n\xA7\xD5!0\0a\x12\x16V[a\tB\x90g\r\xE0\xCC=\x15a\0\0a\x12\x16V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\r\xB5\x87\x88a\x0B\xCDV[a\r\xC1\x90`\0\x19a\x119V[a\r\xCB\x91\x90a\x12UV[a\r\xD5\x91\x90a\x12\x16V[\x92PP`\0a\r\xE3\x83a\x0E\x1FV[\x90P`\0a\r\xF1\x85\x83a\x0B\xCDV[\x90P`\0\x88\x12a\x0E\x01W\x80a\x0B\xC1V[a\x0B\xC1\x81g\x1B\xC1mgN\xC8\0\0a\x12UV[a\x0E\x1C\x81a\x103V[PV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0E:WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0E\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03\x1EV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x02a\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x0F\xF1W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x10\x1EW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x10/WP\x19`\x01\x01\x90V[P\x90V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xB9Wa\x10\xB9a\x10TV[P5\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x10\xDBWa\x10\xDBa\x10TV[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11\x14Wa\x11\x14a\x10TV[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x11UWa\x11Ua\x11#V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02+Wa\x02+a\x11#V[`\0`\x01`\xFF\x1B\x82\x01a\x11~Wa\x11~a\x11#V[P`\0\x03\x90V[`\0\x82a\x11\xA2WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x11\xBCWa\x11\xBCa\x11#V[P\x05\x90V[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x11\xEFW` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x11\xD2V[P`\0``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x126Wa\x126a\x11#V[PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02+Wa\x02+a\x11#V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x12uWa\x12ua\x11#V[P\x92\x91PPV";
    /// The deployed bytecode of the contract.
    pub static ARBITERMATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ArbiterMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ArbiterMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ArbiterMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ArbiterMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ArbiterMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ArbiterMath))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ArbiterMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ARBITERMATH_ABI.clone(),
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
                ARBITERMATH_ABI.clone(),
                ARBITERMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `cdf` (0xd0b71b1e) function
        pub fn cdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([208, 183, 27, 30], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divWadDown` (0x37c6a44a) function
        pub fn div_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 198, 164, 74], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divWadUp` (0xbd252d06) function
        pub fn div_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 37, 45, 6], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant` (0x2f59773a) function
        pub fn invariant(
            &self,
            r_y: ::ethers::core::types::U256,
            r_x: ::ethers::core::types::U256,
            stk: ::ethers::core::types::U256,
            vol: ::ethers::core::types::U256,
            tau: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([47, 89, 119, 58], (r_y, r_x, stk, vol, tau))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2d5b6cb9) function
        pub fn log(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([45, 91, 108, 185], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulWadDown` (0xe524f849) function
        pub fn mul_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 36, 248, 73], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulWadUp` (0xae9768a8) function
        pub fn mul_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([174, 151, 104, 168], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pdf` (0xd24ce6e5) function
        pub fn pdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([210, 76, 230, 229], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ppf` (0x3679723a) function
        pub fn ppf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([54, 121, 114, 58], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sqrt` (0x677342ce) function
        pub fn sqrt(
            &self,
            x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 115, 66, 206], x)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ArbiterMath<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Infinity` with signature `Infinity()` and selector `0x07a02127`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
    ///Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Min", abi = "Min()")]
    pub struct Min;
    ///Custom Error type `NegativeInfinity` with signature `NegativeInfinity()` and selector `0x8bb56614`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    ///Custom Error type `OOB` with signature `OOB()` and selector `0xaaf3956f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OOB", abi = "OOB()")]
    pub struct OOB;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ArbiterMathErrors {
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OOB(OOB),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ArbiterMathErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OOB as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OOB(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ArbiterMathErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OOB(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ArbiterMathErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <OOB as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ArbiterMathErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OOB(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ArbiterMathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Infinity> for ArbiterMathErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for ArbiterMathErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for ArbiterMathErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OOB> for ArbiterMathErrors {
        fn from(value: OOB) -> Self {
            Self::OOB(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for ArbiterMathErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    ///Container type for all input parameters for the `cdf` function with signature `cdf(int256)` and selector `0xd0b71b1e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cdf", abi = "cdf(int256)")]
    pub struct CdfCall {
        pub input: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `divWadDown` function with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "divWadDown", abi = "divWadDown(uint256,uint256)")]
    pub struct DivWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `divWadUp` function with signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "divWadUp", abi = "divWadUp(uint256,uint256)")]
    pub struct DivWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `invariant` function with signature `invariant(uint256,uint256,uint256,uint256,uint256)` and selector `0x2f59773a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "invariant",
        abi = "invariant(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct InvariantCall {
        pub r_y: ::ethers::core::types::U256,
        pub r_x: ::ethers::core::types::U256,
        pub stk: ::ethers::core::types::U256,
        pub vol: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(int256)` and selector `0x2d5b6cb9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(int256)")]
    pub struct LogCall {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `mulWadDown` function with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mulWadDown", abi = "mulWadDown(uint256,uint256)")]
    pub struct MulWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mulWadUp` function with signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mulWadUp", abi = "mulWadUp(uint256,uint256)")]
    pub struct MulWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pdf` function with signature `pdf(int256)` and selector `0xd24ce6e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pdf", abi = "pdf(int256)")]
    pub struct PdfCall {
        pub input: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `ppf` function with signature `ppf(int256)` and selector `0x3679723a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ppf", abi = "ppf(int256)")]
    pub struct PpfCall {
        pub input: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `sqrt` function with signature `sqrt(uint256)` and selector `0x677342ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sqrt", abi = "sqrt(uint256)")]
    pub struct SqrtCall {
        pub x: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ArbiterMathCalls {
        Cdf(CdfCall),
        DivWadDown(DivWadDownCall),
        DivWadUp(DivWadUpCall),
        Invariant(InvariantCall),
        Log(LogCall),
        MulWadDown(MulWadDownCall),
        MulWadUp(MulWadUpCall),
        Pdf(PdfCall),
        Ppf(PpfCall),
        Sqrt(SqrtCall),
    }
    impl ::ethers::core::abi::AbiDecode for ArbiterMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CdfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cdf(decoded));
            }
            if let Ok(decoded) = <DivWadDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivWadDown(decoded));
            }
            if let Ok(decoded) = <DivWadUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivWadUp(decoded));
            }
            if let Ok(decoded) = <InvariantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invariant(decoded));
            }
            if let Ok(decoded) = <LogCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log(decoded));
            }
            if let Ok(decoded) = <MulWadDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulWadDown(decoded));
            }
            if let Ok(decoded) = <MulWadUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulWadUp(decoded));
            }
            if let Ok(decoded) = <PdfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pdf(decoded));
            }
            if let Ok(decoded) = <PpfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ppf(decoded));
            }
            if let Ok(decoded) = <SqrtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sqrt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ArbiterMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Cdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivWadDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivWadUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Log(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulWadDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulWadUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ppf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sqrt(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ArbiterMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Cdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ppf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sqrt(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CdfCall> for ArbiterMathCalls {
        fn from(value: CdfCall) -> Self {
            Self::Cdf(value)
        }
    }
    impl ::core::convert::From<DivWadDownCall> for ArbiterMathCalls {
        fn from(value: DivWadDownCall) -> Self {
            Self::DivWadDown(value)
        }
    }
    impl ::core::convert::From<DivWadUpCall> for ArbiterMathCalls {
        fn from(value: DivWadUpCall) -> Self {
            Self::DivWadUp(value)
        }
    }
    impl ::core::convert::From<InvariantCall> for ArbiterMathCalls {
        fn from(value: InvariantCall) -> Self {
            Self::Invariant(value)
        }
    }
    impl ::core::convert::From<LogCall> for ArbiterMathCalls {
        fn from(value: LogCall) -> Self {
            Self::Log(value)
        }
    }
    impl ::core::convert::From<MulWadDownCall> for ArbiterMathCalls {
        fn from(value: MulWadDownCall) -> Self {
            Self::MulWadDown(value)
        }
    }
    impl ::core::convert::From<MulWadUpCall> for ArbiterMathCalls {
        fn from(value: MulWadUpCall) -> Self {
            Self::MulWadUp(value)
        }
    }
    impl ::core::convert::From<PdfCall> for ArbiterMathCalls {
        fn from(value: PdfCall) -> Self {
            Self::Pdf(value)
        }
    }
    impl ::core::convert::From<PpfCall> for ArbiterMathCalls {
        fn from(value: PpfCall) -> Self {
            Self::Ppf(value)
        }
    }
    impl ::core::convert::From<SqrtCall> for ArbiterMathCalls {
        fn from(value: SqrtCall) -> Self {
            Self::Sqrt(value)
        }
    }
    ///Container type for all return fields from the `cdf` function with signature `cdf(int256)` and selector `0xd0b71b1e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `divWadDown` function with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DivWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `divWadUp` function with signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DivWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `invariant` function with signature `invariant(uint256,uint256,uint256,uint256,uint256)` and selector `0x2f59773a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct InvariantReturn {
        pub k: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `log` function with signature `log(int256)` and selector `0x2d5b6cb9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LogReturn {
        pub z: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `mulWadDown` function with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MulWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mulWadUp` function with signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MulWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `pdf` function with signature `pdf(int256)` and selector `0xd24ce6e5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `ppf` function with signature `ppf(int256)` and selector `0x3679723a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PpfReturn {
        pub output: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `sqrt` function with signature `sqrt(uint256)` and selector `0x677342ce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SqrtReturn {
        pub z: ::ethers::core::types::U256,
    }
}
