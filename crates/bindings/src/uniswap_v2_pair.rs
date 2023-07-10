pub use uniswap_v2_pair::*;
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
pub mod uniswap_v2_pair {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MINIMUM_LIQUIDITY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MINIMUM_LIQUIDITY"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                            constant: ::core::option::Option::Some(true),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserves"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_reserve0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_reserve1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_blockTimestampLast",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("kLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kLast"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                            constant: ::core::option::Option::Some(true),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("skim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("skim"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Out"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sync"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sync"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                            constant: ::core::option::Option::Some(true),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Sync"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Sync"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        112usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        112usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV2PAIR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01`\x0CU4\x80\x15a\0\x15W`\0\x80\xFD[P`@QF\x90\x80`Ra#\x86\x829`@\x80Q\x91\x82\x90\x03`R\x01\x82 \x82\x82\x01\x82R`\n\x83Ri*\xB74\xB9\xBB\xB0\xB8\x10+\x19`\xB1\x1B` \x93\x84\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x84\x01R\x81Q\x80\x84\x01\x91\x90\x91R\x7F\xBF\xCC\x8E\xF9\x8F\xFB\xF7\xB6\xC3\xFE\xC7\xBFQ\x85\xB5f\xB9\x86>5\xA9\xD8:\xCDI\xADh$\xB5\x96\x978\x81\x83\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01R`\x80\x81\x01\x94\x90\x94R0`\xA0\x80\x86\x01\x91\x90\x91R\x81Q\x80\x86\x03\x90\x91\x01\x81R`\xC0\x90\x94\x01\x90R\x82Q\x92\x01\x91\x90\x91 `\x03UP`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\"\x81\x80a\x01\x05`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80cjbxB\x11a\0\xF9W\x80c\xBA\x9AzV\x11a\0\x97W\x80c\xD2\x12 \xA7\x11a\0qW\x80c\xD2\x12 \xA7\x14a\x054W\x80c\xD5\x05\xAC\xCF\x14a\x05<W\x80c\xDDb\xED>\x14a\x05\x8DW\x80c\xFF\xF6\xCA\xE9\x14a\x05\xBBWa\x01\xA9V[\x80c\xBA\x9AzV\x14a\x04\xFEW\x80c\xBC%\xCFw\x14a\x05\x06W\x80c\xC4Z\x01U\x14a\x05,Wa\x01\xA9V[\x80c~\xCE\xBE\0\x11a\0\xD3W\x80c~\xCE\xBE\0\x14a\x04eW\x80c\x89\xAF\xCBD\x14a\x04\x8BW\x80c\x95\xD8\x9BA\x14a\x04\xCAW\x80c\xA9\x05\x9C\xBB\x14a\x04\xD2Wa\x01\xA9V[\x80cjbxB\x14a\x04\x11W\x80cp\xA0\x821\x14a\x047W\x80ctd\xFC=\x14a\x04]Wa\x01\xA9V[\x80c#\xB8r\xDD\x11a\x01fW\x80c6D\xE5\x15\x11a\x01@W\x80c6D\xE5\x15\x14a\x03\xCBW\x80cH\\\xC9U\x14a\x03\xD3W\x80cY\t\xC0\xD5\x14a\x04\x01W\x80cZ=T\x93\x14a\x04\tWa\x01\xA9V[\x80c#\xB8r\xDD\x14a\x03oW\x80c0\xAD\xF8\x1F\x14a\x03\xA5W\x80c1<\xE5g\x14a\x03\xADWa\x01\xA9V[\x80c\x02,\r\x9F\x14a\x01\xAEW\x80c\x06\xFD\xDE\x03\x14a\x02<W\x80c\t\x02\xF1\xAC\x14a\x02\xB9W\x80c\t^\xA7\xB3\x14a\x02\xF1W\x80c\r\xFE\x16\x81\x14a\x031W\x80c\x18\x16\r\xDD\x14a\x03UW[`\0\x80\xFD[a\x02:`\x04\x806\x03`\x80\x81\x10\x15a\x01\xC4W`\0\x80\xFD[\x815\x91` \x81\x015\x91`\x01`\x01`\xA0\x1B\x03`@\x83\x015\x16\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015d\x01\0\0\0\0\x81\x11\x15a\x01\xFBW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x02\rW`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\x02/W`\0\x80\xFD[P\x90\x92P\x90Pa\x05\xC3V[\0[a\x02Da\n\xFEV[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x02~W\x81\x81\x01Q\x83\x82\x01R` \x01a\x02fV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x02\xABW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x02\xC1a\x0B$V[`@\x80Q`\x01`\x01`p\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rc\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x90Q\x90\x81\x90\x03``\x01\x90\xF3[a\x03\x1D`\x04\x806\x03`@\x81\x10\x15a\x03\x07W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0BNV[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x039a\x0BeV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x03]a\x0BtV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x03\x1D`\x04\x806\x03``\x81\x10\x15a\x03\x85W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x0BzV[a\x03]a\x0C\x14V[a\x03\xB5a\x0C8V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x03]a\x0C=V[a\x02:`\x04\x806\x03`@\x81\x10\x15a\x03\xE9W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x0CCV[a\x03]a\x0C\xC7V[a\x03]a\x0C\xCDV[a\x03]`\x04\x806\x03` \x81\x10\x15a\x04'W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xD3V[a\x03]`\x04\x806\x03` \x81\x10\x15a\x04MW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xD3V[a\x03]a\x0F\xE5V[a\x03]`\x04\x806\x03` \x81\x10\x15a\x04{W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xEBV[a\x04\xB1`\x04\x806\x03` \x81\x10\x15a\x04\xA1W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xFDV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[a\x02Da\x13\xA3V[a\x03\x1D`\x04\x806\x03`@\x81\x10\x15a\x04\xE8W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x13\xC5V[a\x03]a\x13\xD2V[a\x02:`\x04\x806\x03` \x81\x10\x15a\x05\x1CW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x13\xD8V[a\x039a\x15CV[a\x039a\x15RV[a\x02:`\x04\x806\x03`\xE0\x81\x10\x15a\x05RW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x81\x015\x90`\xFF`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x015a\x15aV[a\x03]`\x04\x806\x03`@\x81\x10\x15a\x05\xA3W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x17cV[a\x02:a\x17\x80V[`\x0CT`\x01\x14a\x06\x0EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0CU\x84\x15\x15\x80a\x06!WP`\0\x84\x11[a\x06\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a!\x93`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x06ga\x0B$V[P\x91P\x91P\x81`\x01`\x01`p\x1B\x03\x16\x87\x10\x80\x15a\x06\x8CWP\x80`\x01`\x01`p\x1B\x03\x16\x86\x10[a\x06\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`!\x81R` \x01\x80a!\xDC`!\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x06T`\x07T`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x90\x81\x16\x90\x89\x16\x82\x14\x80\x15\x90a\x07\x05WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[a\x07NW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtUniswapV2: INVALID_TO`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x8A\x15a\x07_Wa\x07_\x82\x8A\x8Da\x18\xE2V[\x89\x15a\x07pWa\x07p\x81\x8A\x8Ca\x18\xE2V[\x86\x15a\x08+W\x88`\x01`\x01`\xA0\x1B\x03\x16c\x10\xD1\xE8\\3\x8D\x8D\x8C\x8C`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x96PPPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08&W=`\0\x80>=`\0\xFD[PPPP[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08qW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\x9BW`\0\x80\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x95P`\x01`\x01`\xA0\x1B\x03\x83\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xE7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\t\x11W`\0\x80\xFD[PQ\x92P`\0\x91PP`\x01`\x01`p\x1B\x03\x85\x16\x8A\x90\x03\x83\x11a\t4W`\0a\tCV[\x89\x85`\x01`\x01`p\x1B\x03\x16\x03\x83\x03[\x90P`\0\x89\x85`\x01`\x01`p\x1B\x03\x16\x03\x83\x11a\t`W`\0a\toV[\x89\x85`\x01`\x01`p\x1B\x03\x16\x03\x83\x03[\x90P`\0\x82\x11\x80a\t\x80WP`\0\x81\x11[a\t\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a!\xB8`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a\t\xEFa\t\xD1\x84`\x03c\xFF\xFF\xFF\xFFa\x1A|\x16V[a\t\xE3\x87a\x03\xE8c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[\x90P`\0a\n\x07a\t\xD1\x84`\x03c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90Pa\n8b\x0FB@a\n,`\x01`\x01`p\x1B\x03\x8B\x81\x16\x90\x8B\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90c\xFF\xFF\xFF\xFFa\x1A|\x16V[a\nH\x83\x83c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x10\x15a\n\x8AW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkUniswapV2: K`\xA0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPa\n\x98\x84\x84\x88\x88a\x1B/V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x80\x82\x01\x8D\x90R``\x81\x01\x8C\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x8B\x16\x913\x91\x7F\xD7\x8A\xD9_\xA4l\x99KeQ\xD0\xDA\x85\xFC'_\xE6\x13\xCE7e\x7F\xB8\xD5\xE3\xD10\x84\x01Y\xD8\"\x91\x81\x90\x03`\x80\x01\x90\xA3PP`\x01`\x0CUPPPPPPPPPV[`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i*\xB74\xB9\xBB\xB0\xB8\x10+\x19`\xB1\x1B\x81RP\x81V[`\x08T`\x01`\x01`p\x1B\x03\x80\x82\x16\x92`\x01`p\x1B\x83\x04\x90\x91\x16\x91`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[`\0a\x0B[3\x84\x84a\x1C\xF4V[P`\x01[\x92\x91PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0T\x81V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x14a\x0B\xFFW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 Ta\x0B\xDA\x90\x83c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[a\x0C\n\x84\x84\x84a\x1DVV[P`\x01\x93\x92PPPV[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[`\x12\x81V[`\x03T\x81V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x99W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs*\xB74\xB9\xBB\xB0\xB8+\x19\x1D\x10#'\xA9!$\xA2\"\"\xA7`a\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x07\x80T\x92\x90\x93\x16\x91\x16\x17\x90UV[`\tT\x81V[`\nT\x81V[`\0`\x0CT`\x01\x14a\r W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0C\x81\x90U\x80a\r0a\x0B$V[P`\x06T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x93\x95P\x91\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\r\x84W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\r\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\r\xAEW`\0\x80\xFD[PQ`\x07T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x92\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E\x01W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0E\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E+W`\0\x80\xFD[PQ\x90P`\0a\x0EJ\x83`\x01`\x01`p\x1B\x03\x87\x16c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[\x90P`\0a\x0Eg\x83`\x01`\x01`p\x1B\x03\x87\x16c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[\x90P`\0a\x0Eu\x87\x87a\x1E\x10V[`\0T\x90\x91P\x80a\x0E\xB2Wa\x0E\x9Ea\x03\xE8a\t\xE3a\x0E\x99\x87\x87c\xFF\xFF\xFF\xFFa\x1A|\x16V[a\x1FnV[\x98Pa\x0E\xAD`\0a\x03\xE8a\x1F\xC0V[a\x0F\x01V[a\x0E\xFE`\x01`\x01`p\x1B\x03\x89\x16a\x0E\xCF\x86\x84c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x0E\xD6W\xFE[\x04`\x01`\x01`p\x1B\x03\x89\x16a\x0E\xF1\x86\x85c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x0E\xF8W\xFE[\x04a VV[\x98P[`\0\x89\x11a\x0F@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a\"%`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x0FJ\x8A\x8Aa\x1F\xC0V[a\x0FV\x86\x86\x8A\x8Aa\x1B/V[\x81\x15a\x0F\x86W`\x08Ta\x0F\x82\x90`\x01`\x01`p\x1B\x03\x80\x82\x16\x91`\x01`p\x1B\x90\x04\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[`\x0BU[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x81Q3\x92\x7FL \x9B_\xC8\xADPu\x8F\x13\xE2\xE1\x08\x8B\xA5jV\r\xFFi\n\x1Co\xEF&9OL\x03\x82\x1CO\x92\x82\x90\x03\x01\x90\xA2PP`\x01`\x0CUP\x94\x96\x95PPPPPPV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`\x0BT\x81V[`\x04` R`\0\x90\x81R`@\x90 T\x81V[`\0\x80`\x0CT`\x01\x14a\x10KW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0C\x81\x90U\x80a\x10[a\x0B$V[P`\x06T`\x07T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x94\x96P\x92\x94P`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x91\x16\x91`\0\x91\x84\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x10\xB7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x10\xE1W`\0\x80\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x85\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x11/W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x11CW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x11YW`\0\x80\xFD[PQ0`\0\x90\x81R`\x01` R`@\x81 T\x91\x92Pa\x11x\x88\x88a\x1E\x10V[`\0T\x90\x91P\x80a\x11\x8F\x84\x87c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x11\x96W\xFE[\x04\x9AP\x80a\x11\xAA\x84\x86c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x11\xB1W\xFE[\x04\x99P`\0\x8B\x11\x80\x15a\x11\xC4WP`\0\x8A\x11[a\x11\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a!\xFD`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x12\t0\x84a nV[a\x12\x14\x87\x8D\x8Da\x18\xE2V[a\x12\x1F\x86\x8D\x8Ca\x18\xE2V[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x89\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12eW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12yW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x12\x8FW`\0\x80\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x96P`\x01`\x01`\xA0\x1B\x03\x88\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12\xDBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x13\x05W`\0\x80\xFD[PQ\x93Pa\x13\x15\x85\x85\x8B\x8Ba\x1B/V[\x81\x15a\x13EW`\x08Ta\x13A\x90`\x01`\x01`p\x1B\x03\x80\x82\x16\x91`\x01`p\x1B\x90\x04\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[`\x0BU[`@\x80Q\x8C\x81R` \x81\x01\x8C\x90R\x81Q`\x01`\x01`\xA0\x1B\x03\x8F\x16\x923\x92\x7F\xDC\xCDA/\x0B\x12R\x81\x9C\xB1\xFD3\x0B\x93\"L\xA4&\x12\x89+\xB3\xF4\xF7\x89\x97nm\x81\x93d\x96\x92\x90\x81\x90\x03\x90\x91\x01\x90\xA3PPPPPPPPP`\x01`\x0C\x81\x90UP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*\xA7$\x96\xAB\x19`\xD1\x1B\x81RP\x81V[`\0a\x0B[3\x84\x84a\x1DVV[a\x03\xE8\x81V[`\x0CT`\x01\x14a\x14#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0CU`\x06T`\x07T`\x08T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x94\x90\x93\x16\x92a\x14\xD2\x92\x85\x92\x87\x92a\x14\xCD\x92`\x01`\x01`p\x1B\x03\x16\x91\x85\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x14\x95W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x14\xBFW`\0\x80\xFD[PQ\x90c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[a\x18\xE2V[`\x08T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Qa\x159\x92\x84\x92\x87\x92a\x14\xCD\x92`\x01`p\x1B\x90\x04`\x01`\x01`p\x1B\x03\x16\x91`\x01`\x01`\xA0\x1B\x03\x86\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x14\x95W`\0\x80\xFD[PP`\x01`\x0CUPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x07T`\x01`\x01`\xA0\x1B\x03\x16\x81V[B\x84\x10\x15a\x15\xABW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x11V\x14\x12T\x91Q`r\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01\x80\x82\x01\x90\x92U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x86\x01R\x80\x84\x01\x96\x90\x96R\x95\x8D\x16``\x86\x01R`\x80\x85\x01\x8C\x90R`\xA0\x85\x01\x95\x90\x95R`\xC0\x80\x85\x01\x8B\x90R\x81Q\x80\x86\x03\x90\x91\x01\x81R`\xE0\x85\x01\x82R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1Ba\x01\0\x86\x01Ra\x01\x02\x85\x01\x96\x90\x96Ra\x01\"\x80\x85\x01\x96\x90\x96R\x80Q\x80\x85\x03\x90\x96\x01\x86Ra\x01B\x84\x01\x80\x82R\x86Q\x96\x83\x01\x96\x90\x96 \x95\x83\x90Ra\x01b\x84\x01\x80\x82R\x86\x90R`\xFF\x89\x16a\x01\x82\x85\x01Ra\x01\xA2\x84\x01\x88\x90Ra\x01\xC2\x84\x01\x87\x90RQ\x91\x93\x92a\x01\xE2\x80\x82\x01\x93`\x1F\x19\x81\x01\x92\x81\x90\x03\x90\x91\x01\x90\x85Z\xFA\x15\x80\x15a\x16\xC6W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x16\xFCWP\x88`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x17MW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FUniswapV2: INVALID_SIGNATURE\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x17X\x89\x89\x89a\x1C\xF4V[PPPPPPPPPV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x0CT`\x01\x14a\x17\xCBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0CU`\x06T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Qa\x18\xDB\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x18\x1CW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x180W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18FW`\0\x80\xFD[PQ`\x07T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x18\x93W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x18\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18\xBDW`\0\x80\xFD[PQ`\x08T`\x01`\x01`p\x1B\x03\x80\x82\x16\x91`\x01`p\x1B\x90\x04\x16a\x1B/V[`\x01`\x0CUV[`@\x80Q\x80\x82\x01\x82R`\x19\x81R\x7Ftransfer(address,uint256)\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R`D\x80\x83\x01\x86\x90R\x84Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x84R\x91\x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x81Q`\0\x94``\x94\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x19\x8FW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x19pV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x19\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\xF6V[``\x91P[P\x91P\x91P\x81\x80\x15a\x1A$WP\x80Q\x15\x80a\x1A$WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x1A!W`\0\x80\xFD[PQ[a\x1AuW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUniswapV2: TRANSFER_FAILED\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0\x81\x15\x80a\x1A\x97WPP\x80\x82\x02\x82\x82\x82\x81a\x1A\x94W\xFE[\x04\x14[a\x0B_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x03\x82\x81\x11\x15a\x0B_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`p\x1B\x03\x84\x11\x80\x15\x90a\x1BMWP`\x01`\x01`p\x1B\x03\x83\x11\x15[a\x1B\x94W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrUniswapV2: OVERFLOW`h\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x08Tc\xFF\xFF\xFF\xFFB\x81\x16\x91`\x01`\xE0\x1B\x90\x04\x81\x16\x82\x03\x90\x81\x16\x15\x80\x15\x90a\x1B\xC4WP`\x01`\x01`p\x1B\x03\x84\x16\x15\x15[\x80\x15a\x1B\xD8WP`\x01`\x01`p\x1B\x03\x83\x16\x15\x15[\x15a\x1CIW\x80c\xFF\xFF\xFF\xFF\x16a\x1C\x06\x85a\x1B\xF1\x86a!\x0CV[`\x01`\x01`\xE0\x1B\x03\x16\x90c\xFF\xFF\xFF\xFFa!\x1E\x16V[`\t\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16\x92\x90\x92\x02\x01\x90Uc\xFF\xFF\xFF\xFF\x81\x16a\x1C1\x84a\x1B\xF1\x87a!\x0CV[`\n\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16\x92\x90\x92\x02\x01\x90U[`\x08\x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`p\x1B\x03\x88\x81\x16\x91\x90\x91\x17m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`p\x1B\x19\x16`\x01`p\x1B\x88\x83\x16\x81\x02\x91\x90\x91\x17`\x01`\x01`\xE0\x1B\x03\x16`\x01`\xE0\x1Bc\xFF\xFF\xFF\xFF\x87\x16\x02\x17\x92\x83\x90U`@\x80Q\x84\x84\x16\x81R\x91\x90\x93\x04\x90\x91\x16` \x82\x01R\x81Q\x7F\x1CA\x1E\x9A\x96\xE0q$\x1C/!\xF7rk\x17\xAE\x89\xE3\xCA\xB4\xC7\x8B\xE5\x0E\x06+\x03\xA9\xFF\xFB\xBA\xD1\x92\x91\x81\x90\x03\x90\x91\x01\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x81Q\x85\x81R\x91Q\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x92\x81\x90\x03\x90\x91\x01\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 Ta\x1D\x7F\x90\x82c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x01` R`@\x80\x82 \x93\x90\x93U\x90\x84\x16\x81R Ta\x1D\xB4\x90\x82c\xFF\xFF\xFF\xFFa!C\x16V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x94\x90\x94U\x80Q\x85\x81R\x90Q\x91\x93\x92\x87\x16\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x91\x82\x90\x03\x01\x90\xA3PPPV[`\0\x80`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x01~~X`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1EaW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1EuW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1E\x8BW`\0\x80\xFD[PQ`\x0BT`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x94P\x91\x92P\x90a\x1FZW\x80\x15a\x1FUW`\0a\x1E\xCEa\x0E\x99`\x01`\x01`p\x1B\x03\x88\x81\x16\x90\x88\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90P`\0a\x1E\xDB\x83a\x1FnV[\x90P\x80\x82\x11\x15a\x1FRW`\0a\x1F\ta\x1E\xFA\x84\x84c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\0T\x90c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90P`\0a\x1F.\x83a\x1F\"\x86`\x05c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90c\xFF\xFF\xFF\xFFa!C\x16V[\x90P`\0\x81\x83\x81a\x1F;W\xFE[\x04\x90P\x80\x15a\x1FNWa\x1FN\x87\x82a\x1F\xC0V[PPP[PP[a\x1FfV[\x80\x15a\x1FfW`\0`\x0BU[PP\x92\x91PPV[`\0`\x03\x82\x11\x15a\x1F\xB1WP\x80`\x01`\x02\x82\x04\x01[\x81\x81\x10\x15a\x1F\xABW\x80\x91P`\x02\x81\x82\x85\x81a\x1F\x9AW\xFE[\x04\x01\x81a\x1F\xA3W\xFE[\x04\x90Pa\x1F\x83V[Pa\x1F\xBBV[\x81\x15a\x1F\xBBWP`\x01[\x91\x90PV[`\0Ta\x1F\xD3\x90\x82c\xFF\xFF\xFF\xFFa!C\x16V[`\0\x90\x81U`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`\x01` R`@\x90 Ta\x1F\xFE\x90\x82c\xFF\xFF\xFF\xFFa!C\x16V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x90\x94U\x83Q\x85\x81R\x93Q\x92\x93\x91\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x81\x90\x03\x90\x91\x01\x90\xA3PPV[`\0\x81\x83\x10a eW\x81a gV[\x82[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 Ta \x97\x90\x82c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x91\x90\x91UTa \xC4\x90\x82c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\0\x90\x81U`@\x80Q\x83\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x90\x81\x90\x03` \x01\x90\xA3PPV[`\x01`\x01`p\x1B\x03\x16`\x01`p\x1B\x02\x90V[`\0`\x01`\x01`p\x1B\x03\x82\x16`\x01`\x01`\xE0\x1B\x03\x84\x16\x81a!;W\xFE[\x04\x93\x92PPPV[\x80\x82\x01\x82\x81\x10\x15a\x0B_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD\xFEUniswapV2: INSUFFICIENT_OUTPUT_AMOUNTUniswapV2: INSUFFICIENT_INPUT_AMOUNTUniswapV2: INSUFFICIENT_LIQUIDITYUniswapV2: INSUFFICIENT_LIQUIDITY_BURNEDUniswapV2: INSUFFICIENT_LIQUIDITY_MINTED\xA2ebzzr1X \xB9y\xED\x90\x11\x0F\xB3\xF2\xFA\xA0G\x04{O\\\xFC\xA1\xDF\xBERv\x8Cp\x03\x9Cf\x8F|:\x90\xDD\xA1dsolcC\0\x05\x10\x002EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
    /// The bytecode of the contract.
    pub static UNISWAPV2PAIR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80cjbxB\x11a\0\xF9W\x80c\xBA\x9AzV\x11a\0\x97W\x80c\xD2\x12 \xA7\x11a\0qW\x80c\xD2\x12 \xA7\x14a\x054W\x80c\xD5\x05\xAC\xCF\x14a\x05<W\x80c\xDDb\xED>\x14a\x05\x8DW\x80c\xFF\xF6\xCA\xE9\x14a\x05\xBBWa\x01\xA9V[\x80c\xBA\x9AzV\x14a\x04\xFEW\x80c\xBC%\xCFw\x14a\x05\x06W\x80c\xC4Z\x01U\x14a\x05,Wa\x01\xA9V[\x80c~\xCE\xBE\0\x11a\0\xD3W\x80c~\xCE\xBE\0\x14a\x04eW\x80c\x89\xAF\xCBD\x14a\x04\x8BW\x80c\x95\xD8\x9BA\x14a\x04\xCAW\x80c\xA9\x05\x9C\xBB\x14a\x04\xD2Wa\x01\xA9V[\x80cjbxB\x14a\x04\x11W\x80cp\xA0\x821\x14a\x047W\x80ctd\xFC=\x14a\x04]Wa\x01\xA9V[\x80c#\xB8r\xDD\x11a\x01fW\x80c6D\xE5\x15\x11a\x01@W\x80c6D\xE5\x15\x14a\x03\xCBW\x80cH\\\xC9U\x14a\x03\xD3W\x80cY\t\xC0\xD5\x14a\x04\x01W\x80cZ=T\x93\x14a\x04\tWa\x01\xA9V[\x80c#\xB8r\xDD\x14a\x03oW\x80c0\xAD\xF8\x1F\x14a\x03\xA5W\x80c1<\xE5g\x14a\x03\xADWa\x01\xA9V[\x80c\x02,\r\x9F\x14a\x01\xAEW\x80c\x06\xFD\xDE\x03\x14a\x02<W\x80c\t\x02\xF1\xAC\x14a\x02\xB9W\x80c\t^\xA7\xB3\x14a\x02\xF1W\x80c\r\xFE\x16\x81\x14a\x031W\x80c\x18\x16\r\xDD\x14a\x03UW[`\0\x80\xFD[a\x02:`\x04\x806\x03`\x80\x81\x10\x15a\x01\xC4W`\0\x80\xFD[\x815\x91` \x81\x015\x91`\x01`\x01`\xA0\x1B\x03`@\x83\x015\x16\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015d\x01\0\0\0\0\x81\x11\x15a\x01\xFBW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x02\rW`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\x02/W`\0\x80\xFD[P\x90\x92P\x90Pa\x05\xC3V[\0[a\x02Da\n\xFEV[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x02~W\x81\x81\x01Q\x83\x82\x01R` \x01a\x02fV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x02\xABW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x02\xC1a\x0B$V[`@\x80Q`\x01`\x01`p\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rc\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x90Q\x90\x81\x90\x03``\x01\x90\xF3[a\x03\x1D`\x04\x806\x03`@\x81\x10\x15a\x03\x07W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x0BNV[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x039a\x0BeV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x03]a\x0BtV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x03\x1D`\x04\x806\x03``\x81\x10\x15a\x03\x85W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x0BzV[a\x03]a\x0C\x14V[a\x03\xB5a\x0C8V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x03]a\x0C=V[a\x02:`\x04\x806\x03`@\x81\x10\x15a\x03\xE9W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x0CCV[a\x03]a\x0C\xC7V[a\x03]a\x0C\xCDV[a\x03]`\x04\x806\x03` \x81\x10\x15a\x04'W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xD3V[a\x03]`\x04\x806\x03` \x81\x10\x15a\x04MW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xD3V[a\x03]a\x0F\xE5V[a\x03]`\x04\x806\x03` \x81\x10\x15a\x04{W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xEBV[a\x04\xB1`\x04\x806\x03` \x81\x10\x15a\x04\xA1W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xFDV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[a\x02Da\x13\xA3V[a\x03\x1D`\x04\x806\x03`@\x81\x10\x15a\x04\xE8W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x13\xC5V[a\x03]a\x13\xD2V[a\x02:`\x04\x806\x03` \x81\x10\x15a\x05\x1CW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x13\xD8V[a\x039a\x15CV[a\x039a\x15RV[a\x02:`\x04\x806\x03`\xE0\x81\x10\x15a\x05RW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x81\x015\x90``\x81\x015\x90`\xFF`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x015a\x15aV[a\x03]`\x04\x806\x03`@\x81\x10\x15a\x05\xA3W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x17cV[a\x02:a\x17\x80V[`\x0CT`\x01\x14a\x06\x0EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0CU\x84\x15\x15\x80a\x06!WP`\0\x84\x11[a\x06\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a!\x93`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x06ga\x0B$V[P\x91P\x91P\x81`\x01`\x01`p\x1B\x03\x16\x87\x10\x80\x15a\x06\x8CWP\x80`\x01`\x01`p\x1B\x03\x16\x86\x10[a\x06\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`!\x81R` \x01\x80a!\xDC`!\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x06T`\x07T`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x90\x81\x16\x90\x89\x16\x82\x14\x80\x15\x90a\x07\x05WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[a\x07NW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtUniswapV2: INVALID_TO`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x8A\x15a\x07_Wa\x07_\x82\x8A\x8Da\x18\xE2V[\x89\x15a\x07pWa\x07p\x81\x8A\x8Ca\x18\xE2V[\x86\x15a\x08+W\x88`\x01`\x01`\xA0\x1B\x03\x16c\x10\xD1\xE8\\3\x8D\x8D\x8C\x8C`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x96PPPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08&W=`\0\x80>=`\0\xFD[PPPP[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08qW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\x9BW`\0\x80\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x95P`\x01`\x01`\xA0\x1B\x03\x83\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xE7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\t\x11W`\0\x80\xFD[PQ\x92P`\0\x91PP`\x01`\x01`p\x1B\x03\x85\x16\x8A\x90\x03\x83\x11a\t4W`\0a\tCV[\x89\x85`\x01`\x01`p\x1B\x03\x16\x03\x83\x03[\x90P`\0\x89\x85`\x01`\x01`p\x1B\x03\x16\x03\x83\x11a\t`W`\0a\toV[\x89\x85`\x01`\x01`p\x1B\x03\x16\x03\x83\x03[\x90P`\0\x82\x11\x80a\t\x80WP`\0\x81\x11[a\t\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a!\xB8`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a\t\xEFa\t\xD1\x84`\x03c\xFF\xFF\xFF\xFFa\x1A|\x16V[a\t\xE3\x87a\x03\xE8c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[\x90P`\0a\n\x07a\t\xD1\x84`\x03c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90Pa\n8b\x0FB@a\n,`\x01`\x01`p\x1B\x03\x8B\x81\x16\x90\x8B\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90c\xFF\xFF\xFF\xFFa\x1A|\x16V[a\nH\x83\x83c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x10\x15a\n\x8AW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkUniswapV2: K`\xA0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPa\n\x98\x84\x84\x88\x88a\x1B/V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x80\x82\x01\x8D\x90R``\x81\x01\x8C\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x8B\x16\x913\x91\x7F\xD7\x8A\xD9_\xA4l\x99KeQ\xD0\xDA\x85\xFC'_\xE6\x13\xCE7e\x7F\xB8\xD5\xE3\xD10\x84\x01Y\xD8\"\x91\x81\x90\x03`\x80\x01\x90\xA3PP`\x01`\x0CUPPPPPPPPPV[`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i*\xB74\xB9\xBB\xB0\xB8\x10+\x19`\xB1\x1B\x81RP\x81V[`\x08T`\x01`\x01`p\x1B\x03\x80\x82\x16\x92`\x01`p\x1B\x83\x04\x90\x91\x16\x91`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[`\0a\x0B[3\x84\x84a\x1C\xF4V[P`\x01[\x92\x91PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0T\x81V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x14a\x0B\xFFW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 Ta\x0B\xDA\x90\x83c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[a\x0C\n\x84\x84\x84a\x1DVV[P`\x01\x93\x92PPPV[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[`\x12\x81V[`\x03T\x81V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x99W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs*\xB74\xB9\xBB\xB0\xB8+\x19\x1D\x10#'\xA9!$\xA2\"\"\xA7`a\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x07\x80T\x92\x90\x93\x16\x91\x16\x17\x90UV[`\tT\x81V[`\nT\x81V[`\0`\x0CT`\x01\x14a\r W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0C\x81\x90U\x80a\r0a\x0B$V[P`\x06T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x93\x95P\x91\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\r\x84W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\r\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\r\xAEW`\0\x80\xFD[PQ`\x07T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x92\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E\x01W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0E\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E+W`\0\x80\xFD[PQ\x90P`\0a\x0EJ\x83`\x01`\x01`p\x1B\x03\x87\x16c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[\x90P`\0a\x0Eg\x83`\x01`\x01`p\x1B\x03\x87\x16c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[\x90P`\0a\x0Eu\x87\x87a\x1E\x10V[`\0T\x90\x91P\x80a\x0E\xB2Wa\x0E\x9Ea\x03\xE8a\t\xE3a\x0E\x99\x87\x87c\xFF\xFF\xFF\xFFa\x1A|\x16V[a\x1FnV[\x98Pa\x0E\xAD`\0a\x03\xE8a\x1F\xC0V[a\x0F\x01V[a\x0E\xFE`\x01`\x01`p\x1B\x03\x89\x16a\x0E\xCF\x86\x84c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x0E\xD6W\xFE[\x04`\x01`\x01`p\x1B\x03\x89\x16a\x0E\xF1\x86\x85c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x0E\xF8W\xFE[\x04a VV[\x98P[`\0\x89\x11a\x0F@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a\"%`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x0FJ\x8A\x8Aa\x1F\xC0V[a\x0FV\x86\x86\x8A\x8Aa\x1B/V[\x81\x15a\x0F\x86W`\x08Ta\x0F\x82\x90`\x01`\x01`p\x1B\x03\x80\x82\x16\x91`\x01`p\x1B\x90\x04\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[`\x0BU[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x81Q3\x92\x7FL \x9B_\xC8\xADPu\x8F\x13\xE2\xE1\x08\x8B\xA5jV\r\xFFi\n\x1Co\xEF&9OL\x03\x82\x1CO\x92\x82\x90\x03\x01\x90\xA2PP`\x01`\x0CUP\x94\x96\x95PPPPPPV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`\x0BT\x81V[`\x04` R`\0\x90\x81R`@\x90 T\x81V[`\0\x80`\x0CT`\x01\x14a\x10KW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0C\x81\x90U\x80a\x10[a\x0B$V[P`\x06T`\x07T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x94\x96P\x92\x94P`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x91\x16\x91`\0\x91\x84\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x10\xB7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x10\xE1W`\0\x80\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x85\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x11/W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x11CW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x11YW`\0\x80\xFD[PQ0`\0\x90\x81R`\x01` R`@\x81 T\x91\x92Pa\x11x\x88\x88a\x1E\x10V[`\0T\x90\x91P\x80a\x11\x8F\x84\x87c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x11\x96W\xFE[\x04\x9AP\x80a\x11\xAA\x84\x86c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x81a\x11\xB1W\xFE[\x04\x99P`\0\x8B\x11\x80\x15a\x11\xC4WP`\0\x8A\x11[a\x11\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a!\xFD`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x12\t0\x84a nV[a\x12\x14\x87\x8D\x8Da\x18\xE2V[a\x12\x1F\x86\x8D\x8Ca\x18\xE2V[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x89\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12eW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12yW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x12\x8FW`\0\x80\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x96P`\x01`\x01`\xA0\x1B\x03\x88\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12\xDBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x13\x05W`\0\x80\xFD[PQ\x93Pa\x13\x15\x85\x85\x8B\x8Ba\x1B/V[\x81\x15a\x13EW`\x08Ta\x13A\x90`\x01`\x01`p\x1B\x03\x80\x82\x16\x91`\x01`p\x1B\x90\x04\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[`\x0BU[`@\x80Q\x8C\x81R` \x81\x01\x8C\x90R\x81Q`\x01`\x01`\xA0\x1B\x03\x8F\x16\x923\x92\x7F\xDC\xCDA/\x0B\x12R\x81\x9C\xB1\xFD3\x0B\x93\"L\xA4&\x12\x89+\xB3\xF4\xF7\x89\x97nm\x81\x93d\x96\x92\x90\x81\x90\x03\x90\x91\x01\x90\xA3PPPPPPPPP`\x01`\x0C\x81\x90UP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e*\xA7$\x96\xAB\x19`\xD1\x1B\x81RP\x81V[`\0a\x0B[3\x84\x84a\x1DVV[a\x03\xE8\x81V[`\x0CT`\x01\x14a\x14#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0CU`\x06T`\x07T`\x08T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x94\x90\x93\x16\x92a\x14\xD2\x92\x85\x92\x87\x92a\x14\xCD\x92`\x01`\x01`p\x1B\x03\x16\x91\x85\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x14\x95W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x14\xBFW`\0\x80\xFD[PQ\x90c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[a\x18\xE2V[`\x08T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Qa\x159\x92\x84\x92\x87\x92a\x14\xCD\x92`\x01`p\x1B\x90\x04`\x01`\x01`p\x1B\x03\x16\x91`\x01`\x01`\xA0\x1B\x03\x86\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x14\x95W`\0\x80\xFD[PP`\x01`\x0CUPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x07T`\x01`\x01`\xA0\x1B\x03\x16\x81V[B\x84\x10\x15a\x15\xABW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x11V\x14\x12T\x91Q`r\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01\x80\x82\x01\x90\x92U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x86\x01R\x80\x84\x01\x96\x90\x96R\x95\x8D\x16``\x86\x01R`\x80\x85\x01\x8C\x90R`\xA0\x85\x01\x95\x90\x95R`\xC0\x80\x85\x01\x8B\x90R\x81Q\x80\x86\x03\x90\x91\x01\x81R`\xE0\x85\x01\x82R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1Ba\x01\0\x86\x01Ra\x01\x02\x85\x01\x96\x90\x96Ra\x01\"\x80\x85\x01\x96\x90\x96R\x80Q\x80\x85\x03\x90\x96\x01\x86Ra\x01B\x84\x01\x80\x82R\x86Q\x96\x83\x01\x96\x90\x96 \x95\x83\x90Ra\x01b\x84\x01\x80\x82R\x86\x90R`\xFF\x89\x16a\x01\x82\x85\x01Ra\x01\xA2\x84\x01\x88\x90Ra\x01\xC2\x84\x01\x87\x90RQ\x91\x93\x92a\x01\xE2\x80\x82\x01\x93`\x1F\x19\x81\x01\x92\x81\x90\x03\x90\x91\x01\x90\x85Z\xFA\x15\x80\x15a\x16\xC6W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x16\xFCWP\x88`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x17MW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FUniswapV2: INVALID_SIGNATURE\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x17X\x89\x89\x89a\x1C\xF4V[PPPPPPPPPV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x0CT`\x01\x14a\x17\xCBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15[\x9A\\\xDD\xD8\\\x15\x8C\x8E\x88\x13\x13\xD0\xD2\xD1Q`z\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x0CU`\x06T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Qa\x18\xDB\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x18\x1CW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x180W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18FW`\0\x80\xFD[PQ`\x07T`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x18\x93W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x18\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18\xBDW`\0\x80\xFD[PQ`\x08T`\x01`\x01`p\x1B\x03\x80\x82\x16\x91`\x01`p\x1B\x90\x04\x16a\x1B/V[`\x01`\x0CUV[`@\x80Q\x80\x82\x01\x82R`\x19\x81R\x7Ftransfer(address,uint256)\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R`D\x80\x83\x01\x86\x90R\x84Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x84R\x91\x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x81Q`\0\x94``\x94\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x19\x8FW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x19pV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x19\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\xF6V[``\x91P[P\x91P\x91P\x81\x80\x15a\x1A$WP\x80Q\x15\x80a\x1A$WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x1A!W`\0\x80\xFD[PQ[a\x1AuW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUniswapV2: TRANSFER_FAILED\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0\x81\x15\x80a\x1A\x97WPP\x80\x82\x02\x82\x82\x82\x81a\x1A\x94W\xFE[\x04\x14[a\x0B_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x03\x82\x81\x11\x15a\x0B_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`p\x1B\x03\x84\x11\x80\x15\x90a\x1BMWP`\x01`\x01`p\x1B\x03\x83\x11\x15[a\x1B\x94W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrUniswapV2: OVERFLOW`h\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x08Tc\xFF\xFF\xFF\xFFB\x81\x16\x91`\x01`\xE0\x1B\x90\x04\x81\x16\x82\x03\x90\x81\x16\x15\x80\x15\x90a\x1B\xC4WP`\x01`\x01`p\x1B\x03\x84\x16\x15\x15[\x80\x15a\x1B\xD8WP`\x01`\x01`p\x1B\x03\x83\x16\x15\x15[\x15a\x1CIW\x80c\xFF\xFF\xFF\xFF\x16a\x1C\x06\x85a\x1B\xF1\x86a!\x0CV[`\x01`\x01`\xE0\x1B\x03\x16\x90c\xFF\xFF\xFF\xFFa!\x1E\x16V[`\t\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16\x92\x90\x92\x02\x01\x90Uc\xFF\xFF\xFF\xFF\x81\x16a\x1C1\x84a\x1B\xF1\x87a!\x0CV[`\n\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16\x92\x90\x92\x02\x01\x90U[`\x08\x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`p\x1B\x03\x88\x81\x16\x91\x90\x91\x17m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`p\x1B\x19\x16`\x01`p\x1B\x88\x83\x16\x81\x02\x91\x90\x91\x17`\x01`\x01`\xE0\x1B\x03\x16`\x01`\xE0\x1Bc\xFF\xFF\xFF\xFF\x87\x16\x02\x17\x92\x83\x90U`@\x80Q\x84\x84\x16\x81R\x91\x90\x93\x04\x90\x91\x16` \x82\x01R\x81Q\x7F\x1CA\x1E\x9A\x96\xE0q$\x1C/!\xF7rk\x17\xAE\x89\xE3\xCA\xB4\xC7\x8B\xE5\x0E\x06+\x03\xA9\xFF\xFB\xBA\xD1\x92\x91\x81\x90\x03\x90\x91\x01\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x81Q\x85\x81R\x91Q\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x92\x81\x90\x03\x90\x91\x01\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 Ta\x1D\x7F\x90\x82c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x01` R`@\x80\x82 \x93\x90\x93U\x90\x84\x16\x81R Ta\x1D\xB4\x90\x82c\xFF\xFF\xFF\xFFa!C\x16V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x94\x90\x94U\x80Q\x85\x81R\x90Q\x91\x93\x92\x87\x16\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x91\x82\x90\x03\x01\x90\xA3PPPV[`\0\x80`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x01~~X`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1EaW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1EuW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1E\x8BW`\0\x80\xFD[PQ`\x0BT`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x94P\x91\x92P\x90a\x1FZW\x80\x15a\x1FUW`\0a\x1E\xCEa\x0E\x99`\x01`\x01`p\x1B\x03\x88\x81\x16\x90\x88\x16c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90P`\0a\x1E\xDB\x83a\x1FnV[\x90P\x80\x82\x11\x15a\x1FRW`\0a\x1F\ta\x1E\xFA\x84\x84c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\0T\x90c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90P`\0a\x1F.\x83a\x1F\"\x86`\x05c\xFF\xFF\xFF\xFFa\x1A|\x16V[\x90c\xFF\xFF\xFF\xFFa!C\x16V[\x90P`\0\x81\x83\x81a\x1F;W\xFE[\x04\x90P\x80\x15a\x1FNWa\x1FN\x87\x82a\x1F\xC0V[PPP[PP[a\x1FfV[\x80\x15a\x1FfW`\0`\x0BU[PP\x92\x91PPV[`\0`\x03\x82\x11\x15a\x1F\xB1WP\x80`\x01`\x02\x82\x04\x01[\x81\x81\x10\x15a\x1F\xABW\x80\x91P`\x02\x81\x82\x85\x81a\x1F\x9AW\xFE[\x04\x01\x81a\x1F\xA3W\xFE[\x04\x90Pa\x1F\x83V[Pa\x1F\xBBV[\x81\x15a\x1F\xBBWP`\x01[\x91\x90PV[`\0Ta\x1F\xD3\x90\x82c\xFF\xFF\xFF\xFFa!C\x16V[`\0\x90\x81U`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`\x01` R`@\x90 Ta\x1F\xFE\x90\x82c\xFF\xFF\xFF\xFFa!C\x16V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x90\x94U\x83Q\x85\x81R\x93Q\x92\x93\x91\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x81\x90\x03\x90\x91\x01\x90\xA3PPV[`\0\x81\x83\x10a eW\x81a gV[\x82[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 Ta \x97\x90\x82c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x91\x90\x91UTa \xC4\x90\x82c\xFF\xFF\xFF\xFFa\x1A\xDF\x16V[`\0\x90\x81U`@\x80Q\x83\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x90\x81\x90\x03` \x01\x90\xA3PPV[`\x01`\x01`p\x1B\x03\x16`\x01`p\x1B\x02\x90V[`\0`\x01`\x01`p\x1B\x03\x82\x16`\x01`\x01`\xE0\x1B\x03\x84\x16\x81a!;W\xFE[\x04\x93\x92PPPV[\x80\x82\x01\x82\x81\x10\x15a\x0B_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD\xFEUniswapV2: INSUFFICIENT_OUTPUT_AMOUNTUniswapV2: INSUFFICIENT_INPUT_AMOUNTUniswapV2: INSUFFICIENT_LIQUIDITYUniswapV2: INSUFFICIENT_LIQUIDITY_BURNEDUniswapV2: INSUFFICIENT_LIQUIDITY_MINTED\xA2ebzzr1X \xB9y\xED\x90\x11\x0F\xB3\xF2\xFA\xA0G\x04{O\\\xFC\xA1\xDF\xBERv\x8Cp\x03\x9Cf\x8F|:\x90\xDD\xA1dsolcC\0\x05\x10\x002";
    /// The deployed bytecode of the contract.
    pub static UNISWAPV2PAIR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniswapV2Pair<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV2Pair<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV2Pair<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV2Pair<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV2Pair<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapV2Pair))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV2Pair<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNISWAPV2PAIR_ABI.clone(),
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
                UNISWAPV2PAIR_ABI.clone(),
                UNISWAPV2PAIR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINIMUM_LIQUIDITY` (0xba9a7a56) function
        pub fn minimum_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 154, 122, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function
        pub fn permit_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x89afcb44) function
        pub fn burn(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([137, 175, 203, 68], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
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
        ///Calls the contract's `getReserves` (0x0902f1ac) function
        pub fn get_reserves(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128, u32)> {
            self.0
                .method_hash([9, 2, 241, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            token_0: ::ethers::core::types::Address,
            token_1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (token_0, token_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kLast` (0x7464fc3d) function
        pub fn k_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([116, 100, 252, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x6a627842) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 98, 120, 66], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
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
        ///Calls the contract's `price1CumulativeLast` (0x5a3d5493) function
        pub fn price_1_cumulative_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 61, 84, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `skim` (0xbc25cf77) function
        pub fn skim(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 37, 207, 119], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x022c0d9f) function
        pub fn swap(
            &self,
            amount_0_out: ::ethers::core::types::U256,
            amount_1_out: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 44, 13, 159], (amount_0_out, amount_1_out, to, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sync` (0xfff6cae9) function
        pub fn sync(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 246, 202, 233], ())
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
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `Sync` event
        pub fn sync_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SyncFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UniswapV2PairEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniswapV2Pair<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Burn", abi = "Burn(address,uint256,uint256,address)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,uint256,uint256,uint256,uint256,address)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0_in: ::ethers::core::types::U256,
        pub amount_1_in: ::ethers::core::types::U256,
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "Sync", abi = "Sync(uint112,uint112)")]
    pub struct SyncFilter {
        pub reserve_0: u128,
        pub reserve_1: u128,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UniswapV2PairEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        MintFilter(MintFilter),
        SwapFilter(SwapFilter),
        SyncFilter(SyncFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for UniswapV2PairEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = SyncFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::SyncFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UniswapV2PairEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SyncFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for UniswapV2PairEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for UniswapV2PairEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for UniswapV2PairEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for UniswapV2PairEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<SyncFilter> for UniswapV2PairEvents {
        fn from(value: SyncFilter) -> Self {
            Self::SyncFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for UniswapV2PairEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `MINIMUM_LIQUIDITY` function with signature `MINIMUM_LIQUIDITY()` and selector `0xba9a7a56`
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
    #[ethcall(name = "MINIMUM_LIQUIDITY", abi = "MINIMUM_LIQUIDITY()")]
    pub struct MinimumLiquidityCall;
    ///Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `burn` function with signature `burn(address)` and selector `0x89afcb44`
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
    #[ethcall(name = "burn", abi = "burn(address)")]
    pub struct BurnCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
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
    ///Container type for all input parameters for the `getReserves` function with signature `getReserves()` and selector `0x0902f1ac`
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
    #[ethcall(name = "getReserves", abi = "getReserves()")]
    pub struct GetReservesCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `kLast` function with signature `kLast()` and selector `0x7464fc3d`
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
    #[ethcall(name = "kLast", abi = "kLast()")]
    pub struct KlastCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address)` and selector `0x6a627842`
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
    #[ethcall(name = "mint", abi = "mint(address)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
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
    ///Container type for all input parameters for the `skim` function with signature `skim(address)` and selector `0xbc25cf77`
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
    #[ethcall(name = "skim", abi = "skim(address)")]
    pub struct SkimCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(uint256,uint256,address,bytes)` and selector `0x022c0d9f`
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
    #[ethcall(name = "swap", abi = "swap(uint256,uint256,address,bytes)")]
    pub struct SwapCall {
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `sync` function with signature `sync()` and selector `0xfff6cae9`
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
    #[ethcall(name = "sync", abi = "sync()")]
    pub struct SyncCall;
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
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UniswapV2PairCalls {
        DomainSeparator(DomainSeparatorCall),
        MinimumLiquidity(MinimumLiquidityCall),
        PermitTypehash(PermitTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Factory(FactoryCall),
        GetReserves(GetReservesCall),
        Initialize(InitializeCall),
        Klast(KlastCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Price0CumulativeLast(Price0CumulativeLastCall),
        Price1CumulativeLast(Price1CumulativeLastCall),
        Skim(SkimCall),
        Swap(SwapCall),
        Symbol(SymbolCall),
        Sync(SyncCall),
        Token0(Token0Call),
        Token1(Token1Call),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV2PairCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <MinimumLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MinimumLiquidity(decoded));
            }
            if let Ok(decoded)
                = <PermitTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PermitTypehash(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded)
                = <GetReservesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserves(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <KlastCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Klast(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded)
                = <Price0CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Price0CumulativeLast(decoded));
            }
            if let Ok(decoded)
                = <Price1CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Price1CumulativeLast(decoded));
            }
            if let Ok(decoded)
                = <SkimCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Skim(decoded));
            }
            if let Ok(decoded)
                = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <SyncCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sync(decoded));
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
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapV2PairCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermitTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Klast(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Price0CumulativeLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Price1CumulativeLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Skim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sync(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UniswapV2PairCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermitTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Klast(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Price0CumulativeLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Price1CumulativeLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Skim(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sync(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token1(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for UniswapV2PairCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<MinimumLiquidityCall> for UniswapV2PairCalls {
        fn from(value: MinimumLiquidityCall) -> Self {
            Self::MinimumLiquidity(value)
        }
    }
    impl ::core::convert::From<PermitTypehashCall> for UniswapV2PairCalls {
        fn from(value: PermitTypehashCall) -> Self {
            Self::PermitTypehash(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for UniswapV2PairCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for UniswapV2PairCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for UniswapV2PairCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for UniswapV2PairCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for UniswapV2PairCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for UniswapV2PairCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetReservesCall> for UniswapV2PairCalls {
        fn from(value: GetReservesCall) -> Self {
            Self::GetReserves(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for UniswapV2PairCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<KlastCall> for UniswapV2PairCalls {
        fn from(value: KlastCall) -> Self {
            Self::Klast(value)
        }
    }
    impl ::core::convert::From<MintCall> for UniswapV2PairCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for UniswapV2PairCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for UniswapV2PairCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for UniswapV2PairCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<Price0CumulativeLastCall> for UniswapV2PairCalls {
        fn from(value: Price0CumulativeLastCall) -> Self {
            Self::Price0CumulativeLast(value)
        }
    }
    impl ::core::convert::From<Price1CumulativeLastCall> for UniswapV2PairCalls {
        fn from(value: Price1CumulativeLastCall) -> Self {
            Self::Price1CumulativeLast(value)
        }
    }
    impl ::core::convert::From<SkimCall> for UniswapV2PairCalls {
        fn from(value: SkimCall) -> Self {
            Self::Skim(value)
        }
    }
    impl ::core::convert::From<SwapCall> for UniswapV2PairCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for UniswapV2PairCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<SyncCall> for UniswapV2PairCalls {
        fn from(value: SyncCall) -> Self {
            Self::Sync(value)
        }
    }
    impl ::core::convert::From<Token0Call> for UniswapV2PairCalls {
        fn from(value: Token0Call) -> Self {
            Self::Token0(value)
        }
    }
    impl ::core::convert::From<Token1Call> for UniswapV2PairCalls {
        fn from(value: Token1Call) -> Self {
            Self::Token1(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for UniswapV2PairCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for UniswapV2PairCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for UniswapV2PairCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MINIMUM_LIQUIDITY` function with signature `MINIMUM_LIQUIDITY()` and selector `0xba9a7a56`
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
    pub struct MinimumLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
    pub struct PermitTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `burn` function with signature `burn(address)` and selector `0x89afcb44`
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
    pub struct BurnReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
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
    ///Container type for all return fields from the `getReserves` function with signature `getReserves()` and selector `0x0902f1ac`
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
    pub struct GetReservesReturn {
        pub reserve_0: u128,
        pub reserve_1: u128,
        pub block_timestamp_last: u32,
    }
    ///Container type for all return fields from the `kLast` function with signature `kLast()` and selector `0x7464fc3d`
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
    pub struct KlastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mint` function with signature `mint(address)` and selector `0x6a627842`
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
    pub struct MintReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
