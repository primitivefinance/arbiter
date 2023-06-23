pub use example_oracle_simple::*;
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
pub mod example_oracle_simple {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
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
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PERIOD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PERIOD"),
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
                    ::std::borrow::ToOwned::to_owned("blockTimestampLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blockTimestampLast"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("consult"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("consult"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("price0Average"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("price0Average"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        224usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint224"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("price0CumulativeLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "price0CumulativeLast",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("price1Average"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("price1Average"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        224usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint224"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("price1CumulativeLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "price1CumulativeLast",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("token0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token0"),
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
                    ::std::borrow::ToOwned::to_owned("token1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token1"),
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
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static EXAMPLEORACLESIMPLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0WWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`@Qb\0\x14{8\x03\x80b\0\x14{\x839\x81\x81\x01`@R``\x81\x10\x15b\0\0\xA9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` b\0\x14[\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x80\x83\x01Q`@\x90\x93\x01Q\x91\x92\x91\x90`\0\x90b\0\0\xD9\x90\x85\x90\x85\x90\x85\x90b\0\t\xF1b\0\x05\xE3\x82\x1B\x17\x90\x1CV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16``\x1B\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x01jWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` b\0\x14;\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15b\0\x01\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15b\0\x01\xC2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` b\0\x14[\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ``\x1B`\x01`\x01``\x1B\x03\x19\x16`\xA0R`@\x80Qc\xD2\x12 \xA7`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x91c\xD2\x12 \xA7\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x02MWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` b\0\x14;\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15b\0\x02bW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15b\0\x02\xA5WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` b\0\x14[\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ``\x1B`\x01`\x01``\x1B\x03\x19\x16`\xC0R`@\x80QcY\t\xC0\xD5`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x91cY\t\xC0\xD5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x030WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` b\0\x14;\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15b\0\x03EW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15b\0\x03\x88WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` b\0\x14[\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\0U`@\x80QcZ=T\x93`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x91cZ=T\x93\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04\x06WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` b\0\x14;\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15b\0\x04\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15b\0\x04^WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` b\0\x14[\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\x01U`@\x80Qc\x02@\xBCk`\xE2\x1B\x81R\x90Q`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x85\x16\x91c\t\x02\xF1\xAC\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04\xE1WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` b\0\x14;\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15b\0\x04\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15b\0\x059WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` b\0\x14[\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x82\x01Q`@\x90\x92\x01Q`\x02\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U\x92P\x90P`\x01`\x01`p\x1B\x03\x82\x16\x15\x80\x15\x90b\0\x05\x85WP`\x01`\x01`p\x1B\x03\x81\x16\x15\x15[b\0\x05\xD7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FExampleOracleSimple: NO_RESERVES`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPPb\0\x07\xA3V[`\0\x80\x80b\0\x05\xFC\x85\x85`\x01`\x01`\xE0\x1B\x03b\0\x06\xC0\x16V[`@\x80Q`\x01`\x01``\x1B\x03\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7FO\xE5\x99}g\xF8\x08\x18\xFF\x0BS\xBCZ\x01\xB9\x7F\xEF\xCD\xD7\x89\xB0\xA7\x92g\x97\xF6#\x87>w(\xC9`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15b\0\x07\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80b\0\x14\x16`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10b\0\x078W\x82\x84b\0\x07;V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16b\0\x07\x9CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\x80Q``\x1C`\xA0Q``\x1C`\xC0Q``\x1Ca\x0C3b\0\x07\xE3`\09\x80a\x02\xF1R\x80a\x05\x15RP\x80a\x02OR\x80a\x02uRP\x80a\x03\xC3RPa\x0C3`\0\xF3\xFE`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\xE4W`\x005`\xE0\x1C\x80c\xA2\xE6 E\x11a\0\xACW\x80c\xA2\xE6 E\x14a\x02\nW\x80c\xA6\xBBE9\x14a\x02\x14W\x80c\xB4\xD1\xD7\x95\x14a\x02\x1CW\x80c\xC5p\n\x02\x14a\x02$W\x80c\xD2\x12 \xA7\x14a\x02EWa\0\xE4V[\x80c\r\xFE\x16\x81\x14a\x01BW\x80c=\xDA\xC9S\x14a\x01fW\x80cY\t\xC0\xD5\x14a\x01\xD6W\x80cZ=T\x93\x14a\x01\xDEW\x80c^j\xAF,\x14a\x01\xE6W[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x01Ja\x02MV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\xC4`\x04\x806\x03`@\x81\x10\x15a\x01\xAEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x02qV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\xC4a\x03\x9EV[a\x01\xC4a\x03\xA4V[a\x01\xEEa\x03\xAAV[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02\x12a\x03\xB9V[\0[a\x01\xEEa\x04\xF1V[a\x01\xC4a\x05\0V[a\x02,a\x05\x07V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01Ja\x05\x13V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02\xEFW`@\x80Q` \x81\x01\x90\x91R`\x03T`\x01`\x01`\xE0\x1B\x03\x16\x81Ra\x02\xDF\x90a\x02\xDA\x90\x84c\xFF\xFF\xFF\xFFa\x057\x16V[a\x05\xCBV[`\x01`\x01`\x90\x1B\x03\x16\x90Pa\x03\x98V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x0B\xB5`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x81\x01\x90\x91R`\x04T`\x01`\x01`\xE0\x1B\x03\x16\x81Ra\x03\x8C\x90a\x02\xDA\x90\x84c\xFF\xFF\xFF\xFFa\x057\x16V[`\x01`\x01`\x90\x1B\x03\x16\x90P[\x92\x91PPV[`\0T\x81V[`\x01T\x81V[`\x04T`\x01`\x01`\xE0\x1B\x03\x16\x81V[`\0\x80`\0a\x03\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xD2V[`\x02T\x92\x95P\x90\x93P\x91Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x03\x90b\x01Q\x80\x90\x82\x16\x10\x15a\x04AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80a\x0B\xD7`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`@Q\x80` \x01`@R\x80\x82c\xFF\xFF\xFF\xFF\x16`\0T\x87\x03\x81a\x04_W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x90Q`\x03\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`@\x80Q` \x81\x01\x90\x91R`\x01T\x81\x90c\xFF\xFF\xFF\xFF\x84\x16\x90\x86\x03\x81a\x04\xA6W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x90Q`\x04\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UP`\0\x92\x90\x92U`\x01U`\x02\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x03T`\x01`\x01`\xE0\x1B\x03\x16\x81V[b\x01Q\x80\x81V[`\x02Tc\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05?a\t\xCCV[`\0\x82\x15\x80a\x05eWPP\x82Q`\x01`\x01`\xE0\x1B\x03\x16\x82\x81\x02\x90\x83\x82\x81a\x05bW\xFE[\x04\x14[a\x05\xB6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FFixedPoint: MUL_OVERFLOW\0\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x93\x92PPPV[Q`p\x1C\x90V[`\0\x80`\0a\x05\xDFa\t\x12V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16cY\t\xC0\xD5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06cWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x06wW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x06\xBFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80QcZ=T\x93`\xE0\x1B\x81R\x90Q\x91\x94P`\x01`\x01`\xA0\x1B\x03\x86\x16\x91cZ=T\x93\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x07NWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x07bW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x07\xAAWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qc\x02@\xBCk`\xE2\x1B\x81R\x90Q\x91\x93P`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\t\x02\xF1\xAC\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08?WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x08SW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x08\x9BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x94P\x90\x92P\x90Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x85\x16\x14a\t\x08W\x80\x84\x03c\xFF\xFF\xFF\xFF\x81\x16a\x08\xD5\x84\x86a\t\x1CV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x96\x90\x96\x01\x95c\xFF\xFF\xFF\xFF\x81\x16a\x08\xF6\x85\x85a\t\x1CV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x95\x90\x95\x01\x94P[PPP\x91\x93\x90\x92PV[c\xFF\xFF\xFF\xFFB\x16\x90V[a\t$a\t\xDFV[`\0\x82`\x01`\x01`p\x1B\x03\x16\x11a\t\x82W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FFixedPoint: DIV_BY_ZERO_FRACTION`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x80`\x01`\x01`p\x1B\x03\x84\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`p\x1B`p\x87\x90\x1B\x16\x81a\t\xB7W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x16\x81RP\x90P\x92\x91PPV[`@Q\x80` \x01`@R\x80`\0\x81RP\x90V[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[`\0\x80`\0a\n\0\x85\x85a\n\xB1V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7FO\xE5\x99}g\xF8\x08\x18\xFF\x0BS\xBCZ\x01\xB9\x7F\xEF\xCD\xD7\x89\xB0\xA7\x92g\x97\xF6#\x87>w(\xC9`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0B\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x0B\x90`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0B%W\x82\x84a\x0B(V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\x88W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV\xFEUniswapV2Library: IDENTICAL_ADDRESSESExampleOracleSimple: INVALID_TOKENExampleOracleSimple: PERIOD_NOT_ELAPSED\xA2dipfsX\"\x12 \xAC\xCA\xFB\x19\xA5@\x9B\xABn\x8D\xF4\xC1\x15\x11TYhR\x129\x99N\xFC}96\x8F$b\x8D\xDC\x9DdsolcC\0\x06\x06\x003UniswapV2Library: IDENTICAL_ADDRESSESTarget contract does not containCalldata too short\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
    /// The bytecode of the contract.
    pub static EXAMPLEORACLESIMPLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\xE4W`\x005`\xE0\x1C\x80c\xA2\xE6 E\x11a\0\xACW\x80c\xA2\xE6 E\x14a\x02\nW\x80c\xA6\xBBE9\x14a\x02\x14W\x80c\xB4\xD1\xD7\x95\x14a\x02\x1CW\x80c\xC5p\n\x02\x14a\x02$W\x80c\xD2\x12 \xA7\x14a\x02EWa\0\xE4V[\x80c\r\xFE\x16\x81\x14a\x01BW\x80c=\xDA\xC9S\x14a\x01fW\x80cY\t\xC0\xD5\x14a\x01\xD6W\x80cZ=T\x93\x14a\x01\xDEW\x80c^j\xAF,\x14a\x01\xE6W[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x01Ja\x02MV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\xC4`\x04\x806\x03`@\x81\x10\x15a\x01\xAEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x02qV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\xC4a\x03\x9EV[a\x01\xC4a\x03\xA4V[a\x01\xEEa\x03\xAAV[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02\x12a\x03\xB9V[\0[a\x01\xEEa\x04\xF1V[a\x01\xC4a\x05\0V[a\x02,a\x05\x07V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01Ja\x05\x13V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02\xEFW`@\x80Q` \x81\x01\x90\x91R`\x03T`\x01`\x01`\xE0\x1B\x03\x16\x81Ra\x02\xDF\x90a\x02\xDA\x90\x84c\xFF\xFF\xFF\xFFa\x057\x16V[a\x05\xCBV[`\x01`\x01`\x90\x1B\x03\x16\x90Pa\x03\x98V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x0B\xB5`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x81\x01\x90\x91R`\x04T`\x01`\x01`\xE0\x1B\x03\x16\x81Ra\x03\x8C\x90a\x02\xDA\x90\x84c\xFF\xFF\xFF\xFFa\x057\x16V[`\x01`\x01`\x90\x1B\x03\x16\x90P[\x92\x91PPV[`\0T\x81V[`\x01T\x81V[`\x04T`\x01`\x01`\xE0\x1B\x03\x16\x81V[`\0\x80`\0a\x03\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xD2V[`\x02T\x92\x95P\x90\x93P\x91Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x03\x90b\x01Q\x80\x90\x82\x16\x10\x15a\x04AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80a\x0B\xD7`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`@Q\x80` \x01`@R\x80\x82c\xFF\xFF\xFF\xFF\x16`\0T\x87\x03\x81a\x04_W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x90Q`\x03\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`@\x80Q` \x81\x01\x90\x91R`\x01T\x81\x90c\xFF\xFF\xFF\xFF\x84\x16\x90\x86\x03\x81a\x04\xA6W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x90Q`\x04\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UP`\0\x92\x90\x92U`\x01U`\x02\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x03T`\x01`\x01`\xE0\x1B\x03\x16\x81V[b\x01Q\x80\x81V[`\x02Tc\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05?a\t\xCCV[`\0\x82\x15\x80a\x05eWPP\x82Q`\x01`\x01`\xE0\x1B\x03\x16\x82\x81\x02\x90\x83\x82\x81a\x05bW\xFE[\x04\x14[a\x05\xB6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FFixedPoint: MUL_OVERFLOW\0\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x93\x92PPPV[Q`p\x1C\x90V[`\0\x80`\0a\x05\xDFa\t\x12V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16cY\t\xC0\xD5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06cWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x06wW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x06\xBFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80QcZ=T\x93`\xE0\x1B\x81R\x90Q\x91\x94P`\x01`\x01`\xA0\x1B\x03\x86\x16\x91cZ=T\x93\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x07NWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x07bW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x07\xAAWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qc\x02@\xBCk`\xE2\x1B\x81R\x90Q\x91\x93P`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\t\x02\xF1\xAC\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08?WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x08SW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x08\x9BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x94P\x90\x92P\x90Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x85\x16\x14a\t\x08W\x80\x84\x03c\xFF\xFF\xFF\xFF\x81\x16a\x08\xD5\x84\x86a\t\x1CV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x96\x90\x96\x01\x95c\xFF\xFF\xFF\xFF\x81\x16a\x08\xF6\x85\x85a\t\x1CV[Q`\x01`\x01`\xE0\x1B\x03\x16\x02\x95\x90\x95\x01\x94P[PPP\x91\x93\x90\x92PV[c\xFF\xFF\xFF\xFFB\x16\x90V[a\t$a\t\xDFV[`\0\x82`\x01`\x01`p\x1B\x03\x16\x11a\t\x82W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FFixedPoint: DIV_BY_ZERO_FRACTION`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q` \x81\x01\x90\x91R\x80`\x01`\x01`p\x1B\x03\x84\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`p\x1B`p\x87\x90\x1B\x16\x81a\t\xB7W\xFE[\x04`\x01`\x01`\xE0\x1B\x03\x16\x81RP\x90P\x92\x91PPV[`@Q\x80` \x01`@R\x80`\0\x81RP\x90V[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[`\0\x80`\0a\n\0\x85\x85a\n\xB1V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7FO\xE5\x99}g\xF8\x08\x18\xFF\x0BS\xBCZ\x01\xB9\x7F\xEF\xCD\xD7\x89\xB0\xA7\x92g\x97\xF6#\x87>w(\xC9`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0B\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x0B\x90`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0B%W\x82\x84a\x0B(V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\x88W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV\xFEUniswapV2Library: IDENTICAL_ADDRESSESExampleOracleSimple: INVALID_TOKENExampleOracleSimple: PERIOD_NOT_ELAPSED\xA2dipfsX\"\x12 \xAC\xCA\xFB\x19\xA5@\x9B\xABn\x8D\xF4\xC1\x15\x11TYhR\x129\x99N\xFC}96\x8F$b\x8D\xDC\x9DdsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static EXAMPLEORACLESIMPLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ExampleOracleSimple<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExampleOracleSimple<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExampleOracleSimple<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExampleOracleSimple<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExampleOracleSimple<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExampleOracleSimple))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExampleOracleSimple<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXAMPLEORACLESIMPLE_ABI.clone(),
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
                EXAMPLEORACLESIMPLE_ABI.clone(),
                EXAMPLEORACLESIMPLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `PERIOD` (0xb4d1d795) function
        pub fn period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 209, 215, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockTimestampLast` (0xc5700a02) function
        pub fn block_timestamp_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([197, 112, 10, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `consult` (0x3ddac953) function
        pub fn consult(
            &self,
            token: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([61, 218, 201, 83], (token, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price0Average` (0xa6bb4539) function
        pub fn price_0_average(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 187, 69, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price0CumulativeLast` (0x5909c0d5) function
        pub fn price_0_cumulative_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 9, 192, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price1Average` (0x5e6aaf2c) function
        pub fn price_1_average(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 106, 175, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price1CumulativeLast` (0x5a3d5493) function
        pub fn price_1_cumulative_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 61, 84, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token0` (0x0dfe1681) function
        pub fn token_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token1` (0xd21220a7) function
        pub fn token_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0xa2e62045) function
        pub fn update(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 230, 32, 69], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExampleOracleSimple<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `PERIOD` function with signature `PERIOD()` and selector `0xb4d1d795`
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
    #[ethcall(name = "PERIOD", abi = "PERIOD()")]
    pub struct PeriodCall;
    ///Container type for all input parameters for the `blockTimestampLast` function with signature `blockTimestampLast()` and selector `0xc5700a02`
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
    #[ethcall(name = "blockTimestampLast", abi = "blockTimestampLast()")]
    pub struct BlockTimestampLastCall;
    ///Container type for all input parameters for the `consult` function with signature `consult(address,uint256)` and selector `0x3ddac953`
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
    #[ethcall(name = "consult", abi = "consult(address,uint256)")]
    pub struct ConsultCall {
        pub token: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `price0Average` function with signature `price0Average()` and selector `0xa6bb4539`
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
    #[ethcall(name = "price0Average", abi = "price0Average()")]
    pub struct Price0AverageCall;
    ///Container type for all input parameters for the `price0CumulativeLast` function with signature `price0CumulativeLast()` and selector `0x5909c0d5`
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
    #[ethcall(name = "price0CumulativeLast", abi = "price0CumulativeLast()")]
    pub struct Price0CumulativeLastCall;
    ///Container type for all input parameters for the `price1Average` function with signature `price1Average()` and selector `0x5e6aaf2c`
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
    #[ethcall(name = "price1Average", abi = "price1Average()")]
    pub struct Price1AverageCall;
    ///Container type for all input parameters for the `price1CumulativeLast` function with signature `price1CumulativeLast()` and selector `0x5a3d5493`
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
    #[ethcall(name = "price1CumulativeLast", abi = "price1CumulativeLast()")]
    pub struct Price1CumulativeLastCall;
    ///Container type for all input parameters for the `token0` function with signature `token0()` and selector `0x0dfe1681`
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
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    ///Container type for all input parameters for the `token1` function with signature `token1()` and selector `0xd21220a7`
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
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    ///Container type for all input parameters for the `update` function with signature `update()` and selector `0xa2e62045`
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
    #[ethcall(name = "update", abi = "update()")]
    pub struct UpdateCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExampleOracleSimpleCalls {
        Period(PeriodCall),
        BlockTimestampLast(BlockTimestampLastCall),
        Consult(ConsultCall),
        Price0Average(Price0AverageCall),
        Price0CumulativeLast(Price0CumulativeLastCall),
        Price1Average(Price1AverageCall),
        Price1CumulativeLast(Price1CumulativeLastCall),
        Token0(Token0Call),
        Token1(Token1Call),
        Update(UpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleOracleSimpleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <PeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Period(decoded));
            }
            if let Ok(decoded)
                = <BlockTimestampLastCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BlockTimestampLast(decoded));
            }
            if let Ok(decoded)
                = <ConsultCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Consult(decoded));
            }
            if let Ok(decoded)
                = <Price0AverageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Price0Average(decoded));
            }
            if let Ok(decoded)
                = <Price0CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Price0CumulativeLast(decoded));
            }
            if let Ok(decoded)
                = <Price1AverageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Price1Average(decoded));
            }
            if let Ok(decoded)
                = <Price1CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Price1CumulativeLast(decoded));
            }
            if let Ok(decoded)
                = <Token0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token0(decoded));
            }
            if let Ok(decoded)
                = <Token1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token1(decoded));
            }
            if let Ok(decoded)
                = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleOracleSimpleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Period(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlockTimestampLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Consult(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Price0Average(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Price0CumulativeLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Price1Average(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Price1CumulativeLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ExampleOracleSimpleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Period(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlockTimestampLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Consult(element) => ::core::fmt::Display::fmt(element, f),
                Self::Price0Average(element) => ::core::fmt::Display::fmt(element, f),
                Self::Price0CumulativeLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Price1Average(element) => ::core::fmt::Display::fmt(element, f),
                Self::Price1CumulativeLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Token0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PeriodCall> for ExampleOracleSimpleCalls {
        fn from(value: PeriodCall) -> Self {
            Self::Period(value)
        }
    }
    impl ::core::convert::From<BlockTimestampLastCall> for ExampleOracleSimpleCalls {
        fn from(value: BlockTimestampLastCall) -> Self {
            Self::BlockTimestampLast(value)
        }
    }
    impl ::core::convert::From<ConsultCall> for ExampleOracleSimpleCalls {
        fn from(value: ConsultCall) -> Self {
            Self::Consult(value)
        }
    }
    impl ::core::convert::From<Price0AverageCall> for ExampleOracleSimpleCalls {
        fn from(value: Price0AverageCall) -> Self {
            Self::Price0Average(value)
        }
    }
    impl ::core::convert::From<Price0CumulativeLastCall> for ExampleOracleSimpleCalls {
        fn from(value: Price0CumulativeLastCall) -> Self {
            Self::Price0CumulativeLast(value)
        }
    }
    impl ::core::convert::From<Price1AverageCall> for ExampleOracleSimpleCalls {
        fn from(value: Price1AverageCall) -> Self {
            Self::Price1Average(value)
        }
    }
    impl ::core::convert::From<Price1CumulativeLastCall> for ExampleOracleSimpleCalls {
        fn from(value: Price1CumulativeLastCall) -> Self {
            Self::Price1CumulativeLast(value)
        }
    }
    impl ::core::convert::From<Token0Call> for ExampleOracleSimpleCalls {
        fn from(value: Token0Call) -> Self {
            Self::Token0(value)
        }
    }
    impl ::core::convert::From<Token1Call> for ExampleOracleSimpleCalls {
        fn from(value: Token1Call) -> Self {
            Self::Token1(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for ExampleOracleSimpleCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    ///Container type for all return fields from the `PERIOD` function with signature `PERIOD()` and selector `0xb4d1d795`
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
    pub struct PeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `blockTimestampLast` function with signature `blockTimestampLast()` and selector `0xc5700a02`
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
    pub struct BlockTimestampLastReturn(pub u32);
    ///Container type for all return fields from the `consult` function with signature `consult(address,uint256)` and selector `0x3ddac953`
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
    ///Container type for all return fields from the `price0Average` function with signature `price0Average()` and selector `0xa6bb4539`
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
    pub struct Price0AverageReturn {
        pub x: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `price0CumulativeLast` function with signature `price0CumulativeLast()` and selector `0x5909c0d5`
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
    pub struct Price0CumulativeLastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `price1Average` function with signature `price1Average()` and selector `0x5e6aaf2c`
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
    pub struct Price1AverageReturn {
        pub x: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `price1CumulativeLast` function with signature `price1CumulativeLast()` and selector `0x5a3d5493`
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
    pub struct Price1CumulativeLastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `token0` function with signature `token0()` and selector `0x0dfe1681`
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
    pub struct Token0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token1` function with signature `token1()` and selector `0xd21220a7`
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
    pub struct Token1Return(pub ::ethers::core::types::Address);
}
