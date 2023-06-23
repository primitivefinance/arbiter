pub use router_event_emitter::*;
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
pub mod router_event_emitter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("swapETHForExactTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapETHForExactTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactETHForTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactETHForTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForETH",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("swapTokensForExactETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapTokensForExactETH",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("swapTokensForExactTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapTokensForExactTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Amounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Amounts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ROUTEREVENTEMITTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x15\xA7\x80a\0f`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0YW`\x005`\xE0\x1C\x80c\"\xB5\x84\x10\x14a\0\xB2W\x80c7W4\xD9\x14a\x02vW\x80c\x9C\x91\xFC\xB5\x14a\x048W\x80c\xB0_W\x9E\x14a\x05\xFAW\x80c\xFA2\x19\xD5\x14a\x07tW\x80c\xFD\xE1\xAD\xDA\x14a\t6Wa\0`V[6a\0`W\0[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FUnknown signature and no fallbac`D\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\0\xF2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x01;WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x01\xADWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x01\xFCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x02ZWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\n\xB0V[\0[4\x80\x15a\x02\xB6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x02\xFFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x03qWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x03\xC0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\x1EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0E\tV[4\x80\x15a\x04xWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x04\xC1WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x053WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x05\x82WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x05\xE0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0E\xF2V[a\x02t`\x04\x806\x03`\xA0\x81\x10\x15a\x06BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x06\xADWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06\xFCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x07ZWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0F\xDBV[4\x80\x15a\x07\xB4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x07\xFDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x08oWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x08\xBEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\t\x1CWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x13+V[a\x02t`\x04\x806\x03`\xA0\x81\x10\x15a\t~WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\t\xE9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\n8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\n\x96WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x14\x14V[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16c\x88\x03\xDB\xEE`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x0B\xFAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xFFV[``\x91P[P\x91P\x91P\x81a\x0C\x0BW\xFE[\x7FL\xC1y\x91\xE3a\x0E4\0\xE7J\x83\x06\xAAB\x1D\xAA\xEE\x1E4F\xFA\"\xB4\x90`F\x10\x99\x9E\x10\xB3\x81\x80` \x01\x90Q` \x81\x10\x15a\x0CsWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84`\x01` \x1B\x82\x11\x15a\x0C\xCDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rh0\x9087\xB4\xB7:2\xB9`\xB9\x1B`dR\x90`\x84\x90\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\r\x1BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`'`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rf\x18H\x1C\xDD\x18\\\x9D`\xCA\x1B`dR\x90`\x84\x90\xFD[\x82Q\x86` \x82\x02\x83\x01\x11`\x01` \x1B\x82\x11\x17\x15a\rqWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`(`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rg\x0C$\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`dR\x90`\x84\x90\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\r\x9EW\x81\x81\x01Q\x83\x82\x01R` \x01a\r\x86V[PPPP\x90P\x01`@RPPP`@Q\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\r\xEBW\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xD3V[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16c8\xED\x179`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16c\x18\xCB\xAF\xE5`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\0``\x87`\x01`\x01`\xA0\x1B\x03\x16c\xFB;\xDBA`\xE0\x1B\x88\x88\x88\x88\x88`@Q`$\x01\x80\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x96PPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x10\xBDW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x10\x9EV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x11\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\"V[``\x91P[P\x91P\x91P\x81a\x11.W\xFE[\x7FL\xC1y\x91\xE3a\x0E4\0\xE7J\x83\x06\xAAB\x1D\xAA\xEE\x1E4F\xFA\"\xB4\x90`F\x10\x99\x9E\x10\xB3\x81\x80` \x01\x90Q` \x81\x10\x15a\x11\x96WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84`\x01` \x1B\x82\x11\x15a\x11\xF0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rh0\x9087\xB4\xB7:2\xB9`\xB9\x1B`dR\x90`\x84\x90\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x12>WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`'`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rf\x18H\x1C\xDD\x18\\\x9D`\xCA\x1B`dR\x90`\x84\x90\xFD[\x82Q\x86` \x82\x02\x83\x01\x11`\x01` \x1B\x82\x11\x17\x15a\x12\x94WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`(`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rg\x0C$\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`dR\x90`\x84\x90\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x12\xC1W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xA9V[PPPP\x90P\x01`@RPPP`@Q\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x13\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xF6V[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16cJ%\xD9J`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\0``\x87`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xF3j\xB5`\xE0\x1B\x88\x88\x88\x88\x88`@Q`$\x01\x80\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x83\x82\x01R`@\x80Q`\x1F\x90\x92\x01`\x1F\x19\x90\x81\x16\x90\x94\x01\x82\x81\x03\x90\x94\x01\x82R\x92\x83R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x9D\x16\x9C\x90\x9C\x17\x8CR\x92Q\x81Q\x91\x9B\x90\x9AP\x8A\x99P\x97P\x95P\x85\x94P\x87\x93P\x86\x92PP\x83\x10a\x10\xBDW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x10\x9EV\xFEABI calldata decoding: invalid dABI calldata decoding: invalid hABI memory decoding: invalid datEther sent to non-payable functi\xA2dipfsX\"\x12 \xC1\x96\xBA\xF3\xB7\xF1\xEEk\x8Ba\xC9_\x9E\x88i\xDA\xCC\xEA\xF32\xD2\x8F\x11\xC0B\xEAU\nL;M\x93dsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static ROUTEREVENTEMITTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0YW`\x005`\xE0\x1C\x80c\"\xB5\x84\x10\x14a\0\xB2W\x80c7W4\xD9\x14a\x02vW\x80c\x9C\x91\xFC\xB5\x14a\x048W\x80c\xB0_W\x9E\x14a\x05\xFAW\x80c\xFA2\x19\xD5\x14a\x07tW\x80c\xFD\xE1\xAD\xDA\x14a\t6Wa\0`V[6a\0`W\0[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FUnknown signature and no fallbac`D\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\0\xF2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x01;WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x01\xADWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x01\xFCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x02ZWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\n\xB0V[\0[4\x80\x15a\x02\xB6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x02\xFFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x03qWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x03\xC0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\x1EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0E\tV[4\x80\x15a\x04xWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x04\xC1WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x053WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x05\x82WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x05\xE0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0E\xF2V[a\x02t`\x04\x806\x03`\xA0\x81\x10\x15a\x06BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x06\xADWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06\xFCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x07ZWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0F\xDBV[4\x80\x15a\x07\xB4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\x15R\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02t`\x04\x806\x03`\xC0\x81\x10\x15a\x07\xFDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x08oWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x08\xBEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\t\x1CWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x13+V[a\x02t`\x04\x806\x03`\xA0\x81\x10\x15a\t~WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\t\xE9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\n8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x15\x12\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\n\x96WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` a\x14\xF2\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x14\x14V[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16c\x88\x03\xDB\xEE`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x0B\xFAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xFFV[``\x91P[P\x91P\x91P\x81a\x0C\x0BW\xFE[\x7FL\xC1y\x91\xE3a\x0E4\0\xE7J\x83\x06\xAAB\x1D\xAA\xEE\x1E4F\xFA\"\xB4\x90`F\x10\x99\x9E\x10\xB3\x81\x80` \x01\x90Q` \x81\x10\x15a\x0CsWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84`\x01` \x1B\x82\x11\x15a\x0C\xCDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rh0\x9087\xB4\xB7:2\xB9`\xB9\x1B`dR\x90`\x84\x90\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\r\x1BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`'`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rf\x18H\x1C\xDD\x18\\\x9D`\xCA\x1B`dR\x90`\x84\x90\xFD[\x82Q\x86` \x82\x02\x83\x01\x11`\x01` \x1B\x82\x11\x17\x15a\rqWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`(`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rg\x0C$\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`dR\x90`\x84\x90\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\r\x9EW\x81\x81\x01Q\x83\x82\x01R` \x01a\r\x86V[PPPP\x90P\x01`@RPPP`@Q\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\r\xEBW\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xD3V[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16c8\xED\x179`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16c\x18\xCB\xAF\xE5`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\0``\x87`\x01`\x01`\xA0\x1B\x03\x16c\xFB;\xDBA`\xE0\x1B\x88\x88\x88\x88\x88`@Q`$\x01\x80\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x96PPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x10\xBDW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x10\x9EV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x11\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\"V[``\x91P[P\x91P\x91P\x81a\x11.W\xFE[\x7FL\xC1y\x91\xE3a\x0E4\0\xE7J\x83\x06\xAAB\x1D\xAA\xEE\x1E4F\xFA\"\xB4\x90`F\x10\x99\x9E\x10\xB3\x81\x80` \x01\x90Q` \x81\x10\x15a\x11\x96WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84`\x01` \x1B\x82\x11\x15a\x11\xF0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rh0\x9087\xB4\xB7:2\xB9`\xB9\x1B`dR\x90`\x84\x90\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x12>WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`'`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rf\x18H\x1C\xDD\x18\\\x9D`\xCA\x1B`dR\x90`\x84\x90\xFD[\x82Q\x86` \x82\x02\x83\x01\x11`\x01` \x1B\x82\x11\x17\x15a\x12\x94WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`(`$R`\0\x80Q` a\x152\x839\x81Q\x91R`D\x90\x81Rg\x0C$\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`dR\x90`\x84\x90\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x12\xC1W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xA9V[PPPP\x90P\x01`@RPPP`@Q\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x13\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xF6V[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[`\0``\x88`\x01`\x01`\xA0\x1B\x03\x16cJ%\xD9J`\xE0\x1B\x89\x89\x89\x89\x89\x89`@Q`$\x01\x80\x87\x81R` \x01\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x97PPPPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83` \x83\x10a\x0B\x9AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B{V[`\0``\x87`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xF3j\xB5`\xE0\x1B\x88\x88\x88\x88\x88`@Q`$\x01\x80\x86\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x86\x86\x82\x81\x81R` \x01\x92P` \x02\x80\x82\x847`\0\x83\x82\x01R`@\x80Q`\x1F\x90\x92\x01`\x1F\x19\x90\x81\x16\x90\x94\x01\x82\x81\x03\x90\x94\x01\x82R\x92\x83R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x9D\x16\x9C\x90\x9C\x17\x8CR\x92Q\x81Q\x91\x9B\x90\x9AP\x8A\x99P\x97P\x95P\x85\x94P\x87\x93P\x86\x92PP\x83\x10a\x10\xBDW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x10\x9EV\xFEABI calldata decoding: invalid dABI calldata decoding: invalid hABI memory decoding: invalid datEther sent to non-payable functi\xA2dipfsX\"\x12 \xC1\x96\xBA\xF3\xB7\xF1\xEEk\x8Ba\xC9_\x9E\x88i\xDA\xCC\xEA\xF32\xD2\x8F\x11\xC0B\xEAU\nL;M\x93dsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static ROUTEREVENTEMITTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RouterEventEmitter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RouterEventEmitter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RouterEventEmitter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RouterEventEmitter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RouterEventEmitter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RouterEventEmitter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RouterEventEmitter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROUTEREVENTEMITTER_ABI.clone(),
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
                ROUTEREVENTEMITTER_ABI.clone(),
                ROUTEREVENTEMITTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `swapETHForExactTokens` (0xb05f579e) function
        pub fn swap_eth_for_exact_tokens(
            &self,
            router: ::ethers::core::types::Address,
            amount_out: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [176, 95, 87, 158],
                    (router, amount_out, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactETHForTokens` (0xfde1adda) function
        pub fn swap_exact_eth_for_tokens(
            &self,
            router: ::ethers::core::types::Address,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [253, 225, 173, 218],
                    (router, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForETH` (0x9c91fcb5) function
        pub fn swap_exact_tokens_for_eth(
            &self,
            router: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [156, 145, 252, 181],
                    (router, amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokens` (0x375734d9) function
        pub fn swap_exact_tokens_for_tokens(
            &self,
            router: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [55, 87, 52, 217],
                    (router, amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactETH` (0xfa3219d5) function
        pub fn swap_tokens_for_exact_eth(
            &self,
            router: ::ethers::core::types::Address,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [250, 50, 25, 213],
                    (router, amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactTokens` (0x22b58410) function
        pub fn swap_tokens_for_exact_tokens(
            &self,
            router: ::ethers::core::types::Address,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 181, 132, 16],
                    (router, amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Amounts` event
        pub fn amounts_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AmountsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AmountsFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RouterEventEmitter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Amounts", abi = "Amounts(uint256[])")]
    pub struct AmountsFilter {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `swapETHForExactTokens` function with signature `swapETHForExactTokens(address,uint256,address[],address,uint256)` and selector `0xb05f579e`
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
    #[ethcall(
        name = "swapETHForExactTokens",
        abi = "swapETHForExactTokens(address,uint256,address[],address,uint256)"
    )]
    pub struct SwapETHForExactTokensCall {
        pub router: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactETHForTokens` function with signature `swapExactETHForTokens(address,uint256,address[],address,uint256)` and selector `0xfde1adda`
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
    #[ethcall(
        name = "swapExactETHForTokens",
        abi = "swapExactETHForTokens(address,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensCall {
        pub router: ::ethers::core::types::Address,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForETH` function with signature `swapExactTokensForETH(address,uint256,uint256,address[],address,uint256)` and selector `0x9c91fcb5`
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
    #[ethcall(
        name = "swapExactTokensForETH",
        abi = "swapExactTokensForETH(address,uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHCall {
        pub router: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(address,uint256,uint256,address[],address,uint256)` and selector `0x375734d9`
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
    #[ethcall(
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(address,uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub router: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapTokensForExactETH` function with signature `swapTokensForExactETH(address,uint256,uint256,address[],address,uint256)` and selector `0xfa3219d5`
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
    #[ethcall(
        name = "swapTokensForExactETH",
        abi = "swapTokensForExactETH(address,uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactETHCall {
        pub router: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(address,uint256,uint256,address[],address,uint256)` and selector `0x22b58410`
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
    #[ethcall(
        name = "swapTokensForExactTokens",
        abi = "swapTokensForExactTokens(address,uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactTokensCall {
        pub router: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RouterEventEmitterCalls {
        SwapETHForExactTokens(SwapETHForExactTokensCall),
        SwapExactETHForTokens(SwapExactETHForTokensCall),
        SwapExactTokensForETH(SwapExactTokensForETHCall),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapTokensForExactETH(SwapTokensForExactETHCall),
        SwapTokensForExactTokens(SwapTokensForExactTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for RouterEventEmitterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <SwapETHForExactTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapETHForExactTokens(decoded));
            }
            if let Ok(decoded)
                = <SwapExactETHForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapExactETHForTokens(decoded));
            }
            if let Ok(decoded)
                = <SwapExactTokensForETHCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapExactTokensForETH(decoded));
            }
            if let Ok(decoded)
                = <SwapExactTokensForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapExactTokensForTokens(decoded));
            }
            if let Ok(decoded)
                = <SwapTokensForExactETHCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapTokensForExactETH(decoded));
            }
            if let Ok(decoded)
                = <SwapTokensForExactTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapTokensForExactTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RouterEventEmitterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SwapETHForExactTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactETHForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapTokensForExactETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapTokensForExactTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RouterEventEmitterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SwapETHForExactTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactETHForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapTokensForExactETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapTokensForExactTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<SwapETHForExactTokensCall> for RouterEventEmitterCalls {
        fn from(value: SwapETHForExactTokensCall) -> Self {
            Self::SwapETHForExactTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensCall> for RouterEventEmitterCalls {
        fn from(value: SwapExactETHForTokensCall) -> Self {
            Self::SwapExactETHForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHCall> for RouterEventEmitterCalls {
        fn from(value: SwapExactTokensForETHCall) -> Self {
            Self::SwapExactTokensForETH(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensCall>
    for RouterEventEmitterCalls {
        fn from(value: SwapExactTokensForTokensCall) -> Self {
            Self::SwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<SwapTokensForExactETHCall> for RouterEventEmitterCalls {
        fn from(value: SwapTokensForExactETHCall) -> Self {
            Self::SwapTokensForExactETH(value)
        }
    }
    impl ::core::convert::From<SwapTokensForExactTokensCall>
    for RouterEventEmitterCalls {
        fn from(value: SwapTokensForExactTokensCall) -> Self {
            Self::SwapTokensForExactTokens(value)
        }
    }
}
