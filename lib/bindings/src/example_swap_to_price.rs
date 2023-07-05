pub use example_swap_to_price::*;
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
pub mod example_swap_to_price {
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
                        name: ::std::borrow::ToOwned::to_owned("router_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IUniswapV2Router01",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("router"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("router"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IUniswapV2Router01",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapToPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapToPrice"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("truePriceTokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("truePriceTokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxSpendTokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxSpendTokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EXAMPLESWAPTOPRICE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`@Qa\x12\x118\x03\x80a\x12\x11\x839\x81\x81\x01`@R`@\x81\x10\x15a\0\xABWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\xA0R\x91\x1B\x16`\x80R`\x80Q``\x1C`\xA0Q``\x1Ca\x11\x0Ca\x01\x05`\09\x80a\x02bR\x80a\x06\xDFRP\x80a\x03,R\x80a\x03\xCFR\x80a\x07\x03RPa\x11\x0C`\0\xF3\xFE`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\x87W`\x005`\xE0\x1C\x80c\xBE\xF9\x0F\xD3\x14a\0\xE5W\x80c\xC4Z\x01U\x14a\x01nW\x80c\xF8\x87\xEA@\x14a\x01\x92W[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x01l`\x04\x806\x03a\x01\0\x81\x10\x15a\x01.WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x91`\xC0\x82\x015\x16\x90`\xE0\x015a\x01\x9AV[\0[a\x01va\x06\xDDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01va\x07\x01V[\x85\x15\x80\x15\x90a\x01\xA8WP\x84\x15\x15[a\x01\xF9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FExampleSwapToPrice: ZERO_PRICE\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x83\x15\x15\x80a\x02\x06WP\x82\x15\x15[a\x02WW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FExampleSwapToPrice: ZERO_SPEND\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80`\0\x80a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x8Da\x07%V[\x91P\x91Pa\x02\x98\x8A\x8A\x84\x84a\x08nV[\x90\x94P\x92PPP\x80a\x02\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x10\x91`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x82a\x02\xE8W\x85a\x02\xEAV[\x86[\x90P\x80\x82\x11\x15a\x02\xF8W\x80\x91P[`\0\x83a\x03\x05W\x8Aa\x03\x07V[\x8B[\x90P`\0\x84a\x03\x16W\x8Ca\x03\x18V[\x8B[\x90Pa\x03&\x8230\x87a\tZV[a\x03Q\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\n\xE9V[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x03\x7FW\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\x01\x81Q\x81\x10a\x03\xADW\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c8\xED\x179\x86`\0\x84\x8C\x8C`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x86\x81R` \x01\x85\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x85\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x04{W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04cV[PPPP\x90P\x01\x96PPPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xEDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x05\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@R` \x81\x10\x15a\x05\\WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\x05\xC9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FABI memory decoding: invalid dat`D\x90\x81Rh0\x9087\xB4\xB7:2\xB9`\xB9\x1B`dR\x90`\x84\x90\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x06)WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`'`$R\x7FABI memory decoding: invalid dat`D\x90\x81Rf\x18H\x1C\xDD\x18\\\x9D`\xCA\x1B`dR\x90`\x84\x90\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x06\x92WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`(`$R\x7FABI memory decoding: invalid dat`D\x90\x81Rg\x0C$\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`dR\x90`\x84\x90\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x06\xBFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xA7V[PPPP\x90P\x01`@RPPPPPPPPPPPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0a\x074\x85\x85a\x0C\x85V[P\x90P`\0\x80a\x07E\x88\x88\x88a\rcV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xC6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x07\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x08\"WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14a\x08\\W\x80\x82a\x08_V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x85a\x08}\x85\x87\x86a\x0E#V[\x10\x91P`\0a\x08\x92\x85\x85c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[\x90P`\0a\x08\xE4a\x08\xDFa\x08\xAE\x84a\x03\xE8c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[\x86a\x08\xB9W\x89a\x08\xBBV[\x8A[a\x08\xDAa\x03\xE5\x89a\x08\xCCW\x8Da\x08\xCEV[\x8C[\x90c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[a\x0E#V[a\x0F,V[\x90P`\0a\x03\xE5\x85a\t\x07Wa\t\x02\x87a\x03\xE8c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[a\t\x19V[a\t\x19\x88a\x03\xE8c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[\x81a\t W\xFE[\x04\x90P\x80\x82\x10\x15a\t;WP`\0\x93P\x83\x92Pa\tQ\x91PPV[a\tK\x82\x82c\xFF\xFF\xFF\xFFa\x0F~\x16V[\x93PPPP[\x94P\x94\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\t\xDFW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\t\xC0V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\nAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\nFV[``\x91P[P\x91P\x91P\x81\x80\x15a\n\xA6WP\x80Q\x15\x80a\n\xA6WP\x80\x80` \x01\x90Q` \x81\x10\x15a\n\xA3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\n\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a\x10\xB3`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0BfW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0BGV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B\xC8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xCDV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0C-WP\x80Q\x15\x80a\x0C-WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x0C*WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\x0C~W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FTransferHelper: APPROVE_FAILED\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0C\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x10l`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0C\xF9W\x82\x84a\x0C\xFCV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\r\\W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0\x80`\0a\rr\x85\x85a\x0C\x85V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x96\xE8\xACBw\x19\x8F\xF8\xB6\xF7\x85G\x8A\xA9\xA3\x9F@<\xB7h\xDD\x02\xCB\xEE2l>}\xA3H\x84_`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80`\0a\x0E2\x86\x86a\x0F\xCEV[\x91P\x91P`\0\x84\x80a\x0E@W\xFE[\x86\x88\t\x90P\x82\x81\x11\x15a\x0ETW`\x01\x82\x03\x91P[\x80\x83\x03\x92P\x84\x82\x10a\x0E\xADW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FFullMath: FULLDIV_OVERFLOW\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0E\xB8\x83\x83\x87a\x0F\xFBV[\x97\x96PPPPPPPV[`\0\x81\x15\x80a\x0E\xDEWPP\x80\x82\x02\x82\x82\x82\x81a\x0E\xDBW\xFE[\x04\x14[a\x0F&W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92\x91PPV[`\0`\x03\x82\x11\x15a\x0FoWP\x80`\x01`\x02\x82\x04\x01[\x81\x81\x10\x15a\x0FiW\x80\x91P`\x02\x81\x82\x85\x81a\x0FXW\xFE[\x04\x01\x81a\x0FaW\xFE[\x04\x90Pa\x0FAV[Pa\x0FyV[\x81\x15a\x0FyWP`\x01[\x91\x90PV[\x80\x82\x03\x82\x81\x11\x15a\x0F&W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x80`\0\x19\x84\x86\t\x90P\x83\x85\x02\x92P\x82\x81\x03\x91P\x82\x81\x10\x15a\x0F\xF3W`\x01\x82\x03\x91P[P\x92P\x92\x90PV[`\0\x81\x81\x03\x82\x16\x80\x83\x81a\x10\x0BW\xFE[\x04\x92P\x80\x85\x81a\x10\x17W\xFE[\x04\x94P\x80\x81`\0\x03\x81a\x10&W\xFE[`\x02\x85\x81\x03\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x95\x86\x02\x90\x03\x90\x94\x02\x93\x04`\x01\x01\x93\x90\x93\x02\x93\x90\x93\x01\x02\x92\x91PPV\xFEUniswapV2Library: IDENTICAL_ADDRESSESExampleSwapToPrice: ZERO_AMOUNT_INTransferHelper: TRANSFER_FROM_FAILED\xA2dipfsX\"\x12 \xA0\x8A\xF4\x19O\xD3\x84+\xF5\xCB]\xD6\x02T1\xCE\xA0\xEE\xDCa\x1Bk\xC1\xC1\xBF4\x18\x06\x9B-\x01\x92dsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static EXAMPLESWAPTOPRICE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\x87W`\x005`\xE0\x1C\x80c\xBE\xF9\x0F\xD3\x14a\0\xE5W\x80c\xC4Z\x01U\x14a\x01nW\x80c\xF8\x87\xEA@\x14a\x01\x92W[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x01l`\x04\x806\x03a\x01\0\x81\x10\x15a\x01.WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x91`\xC0\x82\x015\x16\x90`\xE0\x015a\x01\x9AV[\0[a\x01va\x06\xDDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01va\x07\x01V[\x85\x15\x80\x15\x90a\x01\xA8WP\x84\x15\x15[a\x01\xF9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FExampleSwapToPrice: ZERO_PRICE\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x83\x15\x15\x80a\x02\x06WP\x82\x15\x15[a\x02WW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FExampleSwapToPrice: ZERO_SPEND\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80`\0\x80a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x8Da\x07%V[\x91P\x91Pa\x02\x98\x8A\x8A\x84\x84a\x08nV[\x90\x94P\x92PPP\x80a\x02\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x10\x91`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x82a\x02\xE8W\x85a\x02\xEAV[\x86[\x90P\x80\x82\x11\x15a\x02\xF8W\x80\x91P[`\0\x83a\x03\x05W\x8Aa\x03\x07V[\x8B[\x90P`\0\x84a\x03\x16W\x8Ca\x03\x18V[\x8B[\x90Pa\x03&\x8230\x87a\tZV[a\x03Q\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\n\xE9V[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x03\x7FW\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\x01\x81Q\x81\x10a\x03\xADW\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c8\xED\x179\x86`\0\x84\x8C\x8C`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x86\x81R` \x01\x85\x81R` \x01\x80` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81\x03\x82R\x85\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x04{W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04cV[PPPP\x90P\x01\x96PPPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xEDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x05\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@R` \x81\x10\x15a\x05\\WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\x05\xC9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FABI memory decoding: invalid dat`D\x90\x81Rh0\x9087\xB4\xB7:2\xB9`\xB9\x1B`dR\x90`\x84\x90\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x06)WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`'`$R\x7FABI memory decoding: invalid dat`D\x90\x81Rf\x18H\x1C\xDD\x18\\\x9D`\xCA\x1B`dR\x90`\x84\x90\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x06\x92WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`(`$R\x7FABI memory decoding: invalid dat`D\x90\x81Rg\x0C$\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`dR\x90`\x84\x90\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x06\xBFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xA7V[PPPP\x90P\x01`@RPPPPPPPPPPPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0a\x074\x85\x85a\x0C\x85V[P\x90P`\0\x80a\x07E\x88\x88\x88a\rcV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xC6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R\x7FTarget contract does not contain`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x07\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x08\"WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14a\x08\\W\x80\x82a\x08_V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x85a\x08}\x85\x87\x86a\x0E#V[\x10\x91P`\0a\x08\x92\x85\x85c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[\x90P`\0a\x08\xE4a\x08\xDFa\x08\xAE\x84a\x03\xE8c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[\x86a\x08\xB9W\x89a\x08\xBBV[\x8A[a\x08\xDAa\x03\xE5\x89a\x08\xCCW\x8Da\x08\xCEV[\x8C[\x90c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[a\x0E#V[a\x0F,V[\x90P`\0a\x03\xE5\x85a\t\x07Wa\t\x02\x87a\x03\xE8c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[a\t\x19V[a\t\x19\x88a\x03\xE8c\xFF\xFF\xFF\xFFa\x0E\xC3\x16V[\x81a\t W\xFE[\x04\x90P\x80\x82\x10\x15a\t;WP`\0\x93P\x83\x92Pa\tQ\x91PPV[a\tK\x82\x82c\xFF\xFF\xFF\xFFa\x0F~\x16V[\x93PPPP[\x94P\x94\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\t\xDFW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\t\xC0V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\nAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\nFV[``\x91P[P\x91P\x91P\x81\x80\x15a\n\xA6WP\x80Q\x15\x80a\n\xA6WP\x80\x80` \x01\x90Q` \x81\x10\x15a\n\xA3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\n\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a\x10\xB3`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0BfW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0BGV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B\xC8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xCDV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0C-WP\x80Q\x15\x80a\x0C-WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x0C*WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\x0C~W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FTransferHelper: APPROVE_FAILED\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0C\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x10l`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0C\xF9W\x82\x84a\x0C\xFCV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\r\\W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0\x80`\0a\rr\x85\x85a\x0C\x85V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x96\xE8\xACBw\x19\x8F\xF8\xB6\xF7\x85G\x8A\xA9\xA3\x9F@<\xB7h\xDD\x02\xCB\xEE2l>}\xA3H\x84_`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80`\0a\x0E2\x86\x86a\x0F\xCEV[\x91P\x91P`\0\x84\x80a\x0E@W\xFE[\x86\x88\t\x90P\x82\x81\x11\x15a\x0ETW`\x01\x82\x03\x91P[\x80\x83\x03\x92P\x84\x82\x10a\x0E\xADW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FFullMath: FULLDIV_OVERFLOW\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0E\xB8\x83\x83\x87a\x0F\xFBV[\x97\x96PPPPPPPV[`\0\x81\x15\x80a\x0E\xDEWPP\x80\x82\x02\x82\x82\x82\x81a\x0E\xDBW\xFE[\x04\x14[a\x0F&W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92\x91PPV[`\0`\x03\x82\x11\x15a\x0FoWP\x80`\x01`\x02\x82\x04\x01[\x81\x81\x10\x15a\x0FiW\x80\x91P`\x02\x81\x82\x85\x81a\x0FXW\xFE[\x04\x01\x81a\x0FaW\xFE[\x04\x90Pa\x0FAV[Pa\x0FyV[\x81\x15a\x0FyWP`\x01[\x91\x90PV[\x80\x82\x03\x82\x81\x11\x15a\x0F&W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x80`\0\x19\x84\x86\t\x90P\x83\x85\x02\x92P\x82\x81\x03\x91P\x82\x81\x10\x15a\x0F\xF3W`\x01\x82\x03\x91P[P\x92P\x92\x90PV[`\0\x81\x81\x03\x82\x16\x80\x83\x81a\x10\x0BW\xFE[\x04\x92P\x80\x85\x81a\x10\x17W\xFE[\x04\x94P\x80\x81`\0\x03\x81a\x10&W\xFE[`\x02\x85\x81\x03\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x95\x86\x02\x90\x03\x90\x94\x02\x93\x04`\x01\x01\x93\x90\x93\x02\x93\x90\x93\x01\x02\x92\x91PPV\xFEUniswapV2Library: IDENTICAL_ADDRESSESExampleSwapToPrice: ZERO_AMOUNT_INTransferHelper: TRANSFER_FROM_FAILED\xA2dipfsX\"\x12 \xA0\x8A\xF4\x19O\xD3\x84+\xF5\xCB]\xD6\x02T1\xCE\xA0\xEE\xDCa\x1Bk\xC1\xC1\xBF4\x18\x06\x9B-\x01\x92dsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static EXAMPLESWAPTOPRICE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ExampleSwapToPrice<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExampleSwapToPrice<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExampleSwapToPrice<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExampleSwapToPrice<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExampleSwapToPrice<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExampleSwapToPrice))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExampleSwapToPrice<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXAMPLESWAPTOPRICE_ABI.clone(),
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
                EXAMPLESWAPTOPRICE_ABI.clone(),
                EXAMPLESWAPTOPRICE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `router` (0xf887ea40) function
        pub fn router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 135, 234, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapToPrice` (0xbef90fd3) function
        pub fn swap_to_price(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            true_price_token_a: ::ethers::core::types::U256,
            true_price_token_b: ::ethers::core::types::U256,
            max_spend_token_a: ::ethers::core::types::U256,
            max_spend_token_b: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [190, 249, 15, 211],
                    (
                        token_a,
                        token_b,
                        true_price_token_a,
                        true_price_token_b,
                        max_spend_token_a,
                        max_spend_token_b,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExampleSwapToPrice<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Container type for all input parameters for the `router` function with signature `router()` and selector `0xf887ea40`
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
    #[ethcall(name = "router", abi = "router()")]
    pub struct RouterCall;
    ///Container type for all input parameters for the `swapToPrice` function with signature `swapToPrice(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `0xbef90fd3`
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
        name = "swapToPrice",
        abi = "swapToPrice(address,address,uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct SwapToPriceCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub true_price_token_a: ::ethers::core::types::U256,
        pub true_price_token_b: ::ethers::core::types::U256,
        pub max_spend_token_a: ::ethers::core::types::U256,
        pub max_spend_token_b: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExampleSwapToPriceCalls {
        Factory(FactoryCall),
        Router(RouterCall),
        SwapToPrice(SwapToPriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleSwapToPriceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded)
                = <RouterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Router(decoded));
            }
            if let Ok(decoded)
                = <SwapToPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapToPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleSwapToPriceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Router(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapToPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExampleSwapToPriceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Router(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapToPrice(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FactoryCall> for ExampleSwapToPriceCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<RouterCall> for ExampleSwapToPriceCalls {
        fn from(value: RouterCall) -> Self {
            Self::Router(value)
        }
    }
    impl ::core::convert::From<SwapToPriceCall> for ExampleSwapToPriceCalls {
        fn from(value: SwapToPriceCall) -> Self {
            Self::SwapToPrice(value)
        }
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
    ///Container type for all return fields from the `router` function with signature `router()` and selector `0xf887ea40`
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
    pub struct RouterReturn(pub ::ethers::core::types::Address);
}
