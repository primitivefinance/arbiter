pub use example_sliding_window_oracle::*;
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
pub mod example_sliding_window_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("factory_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("windowSize_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("granularity_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("consult"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("consult"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("granularity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("granularity"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("observationIndexOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("observationIndexOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairObservations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pairObservations"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price0Cumulative"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price1Cumulative"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("periodSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("periodSize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("windowSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("windowSize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EXAMPLESLIDINGWINDOWORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15a\0WWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`@Qa\x11l8\x03\x80a\x11l\x839\x81\x81\x01`@R``\x81\x10\x15a\0\xACWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x91\x90`\x01`\xFF\x82\x16\x11a\x01\x15W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FSlidingWindowOracle: GRANULARITY`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\xFF\x82\x16\x80\x82\x81a\x01#W\xFE[\x04`\xE0\x81\x90R\x02\x14a\x01fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`0\x81R` \x01\x80a\x11<`0\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[``\x83\x90\x1B`\x01`\x01``\x1B\x03\x19\x16`\x80R`\xA0\x82\x90R`\xF8\x81\x90\x1B\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\xC0R`\xE0Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92`\xFF\x90\x91\x16\x90a\x0F%a\x02\x17`\09\x80a\x04\x10R\x80a\x06RR\x80a\x06\xA5R\x80a\x07\x05RP\x80a\x03'R\x80a\x05\xAFR\x80a\x06\xD1R\x80a\x07\xF9RP\x80a\x03KR\x80a\x03\xB1R\x80a\x044RP\x80a\x03uR\x80a\x05AR\x80a\x05jRPa\x0F%`\0\xF3\xFE`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c\xC4Z\x01U\x11a\0\xA1W\x80c\xC4Z\x01U\x14a\x02HW\x80c\xC6@u-\x14a\x02lW\x80c\xDB\xAA\xD3/\x14a\x02\xCEW\x80c\xE4F>\xB2\x14a\x03\x1DWa\0\xCEV[\x80cUo\r\xC7\x14a\x01,W\x80c\x8A\x14\x11z\x14a\x01JW\x80c\x8C\x86\xF1\xE4\x14a\x01dW\x80c\xBF\xCC\x8EB\x14a\x01\xCCW[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x014a\x03%V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01Ra\x03IV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01R`\x04\x806\x03``\x81\x10\x15a\x01\xACWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x90\x91\x015\x16a\x03mV[a\x02*`\x04\x806\x03`@\x81\x10\x15a\x02\x14WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x05\0V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x02Pa\x05?V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02\xCC`\x04\x806\x03`@\x81\x10\x15a\x02\xB4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x05cV[\0[a\x014`\x04\x806\x03` \x81\x10\x15a\x03\x16WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P5a\x06\xA0V[a\x01Ra\x07\x03V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x03\x9B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x85a\x07'V[\x90P`\0a\x03\xA8\x82a\x07\xE7V[\x80T\x90\x91PB\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\x04\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`3\x81R` \x01\x80a\x0El`3\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x81\x10\x15a\x04\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80a\x0E\x9F`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x04\x9E\x85a\x08uV[P\x91P\x91P`\0a\x04\xAF\x8A\x89a\x0B\xB5V[P\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x04\xE9Wa\x04\xDC\x85`\x01\x01T\x84\x86\x8Ca\x0C\x93V[\x96PPPPPPPa\x04\xF9V[a\x04\xDC\x85`\x02\x01T\x83\x86\x8Ca\x0C\x93V[\x93\x92PPPV[`\0` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x05\x19W\xFE[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x93P\x90\x91P\x83V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x05\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\x07'V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16\x81\x10\x15a\x06\x01W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T`\x01\x90\x81\x01\x82U\x91R\x01a\x05\xADV[P`\0a\x06\rBa\x06\xA0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x92\x93P\x90\x91`\xFF\x84\x16\x90\x81\x10a\x069W\xFE[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T\x90\x91PB\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\x06\x98W`\0\x80a\x06\x84\x86a\x08uV[PB\x86U`\x01\x86\x01\x91\x90\x91U`\x02\x85\x01UPP[PPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x81a\x06\xCCW\xFE[\x04\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16\x81\x81a\x06\xFBW\xFE[\x06\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0a\x076\x85\x85a\x0B\xB5V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x7F\xB0\xAE1u\xF4\xAE\xF6y\xD8\xE5\xEE\xC9\x1E~\x16\xDE:m\xE1XbwN$\x85\xDBG\x12 P\xF5`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80a\x07\xF3Ba\x06\xA0V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16\x82`\x01\x01`\xFF\x16\x81a\x08)W\xFE[\x06\x90P`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81`\xFF\x16\x81T\x81\x10a\x08^W\xFE[\x90`\0R` `\0 \x90`\x03\x02\x01\x92PPP\x91\x90PV[`\0\x80`\0a\x08\x82a\x0C\xF1V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16cY\t\xC0\xD5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x06WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\t\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\tbWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80QcZ=T\x93`\xE0\x1B\x81R\x90Q\x91\x94P`\x01`\x01`\xA0\x1B\x03\x86\x16\x91cZ=T\x93\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\t\xF1WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\n\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\nMWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qc\x02@\xBCk`\xE2\x1B\x81R\x90Q\x91\x93P`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\t\x02\xF1\xAC\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\n\xE2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\n\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x0B>WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x94P\x90\x92P\x90Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x85\x16\x14a\x0B\xABW\x80\x84\x03c\xFF\xFF\xFF\xFF\x81\x16a\x0Bx\x84\x86a\x0C\xFBV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x96\x90\x96\x01\x95c\xFF\xFF\xFF\xFF\x81\x16a\x0B\x99\x85\x85a\x0C\xFBV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x95\x90\x95\x01\x94P[PPP\x91\x93\x90\x92PV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0C\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x0E\xCB`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0C)W\x82\x84a\x0C,V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0C\x8CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0a\x0C\x9Da\x0EFV[`@Q\x80` \x01`@R\x80\x85\x88\x88\x03\x81a\x0C\xB3W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x16\x90R\x90Pa\x0C\xD3a\x0C\xCE\x82\x85a\r\xABV[a\x0E?V[q\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96\x95PPPPPPV[c\xFF\xFF\xFF\xFFB\x16\x90V[a\r\x03a\x0EFV[`\0\x82`\x01`\x01`p\x1B\x03\x16\x11a\raW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FFixedPoint: DIV_BY_ZERO_FRACTION`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x80`\x01`\x01`p\x1B\x03\x84\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`p\x1B`p\x87\x90\x1B\x16\x81a\r\x96W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x16\x81RP\x90P\x92\x91PPV[a\r\xB3a\x0EXV[`\0\x82\x15\x80a\r\xD9WPP\x82Q`\x01`\x01`\xE0\x1B\x03\x16\x82\x81\x02\x90\x83\x82\x81a\r\xD6W\xFE[\x04\x14[a\x0E*W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FFixedPoint: MUL_OVERFLOW\0\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x93\x92PPPV[Q`p\x1C\x90V[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[`@Q\x80` \x01`@R\x80`\0\x81RP\x90V\xFESlidingWindowOracle: MISSING_HISTORICAL_OBSERVATIONSlidingWindowOracle: UNEXPECTED_TIME_ELAPSEDUniswapV2Library: IDENTICAL_ADDRESSES\xA2dipfsX\"\x12 \xAC\xBCi' \x99\x97\xC0\x9B\\\x95\xB7\x14\x91J\x93\xE9\x87;\xFBf=\x1D7\xFAO\xD0\xD6\xAD\x8E\xA1\x0EdsolcC\0\x06\x06\x003SlidingWindowOracle: WINDOW_NOT_EVENLY_DIVISIBLE";
    /// The bytecode of the contract.
    pub static EXAMPLESLIDINGWINDOWORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c\xC4Z\x01U\x11a\0\xA1W\x80c\xC4Z\x01U\x14a\x02HW\x80c\xC6@u-\x14a\x02lW\x80c\xDB\xAA\xD3/\x14a\x02\xCEW\x80c\xE4F>\xB2\x14a\x03\x1DWa\0\xCEV[\x80cUo\r\xC7\x14a\x01,W\x80c\x8A\x14\x11z\x14a\x01JW\x80c\x8C\x86\xF1\xE4\x14a\x01dW\x80c\xBF\xCC\x8EB\x14a\x01\xCCW[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x014a\x03%V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01Ra\x03IV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01R`\x04\x806\x03``\x81\x10\x15a\x01\xACWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x90\x91\x015\x16a\x03mV[a\x02*`\x04\x806\x03`@\x81\x10\x15a\x02\x14WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x05\0V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x02Pa\x05?V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02\xCC`\x04\x806\x03`@\x81\x10\x15a\x02\xB4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x05cV[\0[a\x014`\x04\x806\x03` \x81\x10\x15a\x03\x16WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P5a\x06\xA0V[a\x01Ra\x07\x03V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x03\x9B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x85a\x07'V[\x90P`\0a\x03\xA8\x82a\x07\xE7V[\x80T\x90\x91PB\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\x04\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`3\x81R` \x01\x80a\x0El`3\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x81\x10\x15a\x04\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80a\x0E\x9F`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x04\x9E\x85a\x08uV[P\x91P\x91P`\0a\x04\xAF\x8A\x89a\x0B\xB5V[P\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x04\xE9Wa\x04\xDC\x85`\x01\x01T\x84\x86\x8Ca\x0C\x93V[\x96PPPPPPPa\x04\xF9V[a\x04\xDC\x85`\x02\x01T\x83\x86\x8Ca\x0C\x93V[\x93\x92PPPV[`\0` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x05\x19W\xFE[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x93P\x90\x91P\x83V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x05\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\x07'V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16\x81\x10\x15a\x06\x01W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T`\x01\x90\x81\x01\x82U\x91R\x01a\x05\xADV[P`\0a\x06\rBa\x06\xA0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x92\x93P\x90\x91`\xFF\x84\x16\x90\x81\x10a\x069W\xFE[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T\x90\x91PB\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\x06\x98W`\0\x80a\x06\x84\x86a\x08uV[PB\x86U`\x01\x86\x01\x91\x90\x91U`\x02\x85\x01UPP[PPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x81a\x06\xCCW\xFE[\x04\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16\x81\x81a\x06\xFBW\xFE[\x06\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0a\x076\x85\x85a\x0B\xB5V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x7F\xB0\xAE1u\xF4\xAE\xF6y\xD8\xE5\xEE\xC9\x1E~\x16\xDE:m\xE1XbwN$\x85\xDBG\x12 P\xF5`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80a\x07\xF3Ba\x06\xA0V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16\x82`\x01\x01`\xFF\x16\x81a\x08)W\xFE[\x06\x90P`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81`\xFF\x16\x81T\x81\x10a\x08^W\xFE[\x90`\0R` `\0 \x90`\x03\x02\x01\x92PPP\x91\x90PV[`\0\x80`\0a\x08\x82a\x0C\xF1V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16cY\t\xC0\xD5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x06WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\t\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\tbWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80QcZ=T\x93`\xE0\x1B\x81R\x90Q\x91\x94P`\x01`\x01`\xA0\x1B\x03\x86\x16\x91cZ=T\x93\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\t\xF1WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\n\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\nMWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qc\x02@\xBCk`\xE2\x1B\x81R\x90Q\x91\x93P`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\t\x02\xF1\xAC\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\n\xE2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\n\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x0B>WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x94P\x90\x92P\x90Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x85\x16\x14a\x0B\xABW\x80\x84\x03c\xFF\xFF\xFF\xFF\x81\x16a\x0Bx\x84\x86a\x0C\xFBV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x96\x90\x96\x01\x95c\xFF\xFF\xFF\xFF\x81\x16a\x0B\x99\x85\x85a\x0C\xFBV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x95\x90\x95\x01\x94P[PPP\x91\x93\x90\x92PV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0C\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x0E\xCB`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0C)W\x82\x84a\x0C,V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0C\x8CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0a\x0C\x9Da\x0EFV[`@Q\x80` \x01`@R\x80\x85\x88\x88\x03\x81a\x0C\xB3W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x16\x90R\x90Pa\x0C\xD3a\x0C\xCE\x82\x85a\r\xABV[a\x0E?V[q\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96\x95PPPPPPV[c\xFF\xFF\xFF\xFFB\x16\x90V[a\r\x03a\x0EFV[`\0\x82`\x01`\x01`p\x1B\x03\x16\x11a\raW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FFixedPoint: DIV_BY_ZERO_FRACTION`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x80`\x01`\x01`p\x1B\x03\x84\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`p\x1B`p\x87\x90\x1B\x16\x81a\r\x96W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x16\x81RP\x90P\x92\x91PPV[a\r\xB3a\x0EXV[`\0\x82\x15\x80a\r\xD9WPP\x82Q`\x01`\x01`\xE0\x1B\x03\x16\x82\x81\x02\x90\x83\x82\x81a\r\xD6W\xFE[\x04\x14[a\x0E*W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FFixedPoint: MUL_OVERFLOW\0\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x93\x92PPPV[Q`p\x1C\x90V[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[`@Q\x80` \x01`@R\x80`\0\x81RP\x90V\xFESlidingWindowOracle: MISSING_HISTORICAL_OBSERVATIONSlidingWindowOracle: UNEXPECTED_TIME_ELAPSEDUniswapV2Library: IDENTICAL_ADDRESSES\xA2dipfsX\"\x12 \xAC\xBCi' \x99\x97\xC0\x9B\\\x95\xB7\x14\x91J\x93\xE9\x87;\xFBf=\x1D7\xFAO\xD0\xD6\xAD\x8E\xA1\x0EdsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static EXAMPLESLIDINGWINDOWORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ExampleSlidingWindowOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExampleSlidingWindowOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExampleSlidingWindowOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExampleSlidingWindowOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExampleSlidingWindowOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExampleSlidingWindowOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExampleSlidingWindowOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXAMPLESLIDINGWINDOWORACLE_ABI.clone(),
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
                EXAMPLESLIDINGWINDOWORACLE_ABI.clone(),
                EXAMPLESLIDINGWINDOWORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `consult` (0x8c86f1e4) function
        pub fn consult(
            &self,
            token_in: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            token_out: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 134, 241, 228], (token_in, amount_in, token_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `granularity` (0x556f0dc7) function
        pub fn granularity(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([85, 111, 13, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `observationIndexOf` (0xdbaad32f) function
        pub fn observation_index_of(
            &self,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([219, 170, 211, 47], timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairObservations` (0xbfcc8e42) function
        pub fn pair_observations(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([191, 204, 142, 66], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `periodSize` (0xe4463eb2) function
        pub fn period_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([228, 70, 62, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0xc640752d) function
        pub fn update(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 64, 117, 45], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `windowSize` (0x8a14117a) function
        pub fn window_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([138, 20, 17, 122], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExampleSlidingWindowOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `consult` function with signature `consult(address,uint256,address)` and selector `0x8c86f1e4`
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
    #[ethcall(name = "consult", abi = "consult(address,uint256,address)")]
    pub struct ConsultCall {
        pub token_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub token_out: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `granularity` function with signature `granularity()` and selector `0x556f0dc7`
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
    #[ethcall(name = "granularity", abi = "granularity()")]
    pub struct GranularityCall;
    ///Container type for all input parameters for the `observationIndexOf` function with signature `observationIndexOf(uint256)` and selector `0xdbaad32f`
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
    #[ethcall(name = "observationIndexOf", abi = "observationIndexOf(uint256)")]
    pub struct ObservationIndexOfCall {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairObservations` function with signature `pairObservations(address,uint256)` and selector `0xbfcc8e42`
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
    #[ethcall(name = "pairObservations", abi = "pairObservations(address,uint256)")]
    pub struct PairObservationsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `periodSize` function with signature `periodSize()` and selector `0xe4463eb2`
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
    #[ethcall(name = "periodSize", abi = "periodSize()")]
    pub struct PeriodSizeCall;
    ///Container type for all input parameters for the `update` function with signature `update(address,address)` and selector `0xc640752d`
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
    #[ethcall(name = "update", abi = "update(address,address)")]
    pub struct UpdateCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `windowSize` function with signature `windowSize()` and selector `0x8a14117a`
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
    #[ethcall(name = "windowSize", abi = "windowSize()")]
    pub struct WindowSizeCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExampleSlidingWindowOracleCalls {
        Consult(ConsultCall),
        Factory(FactoryCall),
        Granularity(GranularityCall),
        ObservationIndexOf(ObservationIndexOfCall),
        PairObservations(PairObservationsCall),
        PeriodSize(PeriodSizeCall),
        Update(UpdateCall),
        WindowSize(WindowSizeCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleSlidingWindowOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ConsultCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Consult(decoded));
            }
            if let Ok(decoded)
                = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded)
                = <GranularityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Granularity(decoded));
            }
            if let Ok(decoded)
                = <ObservationIndexOfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ObservationIndexOf(decoded));
            }
            if let Ok(decoded)
                = <PairObservationsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PairObservations(decoded));
            }
            if let Ok(decoded)
                = <PeriodSizeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeriodSize(decoded));
            }
            if let Ok(decoded)
                = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded)
                = <WindowSizeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WindowSize(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleSlidingWindowOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Consult(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Granularity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ObservationIndexOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PairObservations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeriodSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WindowSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExampleSlidingWindowOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Consult(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Granularity(element) => ::core::fmt::Display::fmt(element, f),
                Self::ObservationIndexOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PairObservations(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeriodSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::WindowSize(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConsultCall> for ExampleSlidingWindowOracleCalls {
        fn from(value: ConsultCall) -> Self {
            Self::Consult(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for ExampleSlidingWindowOracleCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GranularityCall> for ExampleSlidingWindowOracleCalls {
        fn from(value: GranularityCall) -> Self {
            Self::Granularity(value)
        }
    }
    impl ::core::convert::From<ObservationIndexOfCall>
    for ExampleSlidingWindowOracleCalls {
        fn from(value: ObservationIndexOfCall) -> Self {
            Self::ObservationIndexOf(value)
        }
    }
    impl ::core::convert::From<PairObservationsCall>
    for ExampleSlidingWindowOracleCalls {
        fn from(value: PairObservationsCall) -> Self {
            Self::PairObservations(value)
        }
    }
    impl ::core::convert::From<PeriodSizeCall> for ExampleSlidingWindowOracleCalls {
        fn from(value: PeriodSizeCall) -> Self {
            Self::PeriodSize(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for ExampleSlidingWindowOracleCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<WindowSizeCall> for ExampleSlidingWindowOracleCalls {
        fn from(value: WindowSizeCall) -> Self {
            Self::WindowSize(value)
        }
    }
    ///Container type for all return fields from the `consult` function with signature `consult(address,uint256,address)` and selector `0x8c86f1e4`
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
    pub struct ConsultReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `granularity` function with signature `granularity()` and selector `0x556f0dc7`
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
    pub struct GranularityReturn(pub u8);
    ///Container type for all return fields from the `observationIndexOf` function with signature `observationIndexOf(uint256)` and selector `0xdbaad32f`
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
    pub struct ObservationIndexOfReturn {
        pub index: u8,
    }
    ///Container type for all return fields from the `pairObservations` function with signature `pairObservations(address,uint256)` and selector `0xbfcc8e42`
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
    pub struct PairObservationsReturn {
        pub timestamp: ::ethers::core::types::U256,
        pub price_0_cumulative: ::ethers::core::types::U256,
        pub price_1_cumulative: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `periodSize` function with signature `periodSize()` and selector `0xe4463eb2`
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
    pub struct PeriodSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `windowSize` function with signature `windowSize()` and selector `0x8a14117a`
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
    pub struct WindowSizeReturn(pub ::ethers::core::types::U256);
}
