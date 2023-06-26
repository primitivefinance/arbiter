pub use example_compute_liquidity_value::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod example_compute_liquidity_value {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("factory_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("factory"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getGasCostOfGetLiquidityValueAfterArbitrageToPrice",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getGasCostOfGetLiquidityValueAfterArbitrageToPrice",
                        ),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("truePriceTokenB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidityAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidityValue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLiquidityValue"),
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
                                name: ::std::borrow::ToOwned::to_owned("liquidityAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenBAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidityValueAfterArbitrageToPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getLiquidityValueAfterArbitrageToPrice",
                        ),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("truePriceTokenB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidityAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenBAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReservesAfterArbitrage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReservesAfterArbitrage",),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("truePriceTokenB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EXAMPLECOMPUTELIQUIDITYVALUE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`@Qa\x15\x028\x03\x80a\x15\x02\x839\x81\x81\x01`@R` \x81\x10\x15a\0\xABWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ``\x81\x90\x1B`\x01`\x01``\x1B\x03\x19\x16`\x80R`\x01`\x01`\xA0\x1B\x03\x16a\x14\x0Fa\0\xF3`\09\x80a\x02\xF4R\x80a\x032R\x80a\x03uR\x80a\x03\xB3R\x80a\x03\xDDRPa\x14\x0F`\0\xF3\xFE`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c5X\xE9L\x14a\0\xFBW\x80ci\xA2\xFC\xBD\x14a\x01{W\x80c\x80\xCA\xA3S\x14a\x01\xE8W\x80c\xC4Z\x01U\x14a\x02gW\x80c\xD9\xB7\xA6\xE7\x14a\x02\x8BW[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x01b`\x04\x806\x03`\x80\x81\x10\x15a\x01<WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x015a\x02\xECV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[a\x01b`\x04\x806\x03`\xA0\x81\x10\x15a\x01\xBCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x81\x015\x90`\x80\x015a\x03*V[a\x02U`\x04\x806\x03`\xA0\x81\x10\x15a\x02)WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x81\x015\x90`\x80\x015a\x03jV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02oa\x03\xB1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01b`\x04\x806\x03``\x81\x10\x15a\x02\xCCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x03\xD5V[`\0\x80a\x03\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x87\x87\x87a\x04\x10V[\x91P\x91P[\x94P\x94\x92PPPV[`\0\x80a\x03[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x88\x88a\x04\xD5V[\x91P\x91P[\x95P\x95\x93PPPPV[`\0\x80Z\x90Pa\x03\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x88\x88a\x04\xD5V[PP`\0Z\x90\x91\x03\x97\x96PPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x04\x04\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x86\x86a\x07\xECV[\x91P\x91P\x93P\x93\x91PPV[`\0\x80a\x04\x1E\x87\x87\x87a\n\xB4V[\x90\x92P\x90P\x81\x15\x80\x15\x90a\x042WP`\0\x81\x11[a\x04mW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`-\x81R` \x01\x80a\x12\xCE`-\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x04|\x86\x86\x86\x86a\x0B\xE4V[\x91P\x91P\x80`\0\x14\x15a\x04\x91WPa\x03`\x90PV[\x81\x15a\x04\xB2W`\0a\x04\xA4\x82\x86\x86a\x0C\xCFV[\x94\x82\x01\x94\x90\x93\x03\x92Pa\x04\xC9V[`\0a\x04\xBF\x82\x85\x87a\x0C\xCFV[\x90\x94\x03\x93P\x91\x82\x01\x91[PP\x95P\x95\x93PPPPV[`\0\x80`\0\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16c\x01~~X`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05TWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x05hW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x05\xA9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P`\0a\x05\xC5\x8A\x8A\x8Aa\r\xBFV[\x90P`\0\x82a\x05\xD5W`\0a\x06\x9DV[\x81`\x01`\x01`\xA0\x1B\x03\x16ctd\xFC=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x06YW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x06\x9AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\x11WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x07%W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x07fWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x90P\x86\x81\x10\x80\x15\x90a\x07zWP`\0\x87\x11[a\x07\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80a\x13\x88`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x07\xC5\x8E\x8E\x8E\x8E\x8Ea\x04\x10V[\x91P\x91Pa\x07\xD7\x82\x82\x85\x8C\x8A\x89a\x0E\x7FV[\x97P\x97PPPPPPP\x96P\x96\x94PPPPPV[`\0\x80`\0\x80a\x07\xFD\x88\x88\x88a\n\xB4V[\x91P\x91P`\0a\x08\x0E\x89\x89\x89a\r\xBFV[\x90P`\0\x80`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16c\x01~~X`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x8CWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x08\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\xE1WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P`\0\x81a\x08\xFEW`\0a\t\xC6V[\x82`\x01`\x01`\xA0\x1B\x03\x16ctd\xFC=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\tnWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\t\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\t\xC3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[\x90P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n:WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\nNW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\n\x8FWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x90Pa\n\xA1\x86\x86\x83\x8C\x87\x87a\x0E\x7FV[\x97P\x97PPPPPPP\x94P\x94\x92PPPV[`\0\x80`\0a\n\xC3\x85\x85a\x0FXV[P\x90P`\0\x80a\n\xD4\x88\x88\x88a\r\xBFV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x0BWW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x0B\x98WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14a\x0B\xD2W\x80\x82a\x0B\xD5V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x85a\x0B\xF3\x85\x87\x86a\x106V[\x10\x91P`\0a\x0C\x08\x85\x85c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\x0CZa\x0CUa\x0C$\x84a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x86a\x0C/W\x89a\x0C1V[\x8A[a\x0CPa\x03\xE5\x89a\x0CBW\x8Da\x0CDV[\x8C[\x90c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[a\x106V[a\x11?V[\x90P`\0a\x03\xE5\x85a\x0C}Wa\x0Cx\x87a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[a\x0C\x8FV[a\x0C\x8F\x88a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x81a\x0C\x96W\xFE[\x04\x90P\x80\x82\x10\x15a\x0C\xB1WP`\0\x93P\x83\x92Pa\x03!\x91PPV[a\x0C\xC1\x82\x82c\xFF\xFF\xFF\xFFa\x11\x91\x16V[\x93PPPP\x94P\x94\x92PPPV[`\0\x80\x84\x11a\r\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80a\x13\xAF`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a\r\x1FWP`\0\x82\x11[a\rZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a\x13 `(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a\rn\x85a\x03\xE5c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\r\x82\x82\x85c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\r\xA8\x83a\r\x9C\x88a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90c\xFF\xFF\xFF\xFFa\x11\xE1\x16V[\x90P\x80\x82\x81a\r\xB3W\xFE[\x04\x97\x96PPPPPPPV[`\0\x80`\0a\r\xCE\x85\x85a\x0FXV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7Ft\xABbfI\x0E\x96b\x9C\xE0k\x06\xFE\xB6\xF7JSuTc\xF6\x02]1\xC3e\xAD\xB6uP\xF3\xDC`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80\x83\x80\x15a\x0E\x8FWP`\0\x83\x11[\x15a\x0F\x17W`\0a\x0E\xA9a\x0CU\x8A\x8Ac\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\x0E\xB6\x85a\x11?V[\x90P\x80\x82\x11\x15a\x0F\x14W\x87`\0a\x0E\xD3\x84\x84c\xFF\xFF\xFF\xFFa\x11\x91\x16V[\x90P`\0a\x0E\xEC\x84a\r\x9C\x87`\x05c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\x0E\xFB\x84\x84\x84a\x106V[\x90Pa\x0F\r\x8C\x82c\xFF\xFF\xFF\xFFa\x11\xE1\x16V[\x9BPPPPP[PP[\x85a\x0F(\x89\x87c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x81a\x0F/W\xFE[\x04\x86a\x0FA\x89\x88c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x81a\x0FHW\xFE[\x04\x91P\x91P\x96P\x96\x94PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0F\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x12\xFB`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0F\xCCW\x82\x84a\x0F\xCFV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10/W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0\x80`\0a\x10E\x86\x86a\x120V[\x91P\x91P`\0\x84\x80a\x10SW\xFE[\x86\x88\t\x90P\x82\x81\x11\x15a\x10gW`\x01\x82\x03\x91P[\x80\x83\x03\x92P\x84\x82\x10a\x10\xC0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FFullMath: FULLDIV_OVERFLOW\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x10\xCB\x83\x83\x87a\x12]V[\x97\x96PPPPPPPV[`\0\x81\x15\x80a\x10\xF1WPP\x80\x82\x02\x82\x82\x82\x81a\x10\xEEW\xFE[\x04\x14[a\x119W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92\x91PPV[`\0`\x03\x82\x11\x15a\x11\x82WP\x80`\x01`\x02\x82\x04\x01[\x81\x81\x10\x15a\x11|W\x80\x91P`\x02\x81\x82\x85\x81a\x11kW\xFE[\x04\x01\x81a\x11tW\xFE[\x04\x90Pa\x11TV[Pa\x11\x8CV[\x81\x15a\x11\x8CWP`\x01[\x91\x90PV[\x80\x82\x03\x82\x81\x11\x15a\x119W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x01\x82\x81\x10\x15a\x119W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x80`\0\x19\x84\x86\t\x90P\x83\x85\x02\x92P\x82\x81\x03\x91P\x82\x81\x10\x15a\x12UW`\x01\x82\x03\x91P[P\x92P\x92\x90PV[`\0\x81\x81\x03\x82\x16\x80\x83\x81a\x12mW\xFE[\x04\x92P\x80\x85\x81a\x12yW\xFE[\x04\x94P\x80\x81`\0\x03\x81a\x12\x88W\xFE[`\x02\x85\x81\x03\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x95\x86\x02\x90\x03\x90\x94\x02\x93\x04`\x01\x01\x93\x90\x93\x02\x93\x90\x93\x01\x02\x92\x91PPV\xFEUniswapV2ArbitrageLibrary: ZERO_PAIR_RESERVESUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Library: INSUFFICIENT_LIQUIDITYTarget contract does not containCalldata too short\0\0\0\0\0\0\0\0\0\0\0\0\0\0ComputeLiquidityValue: LIQUIDITY_AMOUNTUniswapV2Library: INSUFFICIENT_INPUT_AMOUNT\xA2dipfsX\"\x12 \xDF\xC6`\xD5\xBD\x06d\xB8dJ\xA7ug\x95\xF5L\x13WmG\xCAl\x06{/\xB5l\x99jH\xC3\x1AdsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static EXAMPLECOMPUTELIQUIDITYVALUE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c5X\xE9L\x14a\0\xFBW\x80ci\xA2\xFC\xBD\x14a\x01{W\x80c\x80\xCA\xA3S\x14a\x01\xE8W\x80c\xC4Z\x01U\x14a\x02gW\x80c\xD9\xB7\xA6\xE7\x14a\x02\x8BW[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[a\x01b`\x04\x806\x03`\x80\x81\x10\x15a\x01<WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x015a\x02\xECV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[a\x01b`\x04\x806\x03`\xA0\x81\x10\x15a\x01\xBCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x81\x015\x90`\x80\x015a\x03*V[a\x02U`\x04\x806\x03`\xA0\x81\x10\x15a\x02)WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x81\x015\x90`\x80\x015a\x03jV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02oa\x03\xB1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01b`\x04\x806\x03``\x81\x10\x15a\x02\xCCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x03\xD5V[`\0\x80a\x03\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x87\x87\x87a\x04\x10V[\x91P\x91P[\x94P\x94\x92PPPV[`\0\x80a\x03[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x88\x88a\x04\xD5V[\x91P\x91P[\x95P\x95\x93PPPPV[`\0\x80Z\x90Pa\x03\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x88\x88a\x04\xD5V[PP`\0Z\x90\x91\x03\x97\x96PPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x04\x04\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x86\x86a\x07\xECV[\x91P\x91P\x93P\x93\x91PPV[`\0\x80a\x04\x1E\x87\x87\x87a\n\xB4V[\x90\x92P\x90P\x81\x15\x80\x15\x90a\x042WP`\0\x81\x11[a\x04mW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`-\x81R` \x01\x80a\x12\xCE`-\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x04|\x86\x86\x86\x86a\x0B\xE4V[\x91P\x91P\x80`\0\x14\x15a\x04\x91WPa\x03`\x90PV[\x81\x15a\x04\xB2W`\0a\x04\xA4\x82\x86\x86a\x0C\xCFV[\x94\x82\x01\x94\x90\x93\x03\x92Pa\x04\xC9V[`\0a\x04\xBF\x82\x85\x87a\x0C\xCFV[\x90\x94\x03\x93P\x91\x82\x01\x91[PP\x95P\x95\x93PPPPV[`\0\x80`\0\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16c\x01~~X`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05TWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x05hW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x05\xA9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P`\0a\x05\xC5\x8A\x8A\x8Aa\r\xBFV[\x90P`\0\x82a\x05\xD5W`\0a\x06\x9DV[\x81`\x01`\x01`\xA0\x1B\x03\x16ctd\xFC=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x06YW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x06\x9AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\x11WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x07%W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x07fWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x90P\x86\x81\x10\x80\x15\x90a\x07zWP`\0\x87\x11[a\x07\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80a\x13\x88`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x07\xC5\x8E\x8E\x8E\x8E\x8Ea\x04\x10V[\x91P\x91Pa\x07\xD7\x82\x82\x85\x8C\x8A\x89a\x0E\x7FV[\x97P\x97PPPPPPP\x96P\x96\x94PPPPPV[`\0\x80`\0\x80a\x07\xFD\x88\x88\x88a\n\xB4V[\x91P\x91P`\0a\x08\x0E\x89\x89\x89a\r\xBFV[\x90P`\0\x80`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16c\x01~~X`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\x8CWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x08\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\xE1WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P`\0\x81a\x08\xFEW`\0a\t\xC6V[\x82`\x01`\x01`\xA0\x1B\x03\x16ctd\xFC=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\tnWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\t\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\t\xC3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[\x90P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n:WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\nNW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\n\x8FWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x90Pa\n\xA1\x86\x86\x83\x8C\x87\x87a\x0E\x7FV[\x97P\x97PPPPPPP\x94P\x94\x92PPPV[`\0\x80`\0a\n\xC3\x85\x85a\x0FXV[P\x90P`\0\x80a\n\xD4\x88\x88\x88a\r\xBFV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0BCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x13H\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x0BWW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x0B\x98WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` a\x13h\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14a\x0B\xD2W\x80\x82a\x0B\xD5V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x85a\x0B\xF3\x85\x87\x86a\x106V[\x10\x91P`\0a\x0C\x08\x85\x85c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\x0CZa\x0CUa\x0C$\x84a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x86a\x0C/W\x89a\x0C1V[\x8A[a\x0CPa\x03\xE5\x89a\x0CBW\x8Da\x0CDV[\x8C[\x90c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[a\x106V[a\x11?V[\x90P`\0a\x03\xE5\x85a\x0C}Wa\x0Cx\x87a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[a\x0C\x8FV[a\x0C\x8F\x88a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x81a\x0C\x96W\xFE[\x04\x90P\x80\x82\x10\x15a\x0C\xB1WP`\0\x93P\x83\x92Pa\x03!\x91PPV[a\x0C\xC1\x82\x82c\xFF\xFF\xFF\xFFa\x11\x91\x16V[\x93PPPP\x94P\x94\x92PPPV[`\0\x80\x84\x11a\r\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80a\x13\xAF`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a\r\x1FWP`\0\x82\x11[a\rZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a\x13 `(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a\rn\x85a\x03\xE5c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\r\x82\x82\x85c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\r\xA8\x83a\r\x9C\x88a\x03\xE8c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90c\xFF\xFF\xFF\xFFa\x11\xE1\x16V[\x90P\x80\x82\x81a\r\xB3W\xFE[\x04\x97\x96PPPPPPPV[`\0\x80`\0a\r\xCE\x85\x85a\x0FXV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7Ft\xABbfI\x0E\x96b\x9C\xE0k\x06\xFE\xB6\xF7JSuTc\xF6\x02]1\xC3e\xAD\xB6uP\xF3\xDC`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`\0\x80\x83\x80\x15a\x0E\x8FWP`\0\x83\x11[\x15a\x0F\x17W`\0a\x0E\xA9a\x0CU\x8A\x8Ac\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\x0E\xB6\x85a\x11?V[\x90P\x80\x82\x11\x15a\x0F\x14W\x87`\0a\x0E\xD3\x84\x84c\xFF\xFF\xFF\xFFa\x11\x91\x16V[\x90P`\0a\x0E\xEC\x84a\r\x9C\x87`\x05c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x90P`\0a\x0E\xFB\x84\x84\x84a\x106V[\x90Pa\x0F\r\x8C\x82c\xFF\xFF\xFF\xFFa\x11\xE1\x16V[\x9BPPPPP[PP[\x85a\x0F(\x89\x87c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x81a\x0F/W\xFE[\x04\x86a\x0FA\x89\x88c\xFF\xFF\xFF\xFFa\x10\xD6\x16V[\x81a\x0FHW\xFE[\x04\x91P\x91P\x96P\x96\x94PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0F\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x12\xFB`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0F\xCCW\x82\x84a\x0F\xCFV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10/W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0\x80`\0a\x10E\x86\x86a\x120V[\x91P\x91P`\0\x84\x80a\x10SW\xFE[\x86\x88\t\x90P\x82\x81\x11\x15a\x10gW`\x01\x82\x03\x91P[\x80\x83\x03\x92P\x84\x82\x10a\x10\xC0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FFullMath: FULLDIV_OVERFLOW\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x10\xCB\x83\x83\x87a\x12]V[\x97\x96PPPPPPPV[`\0\x81\x15\x80a\x10\xF1WPP\x80\x82\x02\x82\x82\x82\x81a\x10\xEEW\xFE[\x04\x14[a\x119W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92\x91PPV[`\0`\x03\x82\x11\x15a\x11\x82WP\x80`\x01`\x02\x82\x04\x01[\x81\x81\x10\x15a\x11|W\x80\x91P`\x02\x81\x82\x85\x81a\x11kW\xFE[\x04\x01\x81a\x11tW\xFE[\x04\x90Pa\x11TV[Pa\x11\x8CV[\x81\x15a\x11\x8CWP`\x01[\x91\x90PV[\x80\x82\x03\x82\x81\x11\x15a\x119W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x01\x82\x81\x10\x15a\x119W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x80`\0\x19\x84\x86\t\x90P\x83\x85\x02\x92P\x82\x81\x03\x91P\x82\x81\x10\x15a\x12UW`\x01\x82\x03\x91P[P\x92P\x92\x90PV[`\0\x81\x81\x03\x82\x16\x80\x83\x81a\x12mW\xFE[\x04\x92P\x80\x85\x81a\x12yW\xFE[\x04\x94P\x80\x81`\0\x03\x81a\x12\x88W\xFE[`\x02\x85\x81\x03\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x80\x87\x02\x82\x03\x02\x95\x86\x02\x90\x03\x90\x94\x02\x93\x04`\x01\x01\x93\x90\x93\x02\x93\x90\x93\x01\x02\x92\x91PPV\xFEUniswapV2ArbitrageLibrary: ZERO_PAIR_RESERVESUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Library: INSUFFICIENT_LIQUIDITYTarget contract does not containCalldata too short\0\0\0\0\0\0\0\0\0\0\0\0\0\0ComputeLiquidityValue: LIQUIDITY_AMOUNTUniswapV2Library: INSUFFICIENT_INPUT_AMOUNT\xA2dipfsX\"\x12 \xDF\xC6`\xD5\xBD\x06d\xB8dJ\xA7ug\x95\xF5L\x13WmG\xCAl\x06{/\xB5l\x99jH\xC3\x1AdsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static EXAMPLECOMPUTELIQUIDITYVALUE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ExampleComputeLiquidityValue<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExampleComputeLiquidityValue<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExampleComputeLiquidityValue<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExampleComputeLiquidityValue<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExampleComputeLiquidityValue<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExampleComputeLiquidityValue))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExampleComputeLiquidityValue<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EXAMPLECOMPUTELIQUIDITYVALUE_ABI.clone(),
                client,
            ))
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
                EXAMPLECOMPUTELIQUIDITYVALUE_ABI.clone(),
                EXAMPLECOMPUTELIQUIDITYVALUE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGasCostOfGetLiquidityValueAfterArbitrageToPrice` (0x80caa353) function
        pub fn get_gas_cost_of_get_liquidity_value_after_arbitrage_to_price(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            true_price_token_a: ::ethers::core::types::U256,
            true_price_token_b: ::ethers::core::types::U256,
            liquidity_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [128, 202, 163, 83],
                    (
                        token_a,
                        token_b,
                        true_price_token_a,
                        true_price_token_b,
                        liquidity_amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityValue` (0xd9b7a6e7) function
        pub fn get_liquidity_value(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            liquidity_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([217, 183, 166, 231], (token_a, token_b, liquidity_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityValueAfterArbitrageToPrice` (0x69a2fcbd) function
        pub fn get_liquidity_value_after_arbitrage_to_price(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            true_price_token_a: ::ethers::core::types::U256,
            true_price_token_b: ::ethers::core::types::U256,
            liquidity_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [105, 162, 252, 189],
                    (
                        token_a,
                        token_b,
                        true_price_token_a,
                        true_price_token_b,
                        liquidity_amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAfterArbitrage` (0x3558e94c) function
        pub fn get_reserves_after_arbitrage(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            true_price_token_a: ::ethers::core::types::U256,
            true_price_token_b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [53, 88, 233, 76],
                    (token_a, token_b, true_price_token_a, true_price_token_b),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ExampleComputeLiquidityValue<M>
    {
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
        Hash,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `getGasCostOfGetLiquidityValueAfterArbitrageToPrice` function with signature `getGasCostOfGetLiquidityValueAfterArbitrageToPrice(address,address,uint256,uint256,uint256)` and selector `0x80caa353`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getGasCostOfGetLiquidityValueAfterArbitrageToPrice",
        abi = "getGasCostOfGetLiquidityValueAfterArbitrageToPrice(address,address,uint256,uint256,uint256)"
    )]
    pub struct GetGasCostOfGetLiquidityValueAfterArbitrageToPriceCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub true_price_token_a: ::ethers::core::types::U256,
        pub true_price_token_b: ::ethers::core::types::U256,
        pub liquidity_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLiquidityValue` function with signature `getLiquidityValue(address,address,uint256)` and selector `0xd9b7a6e7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getLiquidityValue",
        abi = "getLiquidityValue(address,address,uint256)"
    )]
    pub struct GetLiquidityValueCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub liquidity_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLiquidityValueAfterArbitrageToPrice` function with signature `getLiquidityValueAfterArbitrageToPrice(address,address,uint256,uint256,uint256)` and selector `0x69a2fcbd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getLiquidityValueAfterArbitrageToPrice",
        abi = "getLiquidityValueAfterArbitrageToPrice(address,address,uint256,uint256,uint256)"
    )]
    pub struct GetLiquidityValueAfterArbitrageToPriceCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub true_price_token_a: ::ethers::core::types::U256,
        pub true_price_token_b: ::ethers::core::types::U256,
        pub liquidity_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getReservesAfterArbitrage` function with signature `getReservesAfterArbitrage(address,address,uint256,uint256)` and selector `0x3558e94c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getReservesAfterArbitrage",
        abi = "getReservesAfterArbitrage(address,address,uint256,uint256)"
    )]
    pub struct GetReservesAfterArbitrageCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub true_price_token_a: ::ethers::core::types::U256,
        pub true_price_token_b: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExampleComputeLiquidityValueCalls {
        Factory(FactoryCall),
        GetGasCostOfGetLiquidityValueAfterArbitrageToPrice(
            GetGasCostOfGetLiquidityValueAfterArbitrageToPriceCall,
        ),
        GetLiquidityValue(GetLiquidityValueCall),
        GetLiquidityValueAfterArbitrageToPrice(GetLiquidityValueAfterArbitrageToPriceCall),
        GetReservesAfterArbitrage(GetReservesAfterArbitrageCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleComputeLiquidityValueCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded)
                = <GetGasCostOfGetLiquidityValueAfterArbitrageToPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::GetGasCostOfGetLiquidityValueAfterArbitrageToPrice(decoded),
                );
            }
            if let Ok(decoded) =
                <GetLiquidityValueCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidityValue(decoded));
            }
            if let Ok(decoded)
                = <GetLiquidityValueAfterArbitrageToPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLiquidityValueAfterArbitrageToPrice(decoded));
            }
            if let Ok(decoded) =
                <GetReservesAfterArbitrageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetReservesAfterArbitrage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleComputeLiquidityValueCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetGasCostOfGetLiquidityValueAfterArbitrageToPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidityValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLiquidityValueAfterArbitrageToPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesAfterArbitrage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExampleComputeLiquidityValueCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGasCostOfGetLiquidityValueAfterArbitrageToPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidityValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidityValueAfterArbitrageToPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReservesAfterArbitrage(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FactoryCall> for ExampleComputeLiquidityValueCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetGasCostOfGetLiquidityValueAfterArbitrageToPriceCall>
        for ExampleComputeLiquidityValueCalls
    {
        fn from(value: GetGasCostOfGetLiquidityValueAfterArbitrageToPriceCall) -> Self {
            Self::GetGasCostOfGetLiquidityValueAfterArbitrageToPrice(value)
        }
    }
    impl ::core::convert::From<GetLiquidityValueCall> for ExampleComputeLiquidityValueCalls {
        fn from(value: GetLiquidityValueCall) -> Self {
            Self::GetLiquidityValue(value)
        }
    }
    impl ::core::convert::From<GetLiquidityValueAfterArbitrageToPriceCall>
        for ExampleComputeLiquidityValueCalls
    {
        fn from(value: GetLiquidityValueAfterArbitrageToPriceCall) -> Self {
            Self::GetLiquidityValueAfterArbitrageToPrice(value)
        }
    }
    impl ::core::convert::From<GetReservesAfterArbitrageCall> for ExampleComputeLiquidityValueCalls {
        fn from(value: GetReservesAfterArbitrageCall) -> Self {
            Self::GetReservesAfterArbitrage(value)
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
        Hash,
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getGasCostOfGetLiquidityValueAfterArbitrageToPrice` function with signature `getGasCostOfGetLiquidityValueAfterArbitrageToPrice(address,address,uint256,uint256,uint256)` and selector `0x80caa353`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetGasCostOfGetLiquidityValueAfterArbitrageToPriceReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getLiquidityValue` function with signature `getLiquidityValue(address,address,uint256)` and selector `0xd9b7a6e7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLiquidityValueReturn {
        pub token_a_amount: ::ethers::core::types::U256,
        pub token_b_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getLiquidityValueAfterArbitrageToPrice` function with signature `getLiquidityValueAfterArbitrageToPrice(address,address,uint256,uint256,uint256)` and selector `0x69a2fcbd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLiquidityValueAfterArbitrageToPriceReturn {
        pub token_a_amount: ::ethers::core::types::U256,
        pub token_b_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReservesAfterArbitrage` function with signature `getReservesAfterArbitrage(address,address,uint256,uint256)` and selector `0x3558e94c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetReservesAfterArbitrageReturn {
        pub reserve_a: ::ethers::core::types::U256,
        pub reserve_b: ::ethers::core::types::U256,
    }
}
