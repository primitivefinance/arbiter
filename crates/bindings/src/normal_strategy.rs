pub use normal_strategy::*;
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
pub mod normal_strategy {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("portfolio_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("afterCreate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("afterCreate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("approximateReservesGivenPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "approximateReservesGivenPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("beforeSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beforeSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("configs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("configs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("creationTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("output"),
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
                    ::std::borrow::ToOwned::to_owned("getFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInvariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInvariant"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("getStrategyData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStrategyData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
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
                                    name: ::std::borrow::ToOwned::to_owned("strategyData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialY"),
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
                    ::std::borrow::ToOwned::to_owned("getSwapInvariants"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapInvariants"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("portfolio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("portfolio"),
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
                    ::std::borrow::ToOwned::to_owned("verifyPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifyPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifySwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CurveLib_ConfigExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CurveLib_ConfigExists",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CurveLib_InvalidDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CurveLib_InvalidDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CurveLib_InvalidStrikePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CurveLib_InvalidStrikePrice",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CurveLib_InvalidVolatility"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CurveLib_InvalidVolatility",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CurveLib_NonExpiringPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CurveLib_NonExpiringPool",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("InvalidBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidBounds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("NotInsideBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotInsideBounds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_FeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SwapLib_FeeTooHigh"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_OutputExceedsReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_OutputExceedsReserves",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ProtocolFeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ProtocolFeeTooHigh",
                            ),
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
    pub static NORMALSTRATEGY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0A\xD38\x03\x80b\0A\xD3\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\x93V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\x01\x10V[`\0` \x82\x84\x03\x12\x15b\0\0\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\tW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa@Sb\0\x01\x80`\09`\0\x81\x81a\x02\x03\x01R\x81\x81a\x04`\x01R\x81\x81a\x06\r\x01R\x81\x81a\x06\xF1\x01R\x81\x81a\x08\x9B\x01R\x81\x81a\t\xD2\x01R\x81\x81a\x0Bc\x01R\x81\x81a\x0FH\x01R\x81\x81a\x10\x96\x01R\x81\x81a\x12U\x01R\x81\x81a\x13\xF2\x01R\x81\x81a\x14\xE6\x01Ra\x16Y\x01Ra@S`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80cE-/\x18\x11a\0\xD9W\x80c\xE0hx\x7F\x11a\0\xB3W\x80c\xE0hx\x7F\x14a\x03\xD5W\x80c\xE31\xBA4\x14a\x03\xEAW\x80c\xEA\x82\0Y\x14a\x03\xFDW\x80c\xECshT\x14a\x04 Wa\x017V[\x80cE-/\x18\x14a\x03xW\x80cE2\x0C\x8C\x14a\x03\x9AW\x80cT'g\xFC\x14a\x03\xC2Wa\x017V[\x80c\x19\x05x\x07\x11a\x01\x15W\x80c\x19\x05x\x07\x14a\x02=W\x80c4\xDB\xC7;\x14a\x02^W\x80c9CMZ\x14a\x02\xF6W\x80cD\x1F\xA75\x14a\x03\tWa\x017V[\x80c\x08\xE6\xCC\xA3\x14a\x01\x9CW\x80c\x0C\x8A\x11?\x14a\x01\xD4W\x80c\x16\xED\xE0\x16\x14a\x01\xFEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xB4a\x01\xAA6`\x04a4bV[P`\0\x90\x81\x90\x81\x90V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE7a\x01\xE26`\x04a4\x80V[a\x043V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xCBV[a\x02%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xCBV[a\x02Pa\x02K6`\x04a4\xE2V[a\x05\xE2V[`@Q\x90\x81R` \x01a\x01\xCBV[a\x02\xB6a\x02l6`\x04a4bV[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\x80\x1B\x03\x81\x16\x90c\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x91`\x01`\xA0\x1B\x81\x04\x82\x16\x91`\x01`\xC0\x1B\x82\x04\x16\x90`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x85V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x96\x16\x86Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x91\x90\x91\x16``\x83\x01R\x15\x15`\x80\x82\x01R`\xA0\x01a\x01\xCBV[a\x02Pa\x03\x046`\x04a4bV[a\t\xA7V[a\x03\x1Ca\x03\x176`\x04a56V[a\x0B\x17V[`@Qa\x01\xCB\x91\x90`\0`\xA0\x82\x01\x90P`\x01`\x01`\x80\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q\x15\x15`@\x83\x01R`\x01`\x01`@\x1B\x03``\x84\x01Q\x16``\x83\x01R`\x80\x83\x01Q\x15\x15`\x80\x83\x01R\x92\x91PPV[a\x03\x8Ba\x03\x866`\x04a5pV[a\x0E-V[`@Qa\x01\xCB\x93\x92\x91\x90a5\xBCV[a\x03\xADa\x03\xA86`\x04a6\xECV[a\x0F\x15V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xCBV[a\x03\xADa\x03\xD06`\x04a7\xC0V[a\x11nV[a\x03\xE8a\x03\xE36`\x04a8\xADV[a\x11\xAFV[\0[a\x02Pa\x03\xF86`\x04a4bV[a\x12*V[a\x04\x10a\x04\x0B6`\x04a4bV[a\x13\xC5V[`@Q\x90\x15\x15\x81R` \x01a\x01\xCBV[a\x01\xE7a\x04.6`\x04a9\xE2V[a\x14\xB9V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x18\x91\x90a:]V[\x90Pa\x05#\x85a\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x16\x81Ra\x057\x84a\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x83\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R\x80\x82R`@\x80\x82 \x81Q`\xA0\x81\x01\x83R\x90T\x94\x85\x16\x81R`\x01`\x80\x1B\x85\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x82\x01\x94\x90\x94R`\x01`\xA0\x1B\x85\x04\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x84\x04\x83\x16``\x82\x01R`\x01`\xE0\x1B\x90\x93\x04`\xFF\x16\x15\x15`\x80\x84\x01R\x91a\x05\xC5\x91\x84\x91\x90a\x17\xE5\x16V[\x90P`\0a\x05\xD3\x88\x83a\x18\xC4V[\x99\x91\x98P\x90\x96PPPPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC5\x91\x90a:]V[`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF`(\x89\x90\x1C\x16`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^Gf<\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA9\x91\x90a;(V[\x90Pa\x07\xCD\x86a\x07\xBDW\x81``\x01Qa\x07\xC3V[\x81` \x01Q[\x86\x90`\xFF\x16a\x18\xF0V[`\x01`\x01`@\x1B\x03\x88\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x80\x82\x01\x85R\x91T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x82\x86\x01R`\x01`\xC0\x1B\x81\x04\x90\x93\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x82Q\x90\x81\x01\x90\x92R\x91\x96Pa\ts\x91\x90\x80a\x08\\\x89a\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01\x8A`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x89\x15\x15\x81RPB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB0\xE2\x1E\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\tDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\th\x91\x90a;\xABV[\x86\x93\x92\x91\x90\x89a\x19\x07V[\x92P`\0\x86a\t\x86W\x81` \x01Qa\t\x8CV[\x81``\x01Q[`\xFF\x16\x90Pa\t\x9B\x84\x82a\x1A\x81V[\x98\x97PPPPPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\nRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\nfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x8A\x91\x90a:]V[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x82\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x91\x92Pa\x0B\x10\x91\x83\x91\x90a\x17\xE5\x16V[\x93\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C#\x91\x90a:]V[`\x01`\x01`@\x1B\x03\x85\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x90\x92\x16``\x83\x01R`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x15\x15`\x80\x82\x01R\x91\x92Pa\x0C\xA4\x82a\x1A\x97V[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\x0C\xC5\x85`@\x01Q\x90`\0\x90V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x8D\x16``\x82\x01R\x8B\x15\x80\x15`\x80\x83\x01R\x92\x94P\x90\x92P\x90a\r\x97W`\x01\x88`\0\x01Qa\r4a\r/\x8B`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x89a\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x17\xCFV[a\r>\x91\x90a;\xDDV[a\rH\x91\x90a;\xDDV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x82R`@\x89\x01Qa\rj\x91a\r/\x91\x85\x91\x16a\x1B=V[\x88` \x01Qa\ry\x91\x90a;\xDDV[a\r\x84\x90`\x01a;\xFDV[`\x01`\x01`\x80\x1B\x03\x16` \x82\x01Ra\x0E\x1DV[`\x01\x88` \x01Qa\r\xC1a\r/\x8B`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x87a\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xCB\x91\x90a;\xDDV[a\r\xD5\x91\x90a;\xDDV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x82R`@\x89\x01Qa\r\xF7\x91a\r/\x91\x87\x91\x16a\x1B=V[\x88Qa\x0E\x03\x91\x90a;\xDDV[a\x0E\x0E\x90`\x01a;\xFDV[`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R[\x97PPPPPPPP[\x92\x91PPV[```\0\x80`\0`@Q\x80`\xA0\x01`@R\x80a\x0EH\x8Ba\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x0E_\x8Aa\x1BRV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x0Es\x89a\x1BRV[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R`\0` \x80\x84\x01\x91\x90\x91R\x89\x15\x15`@\x93\x84\x01R\x82Q\x84Q`\x01`\x01`\x80\x1B\x03\x16\x81\x83\x01R\x90\x84\x01Q\x82\x16\x81\x84\x01R\x91\x83\x01Q\x81\x16``\x80\x84\x01\x91\x90\x91R\x83\x01Q\x16`\x80\x80\x83\x01\x91\x90\x91R\x82\x01Q\x15\x15`\xA0\x82\x01R`\xC0\x81\x01\x87\x90R\x90\x91P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93Pa\x0F\x03\x85a\x0E\xFD\x83a\x1A\x97V[\x90a\x1BeV[\x94\x9A\x90\x99P\x93\x97P\x92\x95PPPPPPV[``\x81\x01Q`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\0\x91\x90a:]V[``\x85\x81\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x83\x86\x01R`\x01`\xA0\x1B\x82\x04\x81\x16\x83\x85\x01R`\x01`\xC0\x1B\x82\x04\x16\x95\x82\x01\x95\x90\x95R`\x01`\xE0\x1B\x90\x94\x04`\xFF\x16\x15\x15`\x80\x85\x01R\x80QcXq\x0FE`\xE1\x1B\x81R\x90Q\x94\x95P\x91\x93\x84\x93a\x11`\x93\x90\x92\x8A\x92B\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92c\xB0\xE2\x1E\x8A\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x11\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x111W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11U\x91\x90a;\xABV[\x87\x93\x92\x91\x903a\x1B\x89V[\x90\x98\x90\x97P\x95PPPPPPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x11\x88\x91\x90a<\x1DV[\x91P\x91P`\0a\x11\x97\x83a\x1A\x97V[\x90Pa\x11\xA3\x81\x83a\x1BeV[\x94P\x94PPPP\x91P\x91V[`\0\x80a\x11\xBE\x83\x85\x01\x85a<\xB7V[\x91P\x91Pa\x12#\x82`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x84`@\x01Qc\xFF\xFF\xFF\xFF\x16\x85`\x80\x01Q`\0\x80\x8B`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 a\x1D\xB6\x90\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\r\x91\x90a:]V[\x90Pa\x0B\x10a\x13?\x82`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`@\x1B\x03\x85\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x90\x91\x16``\x82\x01R`\x01`\xE0\x1B\x90\x91\x04`\xFF\x16\x15\x15`\x80\x82\x01Ra\x13\xBF\x90a\x1A\x97V[\x90a\x1F\x96V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90a\x0E'\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xAA\x91\x90a:]V[``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9E\x91\x90a:]V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x80\x82\x01\x84R\x91T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x83\x87\x01R`\x01`\xA0\x1B\x82\x04\x81\x16\x83\x86\x01R`\x01`\xC0\x1B\x82\x04\x16``\x80\x84\x01\x91\x90\x91R`\x01`\xE0\x1B\x90\x91\x04`\xFF\x16\x15\x15`\x80\x80\x84\x01\x91\x90\x91R\x84Q\x93\x84\x01\x85R\x86\x84R\x83\x86\x01\x87\x90R\x83\x85\x01\x87\x90R\x90\x83\x01\x96\x90\x96R\x8B\x15\x15\x95\x82\x01\x95\x90\x95R\x81QcXq\x0FE`\xE1\x1B\x81R\x91Q\x95\x96P\x92\x94a\x17&\x94\x93\x92B\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92c\xB0\xE2\x1E\x8A\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x16\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x1B\x91\x90a;\xABV[\x86\x93\x92\x91\x90\x8Aa\x1B\x89V[P`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x82\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x91\x93Pa\x17\xAD\x92P\x84\x91a \xC4\x16V[\x15a\x17\xBFW`\0\x93P\x91Pa\x17\xC7\x90PV[`\x01\x93P\x91PP[\x93P\x93\x91PPV[`\0`\x01`\x80\x1B\x82\x10a\x17\xE1W`\0\x80\xFD[P\x90V[`\0\x80a\x18\x15\x84`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18F\x85`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x86` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0`@Q\x80`\xC0\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x86`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x18\x96\x87` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x18\xA5\x88\x88a \xF6V[\x81R` \x01`\0\x81RP\x90Pa\x18\xBA\x81a!\rV[\x96\x95PPPPPPV[`\0\x80a\x18\xD1\x84\x84a=OV[\x90P`\x01\x81\x12\x15a\x18\xE6W`\0\x91PPa\x0E'V[P`\x01\x93\x92PPPV[`\0\x80a\x18\xFC\x83a\"\x14V[\x93\x90\x93\x02\x93\x92PPPV[`\0\x80`\0\x80a\x19\x1B\x8A\x8A\x8A\x8A\x8A\x8Aa\x1B\x89V[\x92P\x92P\x92P`\0a\x19,\x8Aa\x1A\x97V[`\xA0\x81\x01\x84\x90R`\x80\x8A\x01Q\x90\x91P`\0\x90\x80\x15a\x19^W\x85\x83Ra\x19P\x83a\",V[` \x84\x01\x81\x90R\x91Pa\x19tV[` \x83\x01\x86\x90Ra\x19n\x83a\"\xEDV[\x80\x84R\x91P[`\0a\x19\x83\x83`b`da#\xAEV[\x90P`\0a\x19\x94\x84`f`da#\xCDV[\x90P\x83\x98Pa\x1A\x17\x85`@Q` \x01a\x19\xEC\x91\x90`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\0a\x01\0\x88a\x1A\x0FWa#\xFBa$tV[a$Ca$tV[\x98P\x88a\x1A#\x81a=oV[\x99PPa\x1AF\x8F`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x8Aa\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x98P\x88\x83a\x1AUW\x8FQa\x1A[V[\x8F` \x01Q[`\x01`\x01`\x80\x1B\x03\x16a\x1An\x91\x90a=\x88V[\x9F\x9EPPPPPPPPPPPPPPPV[`\0\x80a\x1A\x8D\x83a\"\x14V[\x90\x93\x04\x93\x92PPPV[a\x1A\xD0`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01\x83`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x1B\x1E\x84` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R`@\x93\x84\x01Qc\xFF\xFF\xFF\xFF\x16` \x82\x01R`\0\x93\x01\x92\x90\x92RP\x90V[`\0a\x0B\x10\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xAEV[`\0d\x01\0\0\0\0\x82\x10a\x17\xE1W`\0\x80\xFD[`\0\x80a\x1Br\x84\x84a%\x8AV[\x80\x85R\x91Pa\x1B\x80\x84a\",V[\x90P\x92P\x92\x90PV[`\0\x80`\0a\x1B\xC7`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x89Q` \x8B\x01Q`\x80\x8A\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90\x15a\x1C*W`@\x8C\x01Qa\x1C\x01\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81V[\x91Pa\x1C#\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a&l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x1ChV[`@\x8C\x01Qa\x1CC\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a&lV[\x91Pa\x1Ce\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P[`@Q\x80`\xC0\x01`@R\x80\x83\x81R` \x01\x82\x81R` \x01\x8C`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x1C\xB4\x8D` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x1C\xC4\x8E\x8E\x8Da&\x81V[\x81R` \x01`\0\x81RP\x92PPPa\x1C\xDB\x81a!\rV[\x8AQ` \x8C\x01Q`\xC0\x8D\x01Q\x92\x95P`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x16\x90\x8A\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x16\x14a\x1D\x1AW\x8D`\x80\x01Qa\x1D V[\x8D`\xA0\x01Q[a\xFF\xFF\x16\x90Pa\x1D3\x82\x85\x85\x84\x8Ea&\xBFV[\x90\x91\x92P\x90\x91P\x80\x94P\x81\x95PPPPPa\x1Dd\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83R`@\x8C\x01Qa\x1D\x7F\x90\x82\x90`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81V[` \x84\x01Ra\x1D\x8D\x83a!\rV[\x93P\x89`\x80\x01Qa\x1D\xA2W\x82` \x01Qa\x1D\xA5V[\x82Q[\x95PPPP\x96P\x96P\x96\x93PPPPV[\x84T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x1D\xE3W`@Qc\x1E\xC0\xFD\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1E0W\x84T`\xFF`\xE0\x1B\x19\x16`\x01`\xE0\x1B\x82\x15\x15\x02\x17\x85Ua\x1E\x0Bc\x01\xE1\x85Ya\x1BRV[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85Ua\x1E\xA2V[a\x1E[b\x01Q\x80a\x1EFc\x01\xE1\x85Y`\x03a=\x9BV[\x80\x85\x10\x90\x85\x14\x17\x81\x85\x11\x91\x85\x14\x91\x90\x91\x17\x16\x90V[a\x1ExW`@Qc4\xB8\x03\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x81\x82a\x1BRV[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85U[aa\xA8\x80\x84\x10\x90\x84\x14\x17`\x01\x80\x85\x11\x90\x85\x14\x17\x16a\x1E\xD3W`@Qc\xEC\xDC\x82\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xDC\x83a\x1BRV[\x85Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1Bc\xFF\xFF\xFF\xFF\x92\x83\x16\x02\x17\x86Ua\x1F\x11\x90\x85\x90`\x01\x90`\x01`\x01`\x80\x1B\x03\x90a&\xF3\x16V[a\x1F.W`@Qc\"\xFEks`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F7\x84a\x17\xCFV[\x85T`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`\x01`\xC0\x1BBc\xFF\xFF\xFF\xFF\x16\x02\x17\x90\x94UPPPPV[`\0a\x0B\x10\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xAEV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\x1F\xB6\x87`@\x01Q\x90`\0\x90V[\x91P\x91P\x83\x86\x10a\x1F\xD1W\x86`@\x01Q\x94PPPPPa\x0E'V[\x82\x86\x11a\x1F\xECWP`\x01`\x01`\x80\x1B\x03\x93Pa\x0E'\x92PPPV[`\x80\x87\x01Q`\0\x90a \x02\x90c\x01\xE1\x85Ya\x1F\x81V[\x90P`\0a \x0F\x89a'\tV[\x90P`\0a -a (\x8Ag\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[a'gV[\x90P`\0a ;\x83\x83a=\xB2V[\x90P`\0g\x1B\xC1mgN\xC8\0\0\x85\x8D``\x01Q\x8E``\x01Qa ]\x91\x90a=\x9BV[a g\x91\x90a=\x9BV[a q\x91\x90a=\xF8V[\x90P`\0a \x9Ag\r\xE0\xB6\xB3\xA7d\0\0a \x8B\x84\x86a=OV[a \x95\x91\x90a>\x0CV[a(\x04V[\x90Pa \xB3\x8D`@\x01Q\x82a\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9D\x9CPPPPPPPPPPPPPV[`\0\x81`\x80\x01Q\x80a\x0B\x10WPa \xDB\x83\x83a)\xADV[c\xFF\xFF\xFF\xFF\x16\x83``\x01Qc\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x92\x91PPV[`\0a\x0B\x10\x83\x83\x85``\x01Qc\xFF\xFF\xFF\xFF\x16a&\x81V[`\0\x80a!\x19\x83a'\tV[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a!:\x87`@\x01Q\x90`\0\x90V[\x90\x92P\x90P`\0a!ta!O\x85`\x01a>:V[a!Z`\x01\x88a=\x88V[\x8AQ\x91\x90\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[\x15a!\x94W\x87Qa!\x91\x90a (\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[\x90P[`\0a!\xCCa!\xA4\x84`\x01a>:V[a!\xAF`\x01\x87a=\x88V[` \x8C\x01Q\x91\x90\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[\x15a!\xF2Wa!\xEFa (\x8A`@\x01Q\x8B` \x01Qa&l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P[\x86a!\xFD\x83\x83a=OV[a\"\x07\x91\x90a>MV[\x99\x98PPPPPPPPPV[`\0a\"!\x82`\x12a=\x88V[a\x0E'\x90`\na?YV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\"L\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x83\x86`\0\x01Q\x10a\"dW\x95\x94PPPPPV[\x85Q\x83\x10a\"uWP\x94\x93PPPPV[`\0a\"\x80\x87a'\tV[\x87Q\x90\x91P`\0\x90a\"\x9A\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[\x90P`\0a\"\xA7\x82a'gV[\x90P`\0\x89`\xA0\x01Q\x84\x83a\"\xBC\x91\x90a=OV[a\"\xC6\x91\x90a>MV[\x90Pa\"\xDF\x8A`@\x01Qa\"\xD9\x83a)\xE4V[\x90a\x1B=V[\x9A\x99PPPPPPPPPPV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a#\r\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x81\x86` \x01Q\x10a#&WP\x90\x94\x93PPPPV[\x80\x86` \x01Q\x11a#;WP\x91\x94\x93PPPPV[`\0a#F\x87a'\tV[\x90P`\0a#e\x88`@\x01Q\x89` \x01Qa&l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#r\x82a'gV[\x90P`\0\x89`\xA0\x01Q\x84\x83a#\x87\x91\x90a>MV[a#\x91\x91\x90a=OV[\x90Pa#\x9C\x81a)\xE4V[a\"\xDF\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xC6W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xE5W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a$\x12\x91\x90a?eV[\x83\x81R`\xA0\x81\x01Q\x90\x91Pa$(\x90`\x01a>MV[a$1\x82a!\rV[a$;\x91\x90a=OV[\x94\x93PPPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a$Z\x91\x90a?eV[` \x81\x01\x84\x90R`\xA0\x81\x01Q\x90\x91Pa$(\x90`\x01a>MV[`\0\x84\x86\x11\x15a$\xA6W`@Qcr\x17\r\xED`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a$\xB6\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$\xC8\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$\xD6\x82\x84a=\xB2V[\x13\x15a$\xFFW`@Qc#\x84\x17\xCB`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a$\x9DV[`\0a%\x0B\x89\x89a=\x88V[\x90P`\0[`\x02a%\x1C\x8A\x8Ca>:V[a%&\x91\x90a=\xF8V[\x94P`\0a%8\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a%F\x86\x83a=\xB2V[\x13a%SW\x85\x99Pa%ZV[\x85\x9AP\x80\x94P[a%d\x8B\x8Ba=\x88V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a%xWP\x86\x81\x10[a%\x10WPPPP\x96\x95PPPPPPV[`\0\x80a%\xA4\x84`@\x01Q\x84a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x15a&eW`\0a%\xB7\x82a*MV[\x90P`\0a%\xD6c\x01\xE1\x85Y\x87`\x80\x01Qa\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01Q\x90\x91P`\0\x90g\x1B\xC1mgN\xC8\0\0\x90a%\xF5\x90\x80a=\x9BV[a%\xFF\x91\x90a=\xF8V[\x90P`\0a&\x0C\x88a'\tV[\x90P`\0a&\"g\r\xE0\xB6\xB3\xA7d\0\0\x86a=\xB2V[\x90Pa&.\x84\x84a=\xB2V[a&8\x90\x82a>MV[\x90Pa&D\x82\x82a>\x0CV[\x90P`\0a&Q\x82a)\xE4V[\x90Pa\x0E\x1D\x81g\r\xE0\xB6\xB3\xA7d\0\0a=OV[P\x92\x91PPV[`\0a\x0B\x10\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xCDV[`\0\x82`\x80\x01Q\x15a&\x98WPc\x01\xE1\x85Ya\x0B\x10V[`\0a&\xA4\x85\x85a)\xADV[c\xFF\xFF\xFF\xFF\x16\x80\x84\x03\x90\x84\x10\x02\x83\x03\x92\x90\x92\x03\x94\x93PPPPV[`\0\x80\x80\x80a&\xCF\x89\x87\x87a,(V[\x90\x94P\x92Pa&\xE1\x89\x89\x89\x87\x87a,mV[\x94\x9A\x93\x99P\x97P\x92\x95P\x90\x93PPPPV[\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[`\0\x80a''c\x01\xE1\x85Y\x84`\x80\x01Qa\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0c;\x9A\xCA\0a'9\x83a-\x86V[a'C\x91\x90a=\x9BV[\x90P`\0a'^\x82\x86``\x01Qa\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95\x94PPPPPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a'\x80WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a'\xA8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a'\xC9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\xD6\x83`\x02a=\xB2V[\x90P`\0a'\xE3\x82a.*V[\x90P`\0a'\xF9g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a0\xA8V[\x90Pa'^\x81a?\xE1V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a(\x1FWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a$\x9DV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x81`\x80\x01Q\x15a)\xD2W`@Qc5\x7F\xF5\x0B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x81\x01Q``\x82\x01Q\x01\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a*\x02g\r\xE0\xB6\xB3\xA7d\0\0\x85a=\xB2V[a*\x0C\x91\x90a>\x0CV[\x90P`\0a*\x19\x82a?\xE1V[\x90P`\0a*&\x82a0\xBDV[\x90Pg\x1B\xC1mgN\xC8\0\0a*Cg\r\xE0\xB6\xB3\xA7d\0\0\x83a=\xB2V[a'^\x91\x90a>\x0CV[`\0\x80\x82\x13a*\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a$\x9DV[`\0``a*\x97\x84a2\xA1V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82Q`\0\x90\x81\x90a,E\x90`\x01`\x01`\x80\x1B\x03\x16\x85a'\x10a#\xCDV[\x91P\x82\x15a\x17\xC7Wa,W\x83\x83a=\xF8V[\x90Pa,c\x81\x83a=\x88V[\x91P\x93P\x93\x91PPV[`\0\x80`\0\x87`\x80\x01Qa,\x81W\x85a,\x83V[\x86[\x90P`\0\x88`\x80\x01Qa,\x96W\x87a,\x98V[\x86[\x89Q\x90\x91Pa,\xB0\x90`\x01`\x01`\x80\x1B\x03\x16\x83a>:V[\x91P\x81\x86\x11\x15a,\xD3W`@Qcp\r~\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\xDD\x86\x83a=\x88V[\x91P\x81\x85\x11\x15a-\0W`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\n\x85\x83a=\x88V[\x91P\x80\x89` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15a-:W`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x89\x01Qa-R\x90`\x01`\x01`\x80\x1B\x03\x16\x82a=\x88V[\x90P\x88`\x80\x01Qa-cW\x80a-eV[\x81[\x93P\x88`\x80\x01Qa-vW\x81a-xV[\x80[\x92PPP\x95P\x95\x93PPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a-\x9FW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a-\xBBW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a-\xD3W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-\xE9W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x12\x80a.AWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a._W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a.\x80W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a.\xA8W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a.\xB3W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.\xDBWa.\xD6\x83g\x1B\xC1mgN\xC8\0\0a=OV[a.\xDDV[\x82[\x90P`\0a.\xF3\x82g\x1B\xC1mgN\xC8\0\0a3?V[\x90P\x80`\0\x03a/\x16W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a/!\x82a*MV[\x90P`\0c;\x9A\xCA\0a/La/Ga/Ag\x1B\xC1mgN\xC8\0\0a?\xE1V[\x85a0\xA8V[a-\x86V[a/V\x91\x90a=\xB2V[\x90P`\0\x80a/m\x83g\x03\xC1f\\z\xAB \0a0\xA8V[a/\x7F\x90g \x05\xFEO&\x8E\xA0\0a>MV[\x90P`\0a/\xAF\x84a/\x98\x86f\x9F2u$b\xA0\0a0\xA8V[a/\xAA\x90g\r\xC5R\x7Fd, \0a>MV[a0\xA8V[a/\xC1\x90g\r\xE0\xB6\xB3\xA7d\0\0a>MV[\x90Pa/\xE5g\t\xD0(\xCCo _\xFF\x19\x85a/\xDB\x85\x85a3?V[a/\xAA\x91\x90a=OV[\x92PPP`\0[`\x02\x81\x10\x15a0\x80W`\0\x86a0\x01\x84a0\xBDV[a0\x0B\x91\x90a=OV[\x90P`\0a0\x19\x84\x85a0\xA8V[a0\"\x90a?\xE1V[\x90P`\0a0/\x82a(\x04V[\x90P`\0a0=\x86\x85a0\xA8V[a0Og\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a0\xA8V[a0Y\x91\x90a=OV[\x90Pa0e\x84\x82a3?V[a0o\x90\x87a>MV[\x95P\x84`\x01\x01\x94PPPPPa/\xECV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a0\x9DWa0\x98\x82a?\xE1V[a\t\x9BV[P\x96\x95PPPPPPV[`\0a\x0B\x10\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a3PV[`\0\x81`\0\x03a0\xD6WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0\xEDWP`\0\x91\x90PV[a0\xFEgV\x98\xEE\xF0fp\0\0a?\xE1V[\x82\x13a1\x13WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a1\x1E\x83a3oV[\x90P`\0a1Wg\r\xE0\xB6\xB3\xA7d\0\0a1@\x84g\x1B\xC1mgN\xC8\0\0a\x1F\x81V[a1R\x90g\r\xE0\xB6\xB3\xA7d\0\0a>MV[a3?V[\x90P`\0\x80\x82a1\xB3\x81a1\xA0\x81a1\x8E\x81a1{\x81g\x02_\x0F\xE1\x05\xA3\x14\0a0\xA8V[a/\xAA\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a>MV[a/\xAA\x90g\x14\xA8EL\x19\xE1\xAC\0a>MV[a/\xAA\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a>MV[a1\xC5\x90g\x03\xDE\xBD\x08;\x8C|\0a>MV[\x91P\x83\x90Pa2-\x81a2\x1B\x81a2\t\x81a1\xF7\x81a1\xE4\x81\x8Ba0\xA8V[a/\xAA\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a>MV[a/\xAA\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a>MV[a/\xAA\x90g\x051\n\xA7\xD5!0\0a>MV[a/\xAA\x90g\r\xE0\xCC=\x15a\0\0a>MV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a2C\x87\x88a0\xA8V[a2O\x90`\0\x19a=\xB2V[a2Y\x91\x90a=OV[a2c\x91\x90a>MV[\x92PP`\0a2q\x83a(\x04V[\x90P`\0a2\x7F\x85\x83a0\xA8V[\x90P`\0\x88\x12a2\x8FW\x80a\t\x9BV[a\t\x9B\x81g\x1B\xC1mgN\xC8\0\0a=OV[`\0\x80\x82\x11a2\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a$\x9DV[P`\x01`\x01`\x01`\x80\x1B\x03\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B\x10\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a3hW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a3\x95W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x17\xE1WP\x19`\x01\x01\x90V[\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a3\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4wWa4wa3\xABV[a\x0B\x10\x82a4KV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4\x99Wa4\x99a3\xABV[a4\xA2\x85a4KV[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[\x80\x15\x15\x81\x14a4\xCAW`\0\x80\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a4\xCAW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4\xFBWa4\xFBa3\xABV[a5\x04\x85a4KV[\x93P` \x85\x015a5\x14\x81a4\xBCV[\x92P`@\x85\x015\x91P``\x85\x015a5+\x81a4\xCDV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a5LWa5La3\xABV[a5U\x83a4KV[\x91P` \x83\x015a5e\x81a4\xBCV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a5\x8BWa5\x8Ba3\xABV[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015a5\xAB\x81a4\xBCV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[``\x81R`\0\x84Q\x80``\x84\x01R`\0[\x81\x81\x10\x15a5\xEAW` \x81\x88\x01\x81\x01Q`\x80\x86\x84\x01\x01R\x01a5\xCDV[P`\0`\x80\x82\x85\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x83` \x83\x01R\x82`@\x83\x01R\x94\x93PPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a6\xA1Wa6\xA1a6iV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a6\xCFWa6\xCFa6iV[`@R\x91\x90PV[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a4\xCAW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a7\x01Wa7\x01a3\xABV[a7\ta6\x7FV[\x825a7\x14\x81a6\xD7V[\x81R` \x83\x015a7$\x81a6\xD7V[` \x82\x01R`@\x83\x015a77\x81a4\xBCV[`@\x82\x01Ra7H``\x84\x01a4KV[``\x82\x01R`\x80\x83\x015a7[\x81a4\xBCV[`\x80\x82\x01R\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x80\x83\x85\x03\x12\x15a7\xD6Wa7\xD6a3\xABV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7\xF0Wa7\xF0a3\xFBV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a8\x07Wa8\x07a7gV[\x815\x81\x81\x11\x15a8\x19Wa8\x19a6iV[a8+`\x1F\x82\x01`\x1F\x19\x16\x85\x01a6\xA7V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a8\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a8\xC5Wa8\xC5a3\xABV[a8\xCE\x84a4KV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a8\xEDWa8\xEDa3\xFBV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a9\x04Wa9\x04a7gV[\x815\x81\x81\x11\x15a9gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a9\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a9\xFAWa9\xFAa3\xABV[a:\x03\x84a4KV[\x92P` \x84\x015a:\x13\x81a4\xBCV[\x91P`@\x84\x015a:#\x81a4\xCDV[\x80\x91PP\x92P\x92P\x92V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4\xCAW`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a3\xA6W`\0\x80\xFD[\x80Qa3\xA6\x81a4\xCDV[`\0`\xE0\x82\x84\x03\x12\x15a:rWa:ra3\xABV[`@Q`\xE0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a:\x94Wa:\x94a6iV[`@R\x82Qa:\xA2\x81a6\xD7V[\x81R` \x83\x01Qa:\xB2\x81a6\xD7V[` \x82\x01R`@\x83\x01Qa:\xC5\x81a6\xD7V[`@\x82\x01R``\x83\x01Qa:\xD8\x81a:.V[``\x82\x01Ra:\xE9`\x80\x84\x01a:@V[`\x80\x82\x01Ra:\xFA`\xA0\x84\x01a:@V[`\xA0\x82\x01Ra;\x0B`\xC0\x84\x01a:RV[`\xC0\x82\x01R\x93\x92PPPV[\x80Q`\xFF\x81\x16\x81\x14a3\xA6W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a;=Wa;=a3\xABV[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a;_Wa;_a6iV[`@R\x82Qa;m\x81a4\xCDV[\x81Ra;{` \x84\x01a;\x17V[` \x82\x01R`@\x83\x01Qa;\x8E\x81a4\xCDV[`@\x82\x01Ra;\x9F``\x84\x01a;\x17V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a;\xC0Wa;\xC0a3\xABV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a&eWa&ea;\xC7V[`\x01`\x01`\x80\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a&eWa&ea;\xC7V[`\0\x80\x82\x84\x03`\xC0\x81\x12\x15a<4Wa<4a3\xABV[`\xA0\x81\x12\x15a<EWa<Ea6\x18V[Pa<Na6\x7FV[\x83Qa<Y\x81a6\xD7V[\x81R` \x84\x01Qa<i\x81a:.V[` \x82\x01R`@\x84\x01Qa<|\x81a:.V[`@\x82\x01R``\x84\x01Qa<\x8F\x81a:.V[``\x82\x01R`\x80\x84\x01Qa<\xA2\x81a4\xBCV[`\x80\x82\x01R`\xA0\x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80\x82\x84\x03`\xC0\x81\x12\x15a<\xCEWa<\xCEa3\xABV[`\xA0\x81\x12\x15a<\xDFWa<\xDFa6\x18V[Pa<\xE8a6\x7FV[\x835a<\xF3\x81a6\xD7V[\x81R` \x84\x015a=\x03\x81a:.V[` \x82\x01R`@\x84\x015a=\x16\x81a:.V[`@\x82\x01R``\x84\x015a=)\x81a:.V[``\x82\x01R`\x80\x84\x015a=<\x81a4\xBCV[`\x80\x82\x01R\x94`\xA0\x93\x90\x93\x015\x93PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&eWa&ea;\xC7V[`\0`\x01\x82\x01a=\x81Wa=\x81a;\xC7V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0E'Wa\x0E'a;\xC7V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E'Wa\x0E'a;\xC7V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a=\xCEWa=\xCEa;\xC7V[\x81\x81\x05\x83\x14\x82\x15\x17a\x0E'Wa\x0E'a;\xC7V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a>\x07Wa>\x07a=\xE2V[P\x04\x90V[`\0\x82a>\x1BWa>\x1Ba=\xE2V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a>5Wa>5a;\xC7V[P\x05\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0E'Wa\x0E'a;\xC7V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a>mWa>ma;\xC7V[PP\x92\x91PPV[`\x01\x81\x81[\x80\x85\x11\x15a>\xB0W\x81`\0\x19\x04\x82\x11\x15a>\x96Wa>\x96a;\xC7V[\x80\x85\x16\x15a>\xA3W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a>zV[P\x92P\x92\x90PV[`\0\x82a>\xC7WP`\x01a\x0E'V[\x81a>\xD4WP`\0a\x0E'V[\x81`\x01\x81\x14a>\xEAW`\x02\x81\x14a>\xF4Wa?\x10V[`\x01\x91PPa\x0E'V[`\xFF\x84\x11\x15a?\x05Wa?\x05a;\xC7V[PP`\x01\x82\x1Ba\x0E'V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a?3WP\x81\x81\na\x0E'V[a?=\x83\x83a>uV[\x80`\0\x19\x04\x82\x11\x15a?QWa?Qa;\xC7V[\x02\x93\x92PPPV[`\0a\x0B\x10\x83\x83a>\xB8V[`\0`\xC0\x82\x84\x03\x12\x15a?zWa?za3\xABV[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a?\x9CWa?\x9Ca6iV[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a?\xF6Wa?\xF6a;\xC7V[P`\0\x03\x90V\xFETarget contract does not contain\xA2dipfsX\"\x12 d$Ri\xBDa\x0F\xD0/\xB3\x87\x9B\x91\xE32u \xCA\x03\xE6\x98\x9A\x83\x83\x16\xF4@\xF2\x08+\xCA\xCEdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static NORMALSTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80cE-/\x18\x11a\0\xD9W\x80c\xE0hx\x7F\x11a\0\xB3W\x80c\xE0hx\x7F\x14a\x03\xD5W\x80c\xE31\xBA4\x14a\x03\xEAW\x80c\xEA\x82\0Y\x14a\x03\xFDW\x80c\xECshT\x14a\x04 Wa\x017V[\x80cE-/\x18\x14a\x03xW\x80cE2\x0C\x8C\x14a\x03\x9AW\x80cT'g\xFC\x14a\x03\xC2Wa\x017V[\x80c\x19\x05x\x07\x11a\x01\x15W\x80c\x19\x05x\x07\x14a\x02=W\x80c4\xDB\xC7;\x14a\x02^W\x80c9CMZ\x14a\x02\xF6W\x80cD\x1F\xA75\x14a\x03\tWa\x017V[\x80c\x08\xE6\xCC\xA3\x14a\x01\x9CW\x80c\x0C\x8A\x11?\x14a\x01\xD4W\x80c\x16\xED\xE0\x16\x14a\x01\xFEW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01\xB4a\x01\xAA6`\x04a4bV[P`\0\x90\x81\x90\x81\x90V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE7a\x01\xE26`\x04a4\x80V[a\x043V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xCBV[a\x02%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xCBV[a\x02Pa\x02K6`\x04a4\xE2V[a\x05\xE2V[`@Q\x90\x81R` \x01a\x01\xCBV[a\x02\xB6a\x02l6`\x04a4bV[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\x80\x1B\x03\x81\x16\x90c\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x91`\x01`\xA0\x1B\x81\x04\x82\x16\x91`\x01`\xC0\x1B\x82\x04\x16\x90`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x85V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x96\x16\x86Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x91\x90\x91\x16``\x83\x01R\x15\x15`\x80\x82\x01R`\xA0\x01a\x01\xCBV[a\x02Pa\x03\x046`\x04a4bV[a\t\xA7V[a\x03\x1Ca\x03\x176`\x04a56V[a\x0B\x17V[`@Qa\x01\xCB\x91\x90`\0`\xA0\x82\x01\x90P`\x01`\x01`\x80\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q\x15\x15`@\x83\x01R`\x01`\x01`@\x1B\x03``\x84\x01Q\x16``\x83\x01R`\x80\x83\x01Q\x15\x15`\x80\x83\x01R\x92\x91PPV[a\x03\x8Ba\x03\x866`\x04a5pV[a\x0E-V[`@Qa\x01\xCB\x93\x92\x91\x90a5\xBCV[a\x03\xADa\x03\xA86`\x04a6\xECV[a\x0F\x15V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xCBV[a\x03\xADa\x03\xD06`\x04a7\xC0V[a\x11nV[a\x03\xE8a\x03\xE36`\x04a8\xADV[a\x11\xAFV[\0[a\x02Pa\x03\xF86`\x04a4bV[a\x12*V[a\x04\x10a\x04\x0B6`\x04a4bV[a\x13\xC5V[`@Q\x90\x15\x15\x81R` \x01a\x01\xCBV[a\x01\xE7a\x04.6`\x04a9\xE2V[a\x14\xB9V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x18\x91\x90a:]V[\x90Pa\x05#\x85a\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x16\x81Ra\x057\x84a\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x83\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R\x80\x82R`@\x80\x82 \x81Q`\xA0\x81\x01\x83R\x90T\x94\x85\x16\x81R`\x01`\x80\x1B\x85\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x82\x01\x94\x90\x94R`\x01`\xA0\x1B\x85\x04\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x84\x04\x83\x16``\x82\x01R`\x01`\xE0\x1B\x90\x93\x04`\xFF\x16\x15\x15`\x80\x84\x01R\x91a\x05\xC5\x91\x84\x91\x90a\x17\xE5\x16V[\x90P`\0a\x05\xD3\x88\x83a\x18\xC4V[\x99\x91\x98P\x90\x96PPPPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC5\x91\x90a:]V[`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF`(\x89\x90\x1C\x16`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^Gf<\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA9\x91\x90a;(V[\x90Pa\x07\xCD\x86a\x07\xBDW\x81``\x01Qa\x07\xC3V[\x81` \x01Q[\x86\x90`\xFF\x16a\x18\xF0V[`\x01`\x01`@\x1B\x03\x88\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x80\x82\x01\x85R\x91T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x82\x86\x01R`\x01`\xC0\x1B\x81\x04\x90\x93\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x82Q\x90\x81\x01\x90\x92R\x91\x96Pa\ts\x91\x90\x80a\x08\\\x89a\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01\x8A`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x89\x15\x15\x81RPB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB0\xE2\x1E\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\tDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\th\x91\x90a;\xABV[\x86\x93\x92\x91\x90\x89a\x19\x07V[\x92P`\0\x86a\t\x86W\x81` \x01Qa\t\x8CV[\x81``\x01Q[`\xFF\x16\x90Pa\t\x9B\x84\x82a\x1A\x81V[\x98\x97PPPPPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\nRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\nfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x8A\x91\x90a:]V[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x82\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x91\x92Pa\x0B\x10\x91\x83\x91\x90a\x17\xE5\x16V[\x93\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0B\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C#\x91\x90a:]V[`\x01`\x01`@\x1B\x03\x85\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xA0\x1B\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x82\x04\x90\x92\x16``\x83\x01R`\xFF`\x01`\xE0\x1B\x90\x91\x04\x16\x15\x15`\x80\x82\x01R\x91\x92Pa\x0C\xA4\x82a\x1A\x97V[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\x0C\xC5\x85`@\x01Q\x90`\0\x90V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x8D\x16``\x82\x01R\x8B\x15\x80\x15`\x80\x83\x01R\x92\x94P\x90\x92P\x90a\r\x97W`\x01\x88`\0\x01Qa\r4a\r/\x8B`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x89a\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x17\xCFV[a\r>\x91\x90a;\xDDV[a\rH\x91\x90a;\xDDV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x82R`@\x89\x01Qa\rj\x91a\r/\x91\x85\x91\x16a\x1B=V[\x88` \x01Qa\ry\x91\x90a;\xDDV[a\r\x84\x90`\x01a;\xFDV[`\x01`\x01`\x80\x1B\x03\x16` \x82\x01Ra\x0E\x1DV[`\x01\x88` \x01Qa\r\xC1a\r/\x8B`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x87a\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xCB\x91\x90a;\xDDV[a\r\xD5\x91\x90a;\xDDV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x82R`@\x89\x01Qa\r\xF7\x91a\r/\x91\x87\x91\x16a\x1B=V[\x88Qa\x0E\x03\x91\x90a;\xDDV[a\x0E\x0E\x90`\x01a;\xFDV[`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R[\x97PPPPPPPP[\x92\x91PPV[```\0\x80`\0`@Q\x80`\xA0\x01`@R\x80a\x0EH\x8Ba\x17\xCFV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x0E_\x8Aa\x1BRV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x0Es\x89a\x1BRV[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R`\0` \x80\x84\x01\x91\x90\x91R\x89\x15\x15`@\x93\x84\x01R\x82Q\x84Q`\x01`\x01`\x80\x1B\x03\x16\x81\x83\x01R\x90\x84\x01Q\x82\x16\x81\x84\x01R\x91\x83\x01Q\x81\x16``\x80\x84\x01\x91\x90\x91R\x83\x01Q\x16`\x80\x80\x83\x01\x91\x90\x91R\x82\x01Q\x15\x15`\xA0\x82\x01R`\xC0\x81\x01\x87\x90R\x90\x91P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93Pa\x0F\x03\x85a\x0E\xFD\x83a\x1A\x97V[\x90a\x1BeV[\x94\x9A\x90\x99P\x93\x97P\x92\x95PPPPPPV[``\x81\x01Q`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\0\x91\x90a:]V[``\x85\x81\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x83\x86\x01R`\x01`\xA0\x1B\x82\x04\x81\x16\x83\x85\x01R`\x01`\xC0\x1B\x82\x04\x16\x95\x82\x01\x95\x90\x95R`\x01`\xE0\x1B\x90\x94\x04`\xFF\x16\x15\x15`\x80\x85\x01R\x80QcXq\x0FE`\xE1\x1B\x81R\x90Q\x94\x95P\x91\x93\x84\x93a\x11`\x93\x90\x92\x8A\x92B\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92c\xB0\xE2\x1E\x8A\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x11\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x111W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11U\x91\x90a;\xABV[\x87\x93\x92\x91\x903a\x1B\x89V[\x90\x98\x90\x97P\x95PPPPPPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x11\x88\x91\x90a<\x1DV[\x91P\x91P`\0a\x11\x97\x83a\x1A\x97V[\x90Pa\x11\xA3\x81\x83a\x1BeV[\x94P\x94PPPP\x91P\x91V[`\0\x80a\x11\xBE\x83\x85\x01\x85a<\xB7V[\x91P\x91Pa\x12#\x82`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x84`@\x01Qc\xFF\xFF\xFF\xFF\x16\x85`\x80\x01Q`\0\x80\x8B`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 a\x1D\xB6\x90\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\r\x91\x90a:]V[\x90Pa\x0B\x10a\x13?\x82`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`@\x1B\x03\x85\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x90\x91\x16``\x82\x01R`\x01`\xE0\x1B\x90\x91\x04`\xFF\x16\x15\x15`\x80\x82\x01Ra\x13\xBF\x90a\x1A\x97V[\x90a\x1F\x96V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\0\x90a\x0E'\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x14\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xAA\x91\x90a:]V[``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89\xA5\xF0\x84\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9E\x91\x90a:]V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x80\x82\x01\x84R\x91T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x83\x87\x01R`\x01`\xA0\x1B\x82\x04\x81\x16\x83\x86\x01R`\x01`\xC0\x1B\x82\x04\x16``\x80\x84\x01\x91\x90\x91R`\x01`\xE0\x1B\x90\x91\x04`\xFF\x16\x15\x15`\x80\x80\x84\x01\x91\x90\x91R\x84Q\x93\x84\x01\x85R\x86\x84R\x83\x86\x01\x87\x90R\x83\x85\x01\x87\x90R\x90\x83\x01\x96\x90\x96R\x8B\x15\x15\x95\x82\x01\x95\x90\x95R\x81QcXq\x0FE`\xE1\x1B\x81R\x91Q\x95\x96P\x92\x94a\x17&\x94\x93\x92B\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92c\xB0\xE2\x1E\x8A\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x16\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a?\xFE\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x16\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x1B\x91\x90a;\xABV[\x86\x93\x92\x91\x90\x8Aa\x1B\x89V[P`\x01`\x01`@\x1B\x03\x89\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x01`\x80\x1B\x81\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xA0\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`\xC0\x1B\x83\x04\x82\x16``\x82\x01R`\x01`\xE0\x1B\x90\x92\x04`\xFF\x16\x15\x15`\x80\x83\x01R\x91\x93Pa\x17\xAD\x92P\x84\x91a \xC4\x16V[\x15a\x17\xBFW`\0\x93P\x91Pa\x17\xC7\x90PV[`\x01\x93P\x91PP[\x93P\x93\x91PPV[`\0`\x01`\x80\x1B\x82\x10a\x17\xE1W`\0\x80\xFD[P\x90V[`\0\x80a\x18\x15\x84`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x18F\x85`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x86` \x01Q`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0`@Q\x80`\xC0\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x86`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x18\x96\x87` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x18\xA5\x88\x88a \xF6V[\x81R` \x01`\0\x81RP\x90Pa\x18\xBA\x81a!\rV[\x96\x95PPPPPPV[`\0\x80a\x18\xD1\x84\x84a=OV[\x90P`\x01\x81\x12\x15a\x18\xE6W`\0\x91PPa\x0E'V[P`\x01\x93\x92PPPV[`\0\x80a\x18\xFC\x83a\"\x14V[\x93\x90\x93\x02\x93\x92PPPV[`\0\x80`\0\x80a\x19\x1B\x8A\x8A\x8A\x8A\x8A\x8Aa\x1B\x89V[\x92P\x92P\x92P`\0a\x19,\x8Aa\x1A\x97V[`\xA0\x81\x01\x84\x90R`\x80\x8A\x01Q\x90\x91P`\0\x90\x80\x15a\x19^W\x85\x83Ra\x19P\x83a\",V[` \x84\x01\x81\x90R\x91Pa\x19tV[` \x83\x01\x86\x90Ra\x19n\x83a\"\xEDV[\x80\x84R\x91P[`\0a\x19\x83\x83`b`da#\xAEV[\x90P`\0a\x19\x94\x84`f`da#\xCDV[\x90P\x83\x98Pa\x1A\x17\x85`@Q` \x01a\x19\xEC\x91\x90`\0`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\0a\x01\0\x88a\x1A\x0FWa#\xFBa$tV[a$Ca$tV[\x98P\x88a\x1A#\x81a=oV[\x99PPa\x1AF\x8F`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x8Aa\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x98P\x88\x83a\x1AUW\x8FQa\x1A[V[\x8F` \x01Q[`\x01`\x01`\x80\x1B\x03\x16a\x1An\x91\x90a=\x88V[\x9F\x9EPPPPPPPPPPPPPPPV[`\0\x80a\x1A\x8D\x83a\"\x14V[\x90\x93\x04\x93\x92PPPV[a\x1A\xD0`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01\x83`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x1B\x1E\x84` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R`@\x93\x84\x01Qc\xFF\xFF\xFF\xFF\x16` \x82\x01R`\0\x93\x01\x92\x90\x92RP\x90V[`\0a\x0B\x10\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a#\xAEV[`\0d\x01\0\0\0\0\x82\x10a\x17\xE1W`\0\x80\xFD[`\0\x80a\x1Br\x84\x84a%\x8AV[\x80\x85R\x91Pa\x1B\x80\x84a\",V[\x90P\x92P\x92\x90PV[`\0\x80`\0a\x1B\xC7`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x89Q` \x8B\x01Q`\x80\x8A\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90\x15a\x1C*W`@\x8C\x01Qa\x1C\x01\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81V[\x91Pa\x1C#\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a&l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x1ChV[`@\x8C\x01Qa\x1CC\x90\x83\x90`\x01`\x01`\x80\x1B\x03\x16a&lV[\x91Pa\x1Ce\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x82a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P[`@Q\x80`\xC0\x01`@R\x80\x83\x81R` \x01\x82\x81R` \x01\x8C`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x1C\xB4\x8D` \x01Qc\xFF\xFF\xFF\xFF\x16a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[\x81R` \x01a\x1C\xC4\x8E\x8E\x8Da&\x81V[\x81R` \x01`\0\x81RP\x92PPPa\x1C\xDB\x81a!\rV[\x8AQ` \x8C\x01Q`\xC0\x8D\x01Q\x92\x95P`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x16\x90\x8A\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x16\x14a\x1D\x1AW\x8D`\x80\x01Qa\x1D V[\x8D`\xA0\x01Q[a\xFF\xFF\x16\x90Pa\x1D3\x82\x85\x85\x84\x8Ea&\xBFV[\x90\x91\x92P\x90\x91P\x80\x94P\x81\x95PPPPPa\x1Dd\x8C`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83R`@\x8C\x01Qa\x1D\x7F\x90\x82\x90`\x01`\x01`\x80\x1B\x03\x16a\x1F\x81V[` \x84\x01Ra\x1D\x8D\x83a!\rV[\x93P\x89`\x80\x01Qa\x1D\xA2W\x82` \x01Qa\x1D\xA5V[\x82Q[\x95PPPP\x96P\x96P\x96\x93PPPPV[\x84T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x1D\xE3W`@Qc\x1E\xC0\xFD\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1E0W\x84T`\xFF`\xE0\x1B\x19\x16`\x01`\xE0\x1B\x82\x15\x15\x02\x17\x85Ua\x1E\x0Bc\x01\xE1\x85Ya\x1BRV[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85Ua\x1E\xA2V[a\x1E[b\x01Q\x80a\x1EFc\x01\xE1\x85Y`\x03a=\x9BV[\x80\x85\x10\x90\x85\x14\x17\x81\x85\x11\x91\x85\x14\x91\x90\x91\x17\x16\x90V[a\x1ExW`@Qc4\xB8\x03\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x81\x82a\x1BRV[\x85Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x85U[aa\xA8\x80\x84\x10\x90\x84\x14\x17`\x01\x80\x85\x11\x90\x85\x14\x17\x16a\x1E\xD3W`@Qc\xEC\xDC\x82\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xDC\x83a\x1BRV[\x85Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1Bc\xFF\xFF\xFF\xFF\x92\x83\x16\x02\x17\x86Ua\x1F\x11\x90\x85\x90`\x01\x90`\x01`\x01`\x80\x1B\x03\x90a&\xF3\x16V[a\x1F.W`@Qc\"\xFEks`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F7\x84a\x17\xCFV[\x85T`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`\x01`\xC0\x1BBc\xFF\xFF\xFF\xFF\x16\x02\x17\x90\x94UPPPPV[`\0a\x0B\x10\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xAEV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\x1F\xB6\x87`@\x01Q\x90`\0\x90V[\x91P\x91P\x83\x86\x10a\x1F\xD1W\x86`@\x01Q\x94PPPPPa\x0E'V[\x82\x86\x11a\x1F\xECWP`\x01`\x01`\x80\x1B\x03\x93Pa\x0E'\x92PPPV[`\x80\x87\x01Q`\0\x90a \x02\x90c\x01\xE1\x85Ya\x1F\x81V[\x90P`\0a \x0F\x89a'\tV[\x90P`\0a -a (\x8Ag\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[a'gV[\x90P`\0a ;\x83\x83a=\xB2V[\x90P`\0g\x1B\xC1mgN\xC8\0\0\x85\x8D``\x01Q\x8E``\x01Qa ]\x91\x90a=\x9BV[a g\x91\x90a=\x9BV[a q\x91\x90a=\xF8V[\x90P`\0a \x9Ag\r\xE0\xB6\xB3\xA7d\0\0a \x8B\x84\x86a=OV[a \x95\x91\x90a>\x0CV[a(\x04V[\x90Pa \xB3\x8D`@\x01Q\x82a\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9D\x9CPPPPPPPPPPPPPV[`\0\x81`\x80\x01Q\x80a\x0B\x10WPa \xDB\x83\x83a)\xADV[c\xFF\xFF\xFF\xFF\x16\x83``\x01Qc\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x92\x91PPV[`\0a\x0B\x10\x83\x83\x85``\x01Qc\xFF\xFF\xFF\xFF\x16a&\x81V[`\0\x80a!\x19\x83a'\tV[\x90P`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a!:\x87`@\x01Q\x90`\0\x90V[\x90\x92P\x90P`\0a!ta!O\x85`\x01a>:V[a!Z`\x01\x88a=\x88V[\x8AQ\x91\x90\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[\x15a!\x94W\x87Qa!\x91\x90a (\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[\x90P[`\0a!\xCCa!\xA4\x84`\x01a>:V[a!\xAF`\x01\x87a=\x88V[` \x8C\x01Q\x91\x90\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[\x15a!\xF2Wa!\xEFa (\x8A`@\x01Q\x8B` \x01Qa&l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P[\x86a!\xFD\x83\x83a=OV[a\"\x07\x91\x90a>MV[\x99\x98PPPPPPPPPV[`\0a\"!\x82`\x12a=\x88V[a\x0E'\x90`\na?YV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a\"L\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x83\x86`\0\x01Q\x10a\"dW\x95\x94PPPPPV[\x85Q\x83\x10a\"uWP\x94\x93PPPPV[`\0a\"\x80\x87a'\tV[\x87Q\x90\x91P`\0\x90a\"\x9A\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[\x90P`\0a\"\xA7\x82a'gV[\x90P`\0\x89`\xA0\x01Q\x84\x83a\"\xBC\x91\x90a=OV[a\"\xC6\x91\x90a>MV[\x90Pa\"\xDF\x8A`@\x01Qa\"\xD9\x83a)\xE4V[\x90a\x1B=V[\x9A\x99PPPPPPPPPPV[`\0\x80\x80g\r\xE0\xB6\xB3\xA7d\0\0\x91P`\0\x80a#\r\x86`@\x01Q\x90`\0\x90V[\x91P\x91P\x81\x86` \x01Q\x10a#&WP\x90\x94\x93PPPPV[\x80\x86` \x01Q\x11a#;WP\x91\x94\x93PPPPV[`\0a#F\x87a'\tV[\x90P`\0a#e\x88`@\x01Q\x89` \x01Qa&l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#r\x82a'gV[\x90P`\0\x89`\xA0\x01Q\x84\x83a#\x87\x91\x90a>MV[a#\x91\x91\x90a=OV[\x90Pa#\x9C\x81a)\xE4V[a\"\xDF\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\x88V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xC6W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#\xE5W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a$\x12\x91\x90a?eV[\x83\x81R`\xA0\x81\x01Q\x90\x91Pa$(\x90`\x01a>MV[a$1\x82a!\rV[a$;\x91\x90a=OV[\x94\x93PPPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a$Z\x91\x90a?eV[` \x81\x01\x84\x90R`\xA0\x81\x01Q\x90\x91Pa$(\x90`\x01a>MV[`\0\x84\x86\x11\x15a$\xA6W`@Qcr\x17\r\xED`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a$\xB6\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$\xC8\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$\xD6\x82\x84a=\xB2V[\x13\x15a$\xFFW`@Qc#\x84\x17\xCB`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a$\x9DV[`\0a%\x0B\x89\x89a=\x88V[\x90P`\0[`\x02a%\x1C\x8A\x8Ca>:V[a%&\x91\x90a=\xF8V[\x94P`\0a%8\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a%F\x86\x83a=\xB2V[\x13a%SW\x85\x99Pa%ZV[\x85\x9AP\x80\x94P[a%d\x8B\x8Ba=\x88V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a%xWP\x86\x81\x10[a%\x10WPPPP\x96\x95PPPPPPV[`\0\x80a%\xA4\x84`@\x01Q\x84a\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x15a&eW`\0a%\xB7\x82a*MV[\x90P`\0a%\xD6c\x01\xE1\x85Y\x87`\x80\x01Qa\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01Q\x90\x91P`\0\x90g\x1B\xC1mgN\xC8\0\0\x90a%\xF5\x90\x80a=\x9BV[a%\xFF\x91\x90a=\xF8V[\x90P`\0a&\x0C\x88a'\tV[\x90P`\0a&\"g\r\xE0\xB6\xB3\xA7d\0\0\x86a=\xB2V[\x90Pa&.\x84\x84a=\xB2V[a&8\x90\x82a>MV[\x90Pa&D\x82\x82a>\x0CV[\x90P`\0a&Q\x82a)\xE4V[\x90Pa\x0E\x1D\x81g\r\xE0\xB6\xB3\xA7d\0\0a=OV[P\x92\x91PPV[`\0a\x0B\x10\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a#\xCDV[`\0\x82`\x80\x01Q\x15a&\x98WPc\x01\xE1\x85Ya\x0B\x10V[`\0a&\xA4\x85\x85a)\xADV[c\xFF\xFF\xFF\xFF\x16\x80\x84\x03\x90\x84\x10\x02\x83\x03\x92\x90\x92\x03\x94\x93PPPPV[`\0\x80\x80\x80a&\xCF\x89\x87\x87a,(V[\x90\x94P\x92Pa&\xE1\x89\x89\x89\x87\x87a,mV[\x94\x9A\x93\x99P\x97P\x92\x95P\x90\x93PPPPV[\x81\x83\x14\x91\x83\x11\x91\x90\x91\x17\x82\x82\x14\x91\x90\x92\x10\x17\x16\x90V[`\0\x80a''c\x01\xE1\x85Y\x84`\x80\x01Qa\x1F\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0c;\x9A\xCA\0a'9\x83a-\x86V[a'C\x91\x90a=\x9BV[\x90P`\0a'^\x82\x86``\x01Qa\x1B=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95\x94PPPPPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a'\x80WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a'\xA8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a'\xC9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\xD6\x83`\x02a=\xB2V[\x90P`\0a'\xE3\x82a.*V[\x90P`\0a'\xF9g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a0\xA8V[\x90Pa'^\x81a?\xE1V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a(\x1FWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a(fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a$\x9DV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x81`\x80\x01Q\x15a)\xD2W`@Qc5\x7F\xF5\x0B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x81\x01Q``\x82\x01Q\x01\x92\x91PPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a*\x02g\r\xE0\xB6\xB3\xA7d\0\0\x85a=\xB2V[a*\x0C\x91\x90a>\x0CV[\x90P`\0a*\x19\x82a?\xE1V[\x90P`\0a*&\x82a0\xBDV[\x90Pg\x1B\xC1mgN\xC8\0\0a*Cg\r\xE0\xB6\xB3\xA7d\0\0\x83a=\xB2V[a'^\x91\x90a>\x0CV[`\0\x80\x82\x13a*\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a$\x9DV[`\0``a*\x97\x84a2\xA1V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82Q`\0\x90\x81\x90a,E\x90`\x01`\x01`\x80\x1B\x03\x16\x85a'\x10a#\xCDV[\x91P\x82\x15a\x17\xC7Wa,W\x83\x83a=\xF8V[\x90Pa,c\x81\x83a=\x88V[\x91P\x93P\x93\x91PPV[`\0\x80`\0\x87`\x80\x01Qa,\x81W\x85a,\x83V[\x86[\x90P`\0\x88`\x80\x01Qa,\x96W\x87a,\x98V[\x86[\x89Q\x90\x91Pa,\xB0\x90`\x01`\x01`\x80\x1B\x03\x16\x83a>:V[\x91P\x81\x86\x11\x15a,\xD3W`@Qcp\r~\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\xDD\x86\x83a=\x88V[\x91P\x81\x85\x11\x15a-\0W`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\n\x85\x83a=\x88V[\x91P\x80\x89` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x11\x15a-:W`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x89\x01Qa-R\x90`\x01`\x01`\x80\x1B\x03\x16\x82a=\x88V[\x90P\x88`\x80\x01Qa-cW\x80a-eV[\x81[\x93P\x88`\x80\x01Qa-vW\x81a-xV[\x80[\x92PPP\x95P\x95\x93PPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a-\x9FW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a-\xBBW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a-\xD3W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a-\xE9W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x80\x82\x12\x80a.AWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a._W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a.\x80W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a.\xA8W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a.\xB3W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a.\xDBWa.\xD6\x83g\x1B\xC1mgN\xC8\0\0a=OV[a.\xDDV[\x82[\x90P`\0a.\xF3\x82g\x1B\xC1mgN\xC8\0\0a3?V[\x90P\x80`\0\x03a/\x16W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a/!\x82a*MV[\x90P`\0c;\x9A\xCA\0a/La/Ga/Ag\x1B\xC1mgN\xC8\0\0a?\xE1V[\x85a0\xA8V[a-\x86V[a/V\x91\x90a=\xB2V[\x90P`\0\x80a/m\x83g\x03\xC1f\\z\xAB \0a0\xA8V[a/\x7F\x90g \x05\xFEO&\x8E\xA0\0a>MV[\x90P`\0a/\xAF\x84a/\x98\x86f\x9F2u$b\xA0\0a0\xA8V[a/\xAA\x90g\r\xC5R\x7Fd, \0a>MV[a0\xA8V[a/\xC1\x90g\r\xE0\xB6\xB3\xA7d\0\0a>MV[\x90Pa/\xE5g\t\xD0(\xCCo _\xFF\x19\x85a/\xDB\x85\x85a3?V[a/\xAA\x91\x90a=OV[\x92PPP`\0[`\x02\x81\x10\x15a0\x80W`\0\x86a0\x01\x84a0\xBDV[a0\x0B\x91\x90a=OV[\x90P`\0a0\x19\x84\x85a0\xA8V[a0\"\x90a?\xE1V[\x90P`\0a0/\x82a(\x04V[\x90P`\0a0=\x86\x85a0\xA8V[a0Og\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a0\xA8V[a0Y\x91\x90a=OV[\x90Pa0e\x84\x82a3?V[a0o\x90\x87a>MV[\x95P\x84`\x01\x01\x94PPPPPa/\xECV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a0\x9DWa0\x98\x82a?\xE1V[a\t\x9BV[P\x96\x95PPPPPPV[`\0a\x0B\x10\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a3PV[`\0\x81`\0\x03a0\xD6WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a0\xEDWP`\0\x91\x90PV[a0\xFEgV\x98\xEE\xF0fp\0\0a?\xE1V[\x82\x13a1\x13WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a1\x1E\x83a3oV[\x90P`\0a1Wg\r\xE0\xB6\xB3\xA7d\0\0a1@\x84g\x1B\xC1mgN\xC8\0\0a\x1F\x81V[a1R\x90g\r\xE0\xB6\xB3\xA7d\0\0a>MV[a3?V[\x90P`\0\x80\x82a1\xB3\x81a1\xA0\x81a1\x8E\x81a1{\x81g\x02_\x0F\xE1\x05\xA3\x14\0a0\xA8V[a/\xAA\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a>MV[a/\xAA\x90g\x14\xA8EL\x19\xE1\xAC\0a>MV[a/\xAA\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a>MV[a1\xC5\x90g\x03\xDE\xBD\x08;\x8C|\0a>MV[\x91P\x83\x90Pa2-\x81a2\x1B\x81a2\t\x81a1\xF7\x81a1\xE4\x81\x8Ba0\xA8V[a/\xAA\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a>MV[a/\xAA\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a>MV[a/\xAA\x90g\x051\n\xA7\xD5!0\0a>MV[a/\xAA\x90g\r\xE0\xCC=\x15a\0\0a>MV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a2C\x87\x88a0\xA8V[a2O\x90`\0\x19a=\xB2V[a2Y\x91\x90a=OV[a2c\x91\x90a>MV[\x92PP`\0a2q\x83a(\x04V[\x90P`\0a2\x7F\x85\x83a0\xA8V[\x90P`\0\x88\x12a2\x8FW\x80a\t\x9BV[a\t\x9B\x81g\x1B\xC1mgN\xC8\0\0a=OV[`\0\x80\x82\x11a2\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a$\x9DV[P`\x01`\x01`\x01`\x80\x1B\x03\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x0B\x10\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a3hW`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a3\x95W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x17\xE1WP\x19`\x01\x01\x90V[\x91\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a3\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4wWa4wa3\xABV[a\x0B\x10\x82a4KV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4\x99Wa4\x99a3\xABV[a4\xA2\x85a4KV[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[\x80\x15\x15\x81\x14a4\xCAW`\0\x80\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a4\xCAW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4\xFBWa4\xFBa3\xABV[a5\x04\x85a4KV[\x93P` \x85\x015a5\x14\x81a4\xBCV[\x92P`@\x85\x015\x91P``\x85\x015a5+\x81a4\xCDV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a5LWa5La3\xABV[a5U\x83a4KV[\x91P` \x83\x015a5e\x81a4\xBCV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a5\x8BWa5\x8Ba3\xABV[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015a5\xAB\x81a4\xBCV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[``\x81R`\0\x84Q\x80``\x84\x01R`\0[\x81\x81\x10\x15a5\xEAW` \x81\x88\x01\x81\x01Q`\x80\x86\x84\x01\x01R\x01a5\xCDV[P`\0`\x80\x82\x85\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x83` \x83\x01R\x82`@\x83\x01R\x94\x93PPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a6\xA1Wa6\xA1a6iV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a6\xCFWa6\xCFa6iV[`@R\x91\x90PV[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a4\xCAW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a7\x01Wa7\x01a3\xABV[a7\ta6\x7FV[\x825a7\x14\x81a6\xD7V[\x81R` \x83\x015a7$\x81a6\xD7V[` \x82\x01R`@\x83\x015a77\x81a4\xBCV[`@\x82\x01Ra7H``\x84\x01a4KV[``\x82\x01R`\x80\x83\x015a7[\x81a4\xBCV[`\x80\x82\x01R\x93\x92PPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[`\0` \x80\x83\x85\x03\x12\x15a7\xD6Wa7\xD6a3\xABV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7\xF0Wa7\xF0a3\xFBV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a8\x07Wa8\x07a7gV[\x815\x81\x81\x11\x15a8\x19Wa8\x19a6iV[a8+`\x1F\x82\x01`\x1F\x19\x16\x85\x01a6\xA7V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a8\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a8\xC5Wa8\xC5a3\xABV[a8\xCE\x84a4KV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a8\xEDWa8\xEDa3\xFBV[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a9\x04Wa9\x04a7gV[\x815\x81\x81\x11\x15a9gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x81\xFD[\x87` \x82\x85\x01\x01\x11\x15a9\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R\x91P`\x84\x82\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a9\xFAWa9\xFAa3\xABV[a:\x03\x84a4KV[\x92P` \x84\x015a:\x13\x81a4\xBCV[\x91P`@\x84\x015a:#\x81a4\xCDV[\x80\x91PP\x92P\x92P\x92V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4\xCAW`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a3\xA6W`\0\x80\xFD[\x80Qa3\xA6\x81a4\xCDV[`\0`\xE0\x82\x84\x03\x12\x15a:rWa:ra3\xABV[`@Q`\xE0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a:\x94Wa:\x94a6iV[`@R\x82Qa:\xA2\x81a6\xD7V[\x81R` \x83\x01Qa:\xB2\x81a6\xD7V[` \x82\x01R`@\x83\x01Qa:\xC5\x81a6\xD7V[`@\x82\x01R``\x83\x01Qa:\xD8\x81a:.V[``\x82\x01Ra:\xE9`\x80\x84\x01a:@V[`\x80\x82\x01Ra:\xFA`\xA0\x84\x01a:@V[`\xA0\x82\x01Ra;\x0B`\xC0\x84\x01a:RV[`\xC0\x82\x01R\x93\x92PPPV[\x80Q`\xFF\x81\x16\x81\x14a3\xA6W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a;=Wa;=a3\xABV[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a;_Wa;_a6iV[`@R\x82Qa;m\x81a4\xCDV[\x81Ra;{` \x84\x01a;\x17V[` \x82\x01R`@\x83\x01Qa;\x8E\x81a4\xCDV[`@\x82\x01Ra;\x9F``\x84\x01a;\x17V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a;\xC0Wa;\xC0a3\xABV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a&eWa&ea;\xC7V[`\x01`\x01`\x80\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a&eWa&ea;\xC7V[`\0\x80\x82\x84\x03`\xC0\x81\x12\x15a<4Wa<4a3\xABV[`\xA0\x81\x12\x15a<EWa<Ea6\x18V[Pa<Na6\x7FV[\x83Qa<Y\x81a6\xD7V[\x81R` \x84\x01Qa<i\x81a:.V[` \x82\x01R`@\x84\x01Qa<|\x81a:.V[`@\x82\x01R``\x84\x01Qa<\x8F\x81a:.V[``\x82\x01R`\x80\x84\x01Qa<\xA2\x81a4\xBCV[`\x80\x82\x01R`\xA0\x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80\x82\x84\x03`\xC0\x81\x12\x15a<\xCEWa<\xCEa3\xABV[`\xA0\x81\x12\x15a<\xDFWa<\xDFa6\x18V[Pa<\xE8a6\x7FV[\x835a<\xF3\x81a6\xD7V[\x81R` \x84\x015a=\x03\x81a:.V[` \x82\x01R`@\x84\x015a=\x16\x81a:.V[`@\x82\x01R``\x84\x015a=)\x81a:.V[``\x82\x01R`\x80\x84\x015a=<\x81a4\xBCV[`\x80\x82\x01R\x94`\xA0\x93\x90\x93\x015\x93PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&eWa&ea;\xC7V[`\0`\x01\x82\x01a=\x81Wa=\x81a;\xC7V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0E'Wa\x0E'a;\xC7V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E'Wa\x0E'a;\xC7V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a=\xCEWa=\xCEa;\xC7V[\x81\x81\x05\x83\x14\x82\x15\x17a\x0E'Wa\x0E'a;\xC7V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a>\x07Wa>\x07a=\xE2V[P\x04\x90V[`\0\x82a>\x1BWa>\x1Ba=\xE2V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a>5Wa>5a;\xC7V[P\x05\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0E'Wa\x0E'a;\xC7V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a>mWa>ma;\xC7V[PP\x92\x91PPV[`\x01\x81\x81[\x80\x85\x11\x15a>\xB0W\x81`\0\x19\x04\x82\x11\x15a>\x96Wa>\x96a;\xC7V[\x80\x85\x16\x15a>\xA3W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a>zV[P\x92P\x92\x90PV[`\0\x82a>\xC7WP`\x01a\x0E'V[\x81a>\xD4WP`\0a\x0E'V[\x81`\x01\x81\x14a>\xEAW`\x02\x81\x14a>\xF4Wa?\x10V[`\x01\x91PPa\x0E'V[`\xFF\x84\x11\x15a?\x05Wa?\x05a;\xC7V[PP`\x01\x82\x1Ba\x0E'V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a?3WP\x81\x81\na\x0E'V[a?=\x83\x83a>uV[\x80`\0\x19\x04\x82\x11\x15a?QWa?Qa;\xC7V[\x02\x93\x92PPPV[`\0a\x0B\x10\x83\x83a>\xB8V[`\0`\xC0\x82\x84\x03\x12\x15a?zWa?za3\xABV[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a?\x9CWa?\x9Ca6iV[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a?\xF6Wa?\xF6a;\xC7V[P`\0\x03\x90V\xFETarget contract does not contain\xA2dipfsX\"\x12 d$Ri\xBDa\x0F\xD0/\xB3\x87\x9B\x91\xE32u \xCA\x03\xE6\x98\x9A\x83\x83\x16\xF4@\xF2\x08+\xCA\xCEdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static NORMALSTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct NormalStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NormalStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NormalStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NormalStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NormalStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NormalStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NormalStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    NORMALSTRATEGY_ABI.clone(),
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
                NORMALSTRATEGY_ABI.clone(),
                NORMALSTRATEGY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `afterCreate` (0xe068787f) function
        pub fn after_create(
            &self,
            pool_id: u64,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 104, 120, 127], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approximateReservesGivenPrice` (0x542767fc) function
        pub fn approximate_reserves_given_price(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([84, 39, 103, 252], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beforeSwap` (0xec736854) function
        pub fn before_swap(
            &self,
            pool_id: u64,
            sell_asset: bool,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash([236, 115, 104, 84], (pool_id, sell_asset, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configs` (0x34dbc73b) function
        pub fn configs(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u32, u32, u32, bool)> {
            self.0
                .method_hash([52, 219, 199, 59], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x19057807) function
        pub fn get_amount_out(
            &self,
            pool_id: u64,
            sell_asset: bool,
            amount_in: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 5, 120, 7], (pool_id, sell_asset, amount_in, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFees` (0x08e6cca3) function
        pub fn get_fees(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([8, 230, 204, 163], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInvariant` (0x39434d5a) function
        pub fn get_invariant(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([57, 67, 77, 90], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxOrder` (0x441fa735) function
        pub fn get_max_order(
            &self,
            pool_id: u64,
            sell_asset: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, Order> {
            self.0
                .method_hash([68, 31, 167, 53], (pool_id, sell_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotPrice` (0xe331ba34) function
        pub fn get_spot_price(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 49, 186, 52], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStrategyData` (0x452d2f18) function
        pub fn get_strategy_data(
            &self,
            strike_price_wad: ::ethers::core::types::U256,
            volatility_basis_points: ::ethers::core::types::U256,
            duration_seconds: ::ethers::core::types::U256,
            is_perpetual: bool,
            price_wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [69, 45, 47, 24],
                    (
                        strike_price_wad,
                        volatility_basis_points,
                        duration_seconds,
                        is_perpetual,
                        price_wad,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapInvariants` (0x45320c8c) function
        pub fn get_swap_invariants(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash([69, 50, 12, 140], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `portfolio` (0x16ede016) function
        pub fn portfolio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 237, 224, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyPool` (0xea820059) function
        pub fn verify_pool(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([234, 130, 0, 89], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySwap` (0x0c8a113f) function
        pub fn verify_swap(
            &self,
            pool_id: u64,
            invariant: ::ethers::core::types::I256,
            reserve_x: ::ethers::core::types::U256,
            reserve_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash(
                    [12, 138, 17, 63],
                    (pool_id, invariant, reserve_x, reserve_y),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for NormalStrategy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CurveLib_ConfigExists` with signature `CurveLib_ConfigExists()` and selector `0xf607ed98`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CurveLib_ConfigExists", abi = "CurveLib_ConfigExists()")]
    pub struct CurveLib_ConfigExists;
    ///Custom Error type `CurveLib_InvalidDuration` with signature `CurveLib_InvalidDuration()` and selector `0xd2e00c6c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CurveLib_InvalidDuration", abi = "CurveLib_InvalidDuration()")]
    pub struct CurveLib_InvalidDuration;
    ///Custom Error type `CurveLib_InvalidStrikePrice` with signature `CurveLib_InvalidStrikePrice()` and selector `0x22fe6b73`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CurveLib_InvalidStrikePrice",
        abi = "CurveLib_InvalidStrikePrice()"
    )]
    pub struct CurveLib_InvalidStrikePrice;
    ///Custom Error type `CurveLib_InvalidVolatility` with signature `CurveLib_InvalidVolatility()` and selector `0xecdc82c7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CurveLib_InvalidVolatility",
        abi = "CurveLib_InvalidVolatility()"
    )]
    pub struct CurveLib_InvalidVolatility;
    ///Custom Error type `CurveLib_NonExpiringPool` with signature `CurveLib_NonExpiringPool()` and selector `0xd5ffd42c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CurveLib_NonExpiringPool", abi = "CurveLib_NonExpiringPool()")]
    pub struct CurveLib_NonExpiringPool;
    ///Custom Error type `Infinity` with signature `Infinity()` and selector `0x07a02127`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
    ///Custom Error type `InvalidBounds` with signature `InvalidBounds(uint256,uint256)` and selector `0x72170ded`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidBounds", abi = "InvalidBounds(uint256,uint256)")]
    pub struct InvalidBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    ///Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    ///Custom Error type `NotInsideBounds` with signature `NotInsideBounds(uint256,uint256)` and selector `0x238417cb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotInsideBounds", abi = "NotInsideBounds(uint256,uint256)")]
    pub struct NotInsideBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
    ///Custom Error type `SwapLib_FeeTooHigh` with signature `SwapLib_FeeTooHigh()` and selector `0xe01afdee`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SwapLib_FeeTooHigh", abi = "SwapLib_FeeTooHigh()")]
    pub struct SwapLib_FeeTooHigh;
    ///Custom Error type `SwapLib_OutputExceedsReserves` with signature `SwapLib_OutputExceedsReserves()` and selector `0x866a032b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SwapLib_OutputExceedsReserves",
        abi = "SwapLib_OutputExceedsReserves()"
    )]
    pub struct SwapLib_OutputExceedsReserves;
    ///Custom Error type `SwapLib_ProtocolFeeTooHigh` with signature `SwapLib_ProtocolFeeTooHigh()` and selector `0xec8e1fce`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SwapLib_ProtocolFeeTooHigh",
        abi = "SwapLib_ProtocolFeeTooHigh()"
    )]
    pub struct SwapLib_ProtocolFeeTooHigh;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum NormalStrategyErrors {
        CurveLib_ConfigExists(CurveLib_ConfigExists),
        CurveLib_InvalidDuration(CurveLib_InvalidDuration),
        CurveLib_InvalidStrikePrice(CurveLib_InvalidStrikePrice),
        CurveLib_InvalidVolatility(CurveLib_InvalidVolatility),
        CurveLib_NonExpiringPool(CurveLib_NonExpiringPool),
        Infinity(Infinity),
        InvalidBounds(InvalidBounds),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NotInsideBounds(NotInsideBounds),
        OutOfBounds(OutOfBounds),
        SwapLib_FeeTooHigh(SwapLib_FeeTooHigh),
        SwapLib_OutputExceedsReserves(SwapLib_OutputExceedsReserves),
        SwapLib_ProtocolFeeTooHigh(SwapLib_ProtocolFeeTooHigh),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for NormalStrategyErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <CurveLib_ConfigExists as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurveLib_ConfigExists(decoded));
            }
            if let Ok(decoded)
                = <CurveLib_InvalidDuration as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurveLib_InvalidDuration(decoded));
            }
            if let Ok(decoded)
                = <CurveLib_InvalidStrikePrice as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurveLib_InvalidStrikePrice(decoded));
            }
            if let Ok(decoded)
                = <CurveLib_InvalidVolatility as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurveLib_InvalidVolatility(decoded));
            }
            if let Ok(decoded)
                = <CurveLib_NonExpiringPool as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurveLib_NonExpiringPool(decoded));
            }
            if let Ok(decoded)
                = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded)
                = <InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidBounds(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded)
                = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded)
                = <NotInsideBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotInsideBounds(decoded));
            }
            if let Ok(decoded)
                = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            if let Ok(decoded)
                = <SwapLib_FeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapLib_FeeTooHigh(decoded));
            }
            if let Ok(decoded)
                = <SwapLib_OutputExceedsReserves as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapLib_OutputExceedsReserves(decoded));
            }
            if let Ok(decoded)
                = <SwapLib_ProtocolFeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapLib_ProtocolFeeTooHigh(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NormalStrategyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CurveLib_ConfigExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurveLib_InvalidDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurveLib_InvalidStrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurveLib_InvalidVolatility(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurveLib_NonExpiringPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_FeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for NormalStrategyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CurveLib_ConfigExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CurveLib_InvalidDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CurveLib_InvalidStrikePrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CurveLib_InvalidVolatility as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CurveLib_NonExpiringPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInsideBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SwapLib_FeeTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_OutputExceedsReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ProtocolFeeTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for NormalStrategyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CurveLib_ConfigExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurveLib_InvalidDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurveLib_InvalidStrikePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurveLib_InvalidVolatility(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurveLib_NonExpiringPool(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInsideBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_FeeTooHigh(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for NormalStrategyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CurveLib_ConfigExists> for NormalStrategyErrors {
        fn from(value: CurveLib_ConfigExists) -> Self {
            Self::CurveLib_ConfigExists(value)
        }
    }
    impl ::core::convert::From<CurveLib_InvalidDuration> for NormalStrategyErrors {
        fn from(value: CurveLib_InvalidDuration) -> Self {
            Self::CurveLib_InvalidDuration(value)
        }
    }
    impl ::core::convert::From<CurveLib_InvalidStrikePrice> for NormalStrategyErrors {
        fn from(value: CurveLib_InvalidStrikePrice) -> Self {
            Self::CurveLib_InvalidStrikePrice(value)
        }
    }
    impl ::core::convert::From<CurveLib_InvalidVolatility> for NormalStrategyErrors {
        fn from(value: CurveLib_InvalidVolatility) -> Self {
            Self::CurveLib_InvalidVolatility(value)
        }
    }
    impl ::core::convert::From<CurveLib_NonExpiringPool> for NormalStrategyErrors {
        fn from(value: CurveLib_NonExpiringPool) -> Self {
            Self::CurveLib_NonExpiringPool(value)
        }
    }
    impl ::core::convert::From<Infinity> for NormalStrategyErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<InvalidBounds> for NormalStrategyErrors {
        fn from(value: InvalidBounds) -> Self {
            Self::InvalidBounds(value)
        }
    }
    impl ::core::convert::From<Min> for NormalStrategyErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for NormalStrategyErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<NotInsideBounds> for NormalStrategyErrors {
        fn from(value: NotInsideBounds) -> Self {
            Self::NotInsideBounds(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for NormalStrategyErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    impl ::core::convert::From<SwapLib_FeeTooHigh> for NormalStrategyErrors {
        fn from(value: SwapLib_FeeTooHigh) -> Self {
            Self::SwapLib_FeeTooHigh(value)
        }
    }
    impl ::core::convert::From<SwapLib_OutputExceedsReserves> for NormalStrategyErrors {
        fn from(value: SwapLib_OutputExceedsReserves) -> Self {
            Self::SwapLib_OutputExceedsReserves(value)
        }
    }
    impl ::core::convert::From<SwapLib_ProtocolFeeTooHigh> for NormalStrategyErrors {
        fn from(value: SwapLib_ProtocolFeeTooHigh) -> Self {
            Self::SwapLib_ProtocolFeeTooHigh(value)
        }
    }
    ///Container type for all input parameters for the `afterCreate` function with signature `afterCreate(uint64,bytes)` and selector `0xe068787f`
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
    #[ethcall(name = "afterCreate", abi = "afterCreate(uint64,bytes)")]
    pub struct AfterCreateCall {
        pub pool_id: u64,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `approximateReservesGivenPrice` function with signature `approximateReservesGivenPrice(bytes)` and selector `0x542767fc`
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
        name = "approximateReservesGivenPrice",
        abi = "approximateReservesGivenPrice(bytes)"
    )]
    pub struct ApproximateReservesGivenPriceCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `beforeSwap` function with signature `beforeSwap(uint64,bool,address)` and selector `0xec736854`
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
    #[ethcall(name = "beforeSwap", abi = "beforeSwap(uint64,bool,address)")]
    pub struct BeforeSwapCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `configs` function with signature `configs(uint64)` and selector `0x34dbc73b`
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
    #[ethcall(name = "configs", abi = "configs(uint64)")]
    pub struct ConfigsCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint64,bool,uint256,address)")]
    pub struct GetAmountOutCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getFees` function with signature `getFees(uint64)` and selector `0x08e6cca3`
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
    #[ethcall(name = "getFees", abi = "getFees(uint64)")]
    pub struct GetFeesCall(pub u64);
    ///Container type for all input parameters for the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
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
    #[ethcall(name = "getInvariant", abi = "getInvariant(uint64)")]
    pub struct GetInvariantCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getMaxOrder` function with signature `getMaxOrder(uint64,bool)` and selector `0x441fa735`
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
    #[ethcall(name = "getMaxOrder", abi = "getMaxOrder(uint64,bool)")]
    pub struct GetMaxOrderCall {
        pub pool_id: u64,
        pub sell_asset: bool,
    }
    ///Container type for all input parameters for the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
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
    #[ethcall(name = "getSpotPrice", abi = "getSpotPrice(uint64)")]
    pub struct GetSpotPriceCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,bool,uint256)` and selector `0x452d2f18`
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
        name = "getStrategyData",
        abi = "getStrategyData(uint256,uint256,uint256,bool,uint256)"
    )]
    pub struct GetStrategyDataCall {
        pub strike_price_wad: ::ethers::core::types::U256,
        pub volatility_basis_points: ::ethers::core::types::U256,
        pub duration_seconds: ::ethers::core::types::U256,
        pub is_perpetual: bool,
        pub price_wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSwapInvariants` function with signature `getSwapInvariants((uint128,uint128,bool,uint64,bool))` and selector `0x45320c8c`
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
        name = "getSwapInvariants",
        abi = "getSwapInvariants((uint128,uint128,bool,uint64,bool))"
    )]
    pub struct GetSwapInvariantsCall {
        pub order: Order,
    }
    ///Container type for all input parameters for the `portfolio` function with signature `portfolio()` and selector `0x16ede016`
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
    #[ethcall(name = "portfolio", abi = "portfolio()")]
    pub struct PortfolioCall;
    ///Container type for all input parameters for the `verifyPool` function with signature `verifyPool(uint64)` and selector `0xea820059`
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
    #[ethcall(name = "verifyPool", abi = "verifyPool(uint64)")]
    pub struct VerifyPoolCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `verifySwap` function with signature `verifySwap(uint64,int256,uint256,uint256)` and selector `0x0c8a113f`
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
    #[ethcall(name = "verifySwap", abi = "verifySwap(uint64,int256,uint256,uint256)")]
    pub struct VerifySwapCall {
        pub pool_id: u64,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum NormalStrategyCalls {
        AfterCreate(AfterCreateCall),
        ApproximateReservesGivenPrice(ApproximateReservesGivenPriceCall),
        BeforeSwap(BeforeSwapCall),
        Configs(ConfigsCall),
        GetAmountOut(GetAmountOutCall),
        GetFees(GetFeesCall),
        GetInvariant(GetInvariantCall),
        GetMaxOrder(GetMaxOrderCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategyData(GetStrategyDataCall),
        GetSwapInvariants(GetSwapInvariantsCall),
        Portfolio(PortfolioCall),
        VerifyPool(VerifyPoolCall),
        VerifySwap(VerifySwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for NormalStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AfterCreateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AfterCreate(decoded));
            }
            if let Ok(decoded)
                = <ApproximateReservesGivenPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ApproximateReservesGivenPrice(decoded));
            }
            if let Ok(decoded)
                = <BeforeSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BeforeSwap(decoded));
            }
            if let Ok(decoded)
                = <ConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Configs(decoded));
            }
            if let Ok(decoded)
                = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded)
                = <GetFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetFees(decoded));
            }
            if let Ok(decoded)
                = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded)
                = <GetMaxOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxOrder(decoded));
            }
            if let Ok(decoded)
                = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded)
                = <GetStrategyDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetStrategyData(decoded));
            }
            if let Ok(decoded)
                = <GetSwapInvariantsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSwapInvariants(decoded));
            }
            if let Ok(decoded)
                = <PortfolioCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Portfolio(decoded));
            }
            if let Ok(decoded)
                = <VerifyPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifyPool(decoded));
            }
            if let Ok(decoded)
                = <VerifySwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifySwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NormalStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AfterCreate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproximateReservesGivenPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Configs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStrategyData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapInvariants(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for NormalStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AfterCreate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproximateReservesGivenPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeforeSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Configs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapInvariants(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AfterCreateCall> for NormalStrategyCalls {
        fn from(value: AfterCreateCall) -> Self {
            Self::AfterCreate(value)
        }
    }
    impl ::core::convert::From<ApproximateReservesGivenPriceCall>
    for NormalStrategyCalls {
        fn from(value: ApproximateReservesGivenPriceCall) -> Self {
            Self::ApproximateReservesGivenPrice(value)
        }
    }
    impl ::core::convert::From<BeforeSwapCall> for NormalStrategyCalls {
        fn from(value: BeforeSwapCall) -> Self {
            Self::BeforeSwap(value)
        }
    }
    impl ::core::convert::From<ConfigsCall> for NormalStrategyCalls {
        fn from(value: ConfigsCall) -> Self {
            Self::Configs(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for NormalStrategyCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetFeesCall> for NormalStrategyCalls {
        fn from(value: GetFeesCall) -> Self {
            Self::GetFees(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for NormalStrategyCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetMaxOrderCall> for NormalStrategyCalls {
        fn from(value: GetMaxOrderCall) -> Self {
            Self::GetMaxOrder(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for NormalStrategyCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetStrategyDataCall> for NormalStrategyCalls {
        fn from(value: GetStrategyDataCall) -> Self {
            Self::GetStrategyData(value)
        }
    }
    impl ::core::convert::From<GetSwapInvariantsCall> for NormalStrategyCalls {
        fn from(value: GetSwapInvariantsCall) -> Self {
            Self::GetSwapInvariants(value)
        }
    }
    impl ::core::convert::From<PortfolioCall> for NormalStrategyCalls {
        fn from(value: PortfolioCall) -> Self {
            Self::Portfolio(value)
        }
    }
    impl ::core::convert::From<VerifyPoolCall> for NormalStrategyCalls {
        fn from(value: VerifyPoolCall) -> Self {
            Self::VerifyPool(value)
        }
    }
    impl ::core::convert::From<VerifySwapCall> for NormalStrategyCalls {
        fn from(value: VerifySwapCall) -> Self {
            Self::VerifySwap(value)
        }
    }
    ///Container type for all return fields from the `approximateReservesGivenPrice` function with signature `approximateReservesGivenPrice(bytes)` and selector `0x542767fc`
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
    pub struct ApproximateReservesGivenPriceReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `beforeSwap` function with signature `beforeSwap(uint64,bool,address)` and selector `0xec736854`
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
    pub struct BeforeSwapReturn(pub bool, pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `configs` function with signature `configs(uint64)` and selector `0x34dbc73b`
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
    pub struct ConfigsReturn {
        pub strike_price_wad: u128,
        pub volatility_basis_points: u32,
        pub duration_seconds: u32,
        pub creation_timestamp: u32,
        pub is_perpetual: bool,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    pub struct GetAmountOutReturn {
        pub output: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getFees` function with signature `getFees(uint64)` and selector `0x08e6cca3`
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
    pub struct GetFeesReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
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
    pub struct GetInvariantReturn {
        pub invariant: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `getMaxOrder` function with signature `getMaxOrder(uint64,bool)` and selector `0x441fa735`
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
    pub struct GetMaxOrderReturn(pub Order);
    ///Container type for all return fields from the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
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
    pub struct GetSpotPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,bool,uint256)` and selector `0x452d2f18`
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
    pub struct GetStrategyDataReturn {
        pub strategy_data: ::ethers::core::types::Bytes,
        pub initial_x: ::ethers::core::types::U256,
        pub initial_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getSwapInvariants` function with signature `getSwapInvariants((uint128,uint128,bool,uint64,bool))` and selector `0x45320c8c`
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
    pub struct GetSwapInvariantsReturn(
        pub ::ethers::core::types::I256,
        pub ::ethers::core::types::I256,
    );
    ///Container type for all return fields from the `portfolio` function with signature `portfolio()` and selector `0x16ede016`
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
    pub struct PortfolioReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `verifyPool` function with signature `verifyPool(uint64)` and selector `0xea820059`
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
    pub struct VerifyPoolReturn(pub bool);
    ///Container type for all return fields from the `verifySwap` function with signature `verifySwap(uint64,int256,uint256,uint256)` and selector `0x0c8a113f`
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
    pub struct VerifySwapReturn(pub bool, pub ::ethers::core::types::I256);
}
