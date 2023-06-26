pub use uniswap_v2_router_01::*;
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
pub mod uniswap_v2_router_01 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_WETH"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("WETH"),
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
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addLiquidity"),
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
                                name: ::std::borrow::ToOwned::to_owned("amountADesired"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountBDesired"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountBMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidityETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addLiquidityETH"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountTokenDesired",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
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
                    ::std::borrow::ToOwned::to_owned("getAmountIn"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAmountIn"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountIn"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountOut"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountsIn"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAmountsIn"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountsOut"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAmountsOut"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("quote"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountB"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeLiquidity"),
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
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountBMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeLiquidityETH"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityETHWithPermit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeLiquidityETHWithPermit",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approveMax"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityWithPermit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeLiquidityWithPermit",),
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
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountBMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approveMax"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapETHForExactTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapETHForExactTokens",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactETHForTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapExactETHForTokens",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapExactTokensForETH",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapExactTokensForTokens",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapTokensForExactETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapTokensForExactETH",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapTokensForExactTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapTokensForExactTokens",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amounts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV2ROUTER01_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`@Qb\0H\xCA8\x03\x80b\0H\xCA\x839\x81\x81\x01`@R`@\x81\x10\x15a\0\xADWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x91\x1B\x16`\xA0R`\x80Q``\x1C`\xA0Q``\x1CaG\x13b\0\x01\xB7`\09\x80a\x01(R\x80a\x15\xBAR\x80a\x15\xF5R\x80a\x17#R\x80a\x19AR\x80a\x1D9R\x80a\x1E\xBDR\x80a \x03R\x80a \xC2R\x80a#yR\x80a'-R\x80a*9R\x80a*\x8FR\x80a*\xC3R\x80a+nR\x80a-\xD2R\x80a/\x15R\x80a/\xD4RP\x80a\x17\xB1R\x80a\x18\x88R\x80a\x1A>R\x80a\x1AwR\x80a\x1B\xE9R\x80a\x1D\xC7R\x80a\x1FPR\x80a \xF4R\x80a\"\xC9R\x80a#\xF6R\x80a&\xB2R\x80a&\xDBR\x80a'\x0BR\x80a(\xAFR\x80a*mR\x80a.eR\x80a0\x06R\x80a9eR\x80a9\xA8R\x80a>&R\x80a?6R\x80a@\x18RPaG\x13`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x18W`\x005`\xE0\x1C\x80c\x88\x03\xDB\xEE\x11a\0\xA0W\x80c\xD0l\xA6\x1F\x11a\0dW\x80c\xD0l\xA6\x1F\x14a\x10\x1CW\x80c\xDE\xD98*\x14a\x11\xE6W\x80c\xE8\xE37\0\x14a\x12\xB8W\x80c\xF3\x05\xD7\x19\x14a\x13\x97W\x80c\xFB;\xDBA\x14a\x14\x08Wa\x01QV[\x80c\x88\x03\xDB\xEE\x14a\x0CrW\x80c\xAD\\FH\x14a\x0E\x1DW\x80c\xADa]\xEC\x14a\x0E\x82W\x80c\xBA\xA2\xAB\xDE\x14a\x0F\x17W\x80c\xC4Z\x01U\x14a\x0F\xD3Wa\x01QV[\x80c!\x95\x99\\\x11a\0\xE7W\x80c!\x95\x99\\\x14a\x06\xDAW\x80c8\xED\x179\x14a\x07\xB7W\x80cJ%\xD9J\x14a\tbW\x80c\x7F\xF3j\xB5\x14a\x0B\rW\x80c\x85\xF8\xC2Y\x14a\x02nWa\x01QV[\x80c\x02u\x1C\xEC\x14a\x01\xA3W\x80c\x05MP\xD4\x14a\x02nW\x80c\x18\xCB\xAF\xE5\x14a\x03\x15W\x80c\x1F\0\xCAt\x14a\x05\x10Wa\x01QV[6a\x01QW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01OW\xFE[\0[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FUnknown signature and no fallbac`D\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\x01\xE3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03`\xC0\x81\x10\x15a\x02%WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a\x15mV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[4\x80\x15a\x02\xAEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x03\x03`\x04\x806\x03``\x81\x10\x15a\x02\xF0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x805\x90` \x81\x015\x90`@\x015a\x16\xBEV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x03UWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\x03\x97WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x03\xF9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04HWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\xA6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x16\xD3V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x81\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x04\xFCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xE4V[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05PWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`@\x81\x10\x15a\x05\x92WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x05\xEFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06>WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x06\x9CWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\x1A7\x94PPPPPV[4\x80\x15a\x07\x1AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03a\x01`\x81\x10\x15a\x07]WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x81\x015\x90`\xE0\x81\x015\x15\x15\x90`\xFFa\x01\0\x82\x015\x16\x90a\x01 \x81\x015\x90a\x01@\x015a\x1AmV[4\x80\x15a\x07\xF7WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\x089WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x08\x9BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x08\xEAWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\tHWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1B\x9EV[4\x80\x15a\t\xA2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\t\xE4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\nFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\n\x95WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\n\xF3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1C\xE9V[a\x04\xC0`\x04\x806\x03`\x80\x81\x10\x15a\x0BNWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x0B\xABWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x0B\xFAWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x0CXWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1EuV[4\x80\x15a\x0C\xB2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\x0C\xF4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\rVWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\r\xA5WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x0E\x03WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\"~V[4\x80\x15a\x0E]WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x0Efa#wV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x0E\xC2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x03\x03`\x04\x806\x03``\x81\x10\x15a\x0F\x04WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x805\x90` \x81\x015\x90`@\x015a#\x9BV[4\x80\x15a\x0FWWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03`\xE0\x81\x10\x15a\x0F\x99WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x015a#\xA8V[4\x80\x15a\x10\x13WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x0Efa&\xB0V[4\x80\x15a\x10\\WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`@\x81\x10\x15a\x10\x9EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x10\xFBWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x11JWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x11\xA8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa&\xD4\x94PPPPPV[4\x80\x15a\x12&WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03a\x01@\x81\x10\x15a\x12iWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x81\x015\x15\x15\x90`\xFF`\xE0\x82\x015\x16\x90a\x01\0\x81\x015\x90a\x01 \x015a'\x01V[4\x80\x15a\x12\xF8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x13y`\x04\x806\x03a\x01\0\x81\x10\x15a\x13;WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x91`\xC0\x82\x015\x16\x90`\xE0\x015a(LV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x13y`\x04\x806\x03`\xC0\x81\x10\x15a\x13\xD8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a)\xEAV[a\x04\xC0`\x04\x806\x03`\x80\x81\x10\x15a\x14IWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x14\xA6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x14\xF5WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x15SWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a-\x8AV[`\0\x80\x82B\x81\x10\x15a\x15\xB4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x15\xE3\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A\x8A0\x8Aa#\xA8V[\x90\x93P\x91Pa\x15\xF3\x89\x86\x85a1\xA5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x90WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x16\xA4W=`\0\x80>=`\0\xFD[PPPPa\x16\xB2\x85\x83a3:V[P\x96P\x96\x94PPPPPV[`\0a\x16\xCB\x84\x84\x84a42V[\x94\x93PPPPV[``\x81B\x81\x10\x15a\x17\x19W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x17SW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xACW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x18\n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\"\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x18\x1DW\xFE[` \x02` \x01\x01Q\x10\x15a\x18bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aFD`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x19\0\x86\x86`\0\x81\x81\x10a\x18rW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x163a\x18\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A`\0\x81\x81\x10a\x18\xB4W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8B\x8B`\x01\x81\x81\x10a\x18\xD1W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16a6nV[\x85`\0\x81Q\x81\x10a\x18\xF3W\xFE[` \x02` \x01\x01Qa7.V[a\x19?\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x92Pa8\xB6\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`\x01\x85Q\x03\x81Q\x81\x10a\x19~W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xF3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x1A\x07W=`\0\x80>=`\0\xFD[PPPPa\x1A,\x84\x83`\x01\x85Q\x03\x81Q\x81\x10a\x1A\x1FW\xFE[` \x02` \x01\x01Qa3:V[P\x96\x95PPPPPPV[``a\x1Ad\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a;3V[\x90P[\x92\x91PPV[`\0\x80`\0a\x1A\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8F\x8Fa6nV[\x90P`\0\x87a\x1A\xACW\x8Ca\x1A\xB0V[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x1B]WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x1BqW=`\0\x80>=`\0\xFD[PPPPa\x1B\x84\x8F\x8F\x8F\x8F\x8F\x8F\x8Fa#\xA8V[\x80\x94P\x81\x95PPPPP\x9BP\x9B\x99PPPPPPPPPPV[``\x81B\x81\x10\x15a\x1B\xE4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1CB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\"\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x1CUW\xFE[` \x02` \x01\x01Q\x10\x15a\x1C\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aFD`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x1C\xAA\x86\x86`\0\x81\x81\x10a\x18rW\xFE[a\x1A,\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa8\xB6\x91PPV[``\x81B\x81\x10\x15a\x1D/W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x1DiW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xC2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1E \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;3\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a\x1E0W\xFE[` \x02` \x01\x01Q\x11\x15a\x18bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aE\xB4`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[``\x81B\x81\x10\x15a\x1E\xBBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a\x1E\xF2W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1FKW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1F\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\"\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x1F\xBCW\xFE[` \x02` \x01\x01Q\x10\x15a \x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aFD`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a =W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a \xA7WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a \xBBW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa!=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a! W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8A\x8A`\x01\x81\x81\x10a\x18\xD1W\xFE[\x84`\0\x81Q\x81\x10a!JW\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xD8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a!\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\"-WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQa\"5W\xFE[a\"t\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa8\xB6\x91PPV[P\x95\x94PPPPPV[``\x81B\x81\x10\x15a\"\xC4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a#\"\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;3\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a#2W\xFE[` \x02` \x01\x01Q\x11\x15a\x1C\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aE\xB4`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x16\xCB\x84\x84\x84a<kV[`\0\x80\x82B\x81\x10\x15a#\xEFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a$\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x8Ca6nV[`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01\x81\x90R`D\x82\x01\x8D\x90R\x91Q\x92\x93P\x90\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a$\xAEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a$\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a%\x03WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PP`@\x80Qc\"k\xF2\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82Q`\0\x93\x84\x93\x92\x86\x16\x92c\x89\xAF\xCBD\x92`$\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x87\x87\x80;\x15\x80\x15a%\x87WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a%\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`@\x81\x10\x15a%\xDCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90P`\0a%\xF6\x8E\x8Ea=\x17V[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x8E`\x01`\x01`\xA0\x1B\x03\x16\x14a&\x19W\x81\x83a&\x1CV[\x82\x82[\x90\x97P\x95P\x8A\x87\x10\x15a&`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aE\xFB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x89\x86\x10\x15a&\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xC1`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPP\x97P\x97\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``a\x1Ad\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a5\"V[`\0\x80`\0a'Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a6nV[\x90P`\0\x87a'`W\x8Ca'dV[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a(\x11WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a(%W=`\0\x80>=`\0\xFD[PPPPa(7\x8E\x8E\x8E\x8E\x8E\x8Ea\x15mV[\x90\x9F\x90\x9EP\x9CPPPPPPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a(\x95W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a(\xA3\x8C\x8C\x8C\x8C\x8C\x8Ca=\xF5V[\x90\x94P\x92P`\0a(\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x8Ea6nV[\x90Pa(\xE3\x8D3\x83\x88a7.V[a(\xEF\x8C3\x83\x87a7.V[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)~WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a)\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a)\xD3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x94\x9D\x93\x9CP\x93\x9AP\x91\x98PPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a*3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a*a\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B4\x8C\x8Ca=\xF5V[\x90\x94P\x92P`\0a*\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a6nV[\x90Pa*\xC1\x8B3\x83\x88a7.V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a+SWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a+gW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x82\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,#WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a,7W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a,xWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQa,\x80W\xFE[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\x0FWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a-#W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a-dWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x92P4\x84\x10\x15a-|Wa-|3\x854\x03a3:V[PP\x96P\x96P\x96\x93PPPPV[``\x81B\x81\x10\x15a-\xD0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a.\x07W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a.`W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a.\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;3\x92PPPV[\x91P4\x82`\0\x81Q\x81\x10a.\xCEW\xFE[` \x02` \x01\x01Q\x11\x15a/\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aE\xB4`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a/OW\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a/\xB9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a/\xCDW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a! W\xFE[\x84`\0\x81Q\x81\x10a0?W\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xCDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a0\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a1\"WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQa1*W\xFE[a1i\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa8\xB6\x91PPV[\x81`\0\x81Q\x81\x10a1vW\xFE[` \x02` \x01\x01Q4\x11\x15a\"tWa\"t3\x83`\0\x81Q\x81\x10a1\x96W\xFE[` \x02` \x01\x01Q4\x03a3:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a2\"W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a2\x03V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a2\x84W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a2\x89V[``\x91P[P\x91P\x91P\x81\x80\x15a2\xE2WP\x80Q\x15\x80a2\xE2WP\x80\x80` \x01\x90Q` \x81\x10\x15a2\xDFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[a33W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FTransferHelper: TRANSFER_FAILED\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a3\x86W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a3gV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a3\xE8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a3\xEDV[``\x91P[PP\x90P\x80a4-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80aF!`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80\x84\x11a4rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aF\x93`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a4\x82WP`\0\x82\x11[a4\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aE'`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a4\xD1\x85a\x03\xE5c\xFF\xFF\xFF\xFFaA-\x16V[\x90P`\0a4\xE5\x82\x85c\xFF\xFF\xFF\xFFaA-\x16V[\x90P`\0a5\x0B\x83a4\xFF\x88a\x03\xE8c\xFF\xFF\xFF\xFFaA-\x16V[\x90c\xFF\xFF\xFF\xFFaA\x90\x16V[\x90P\x80\x82\x81a5\x16W\xFE[\x04\x97\x96PPPPPPPV[```\x02\x82Q\x10\x15a5{W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a5\x93W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\xBDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\0\x81Q\x81\x10a5\xCEW\xFE[` \x02` \x01\x01\x81\x81RPP`\0[`\x01\x83Q\x03\x81\x10\x15a6fW`\0\x80a6 \x87\x86\x85\x81Q\x81\x10a5\xFCW\xFE[` \x02` \x01\x01Q\x87\x86`\x01\x01\x81Q\x81\x10a6\x13W\xFE[` \x02` \x01\x01QaA\xDFV[\x91P\x91Pa6B\x84\x84\x81Q\x81\x10a63W\xFE[` \x02` \x01\x01Q\x83\x83a42V[\x84\x84`\x01\x01\x81Q\x81\x10a6QW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a5\xDDV[P\x93\x92PPPV[`\0\x80`\0a6}\x85\x85a=\x17V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x7F\xB0\xAE1u\xF4\xAE\xF6y\xD8\xE5\xEE\xC9\x1E~\x16\xDE:m\xE1XbwN$\x85\xDBG\x12 P\xF5`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a7\xB3W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a7\x94V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a8\x15W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a8\x1AV[``\x91P[P\x91P\x91P\x81\x80\x15a8sWP\x80Q\x15\x80a8sWP\x80\x80` \x01\x90Q` \x81\x10\x15a8pWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[a8\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80aFo`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0[`\x01\x83Q\x03\x81\x10\x15a;-W`\0\x80\x84\x83\x81Q\x81\x10a8\xD4W\xFE[` \x02` \x01\x01Q\x85\x84`\x01\x01\x81Q\x81\x10a8\xEBW\xFE[` \x02` \x01\x01Q\x91P\x91P`\0a9\x03\x83\x83a=\x17V[P\x90P`\0\x87\x85`\x01\x01\x81Q\x81\x10a9\x17W\xFE[` \x02` \x01\x01Q\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a9EW\x82`\0a9IV[`\0\x83[\x91P\x91P`\0`\x02\x8AQ\x03\x88\x10a9`W\x88a9\xA1V[a9\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x8C\x8B`\x02\x01\x81Q\x81\x10a9\x94W\xFE[` \x02` \x01\x01Qa6nV[\x90Pa9\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88a6nV[`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x84\x84\x84`\0`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a:\x0BW` \x82\x01\x81\x806\x837\x01\x90P[P`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a:|W\x81\x81\x01Q\x83\x82\x01R` \x01a:dV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a:\xA9W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a;\x02WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a;\x16W=`\0\x80>=`\0\xFD[PP`\x01\x90\x99\x01\x98Pa8\xB9\x97PPPPPPPPV[PPPPV[```\x02\x82Q\x10\x15a;\x8CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a;\xA4W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a;\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\x01\x83Q\x03\x81Q\x81\x10a;\xE2W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81Q`\0\x19\x01[\x80\x15a6fW`\0\x80a<$\x87\x86`\x01\x86\x03\x81Q\x81\x10a<\x10W\xFE[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a6\x13W\xFE[\x91P\x91Pa<F\x84\x84\x81Q\x81\x10a<7W\xFE[` \x02` \x01\x01Q\x83\x83aC\x0FV[\x84`\x01\x85\x03\x81Q\x81\x10a<UW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\0\x19\x01a;\xF4V[`\0\x80\x84\x11a<\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aE\x8F`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a<\xBBWP`\0\x82\x11[a<\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aE'`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82a=\x07\x85\x84c\xFF\xFF\xFF\xFFaA-\x16V[\x81a=\x0EW\xFE[\x04\x94\x93PPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a=kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aD\x9C`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a=\x8BW\x82\x84a=\x8EV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a=\xEEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`@\x80Qc\xE6\xA49\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x91Q`\0\x92\x83\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xE6\xA49\x05\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a>\xA6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a>\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a>\xFBWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a@\x10W`@\x80Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xC9\xC6S\x96\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a?\xB8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a?\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a@\rWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PP[`\0\x80a@>\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8BaA\xDFV[\x91P\x91P\x81`\0\x14\x80\x15a@PWP\x80\x15[\x15a@`W\x87\x93P\x86\x92PaA V[`\0a@m\x89\x84\x84a<kV[\x90P\x87\x81\x11a@\xC0W\x85\x81\x10\x15a@\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xC1`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x88\x94P\x92P\x82aA\x1EV[`\0a@\xCD\x89\x84\x86a<kV[\x90P\x89\x81\x11\x15a@\xD9W\xFE[\x87\x81\x10\x15aA\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aE\xFB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x94P\x87\x93P[P[PP\x96P\x96\x94PPPPPV[`\0\x81\x15\x80aAHWPP\x80\x82\x02\x82\x82\x82\x81aAEW\xFE[\x04\x14[a\x1AgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x01\x82\x81\x10\x15a\x1AgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80`\0aA\xEE\x85\x85a=\x17V[P\x90P`\0\x80aA\xFF\x88\x88\x88a6nV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aBnWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15aB\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15aB\xC3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14aB\xFDW\x80\x82aC\0V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x84\x11aCOW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80aDp`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15aC_WP`\0\x82\x11[aC\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aE'`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0aC\xBEa\x03\xE8aC\xB2\x86\x88c\xFF\xFF\xFF\xFFaA-\x16V[\x90c\xFF\xFF\xFF\xFFaA-\x16V[\x90P`\0aC\xD8a\x03\xE5aC\xB2\x86\x89c\xFF\xFF\xFF\xFFaC\xFF\x16V[\x90PaC\xF5`\x01\x82\x84\x81aC\xE8W\xFE[\x04\x90c\xFF\xFF\xFF\xFFaA\x90\x16V[\x96\x95PPPPPPV[\x80\x82\x03\x82\x81\x11\x15a\x1AgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD\xFEABI calldata decoding: invalid dUniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNTUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Router: INSUFFICIENT_B_AMOUNTABI calldata decoding: invalid hEther sent to non-payable functiUniswapV2Library: INSUFFICIENT_LIQUIDITYTarget contract does not containCalldata too short\0\0\0\0\0\0\0\0\0\0\0\0\0\0UniswapV2Library: INSUFFICIENT_AMOUNTUniswapV2Router: EXCESSIVE_INPUT_AMOUNTUniswapV2Router: INVALID_PATH\0\0\0UniswapV2Router: INSUFFICIENT_A_AMOUNTTransferHelper: ETH_TRANSFER_FAILEDUniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNTTransferHelper: TRANSFER_FROM_FAILEDUniswapV2Library: INSUFFICIENT_INPUT_AMOUNTUniswapV2Router: EXPIRED\0\0\0\0\0\0\0\0\xA2dipfsX\"\x12 D\xA8\xB0 9\xF0\xE4\x94\x9D\xC9\xED\xCD\x02\x16\xC44\x05#h+GI@+B\xD0\x82\xCE>\xB7\xA9\x9EdsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static UNISWAPV2ROUTER01_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x18W`\x005`\xE0\x1C\x80c\x88\x03\xDB\xEE\x11a\0\xA0W\x80c\xD0l\xA6\x1F\x11a\0dW\x80c\xD0l\xA6\x1F\x14a\x10\x1CW\x80c\xDE\xD98*\x14a\x11\xE6W\x80c\xE8\xE37\0\x14a\x12\xB8W\x80c\xF3\x05\xD7\x19\x14a\x13\x97W\x80c\xFB;\xDBA\x14a\x14\x08Wa\x01QV[\x80c\x88\x03\xDB\xEE\x14a\x0CrW\x80c\xAD\\FH\x14a\x0E\x1DW\x80c\xADa]\xEC\x14a\x0E\x82W\x80c\xBA\xA2\xAB\xDE\x14a\x0F\x17W\x80c\xC4Z\x01U\x14a\x0F\xD3Wa\x01QV[\x80c!\x95\x99\\\x11a\0\xE7W\x80c!\x95\x99\\\x14a\x06\xDAW\x80c8\xED\x179\x14a\x07\xB7W\x80cJ%\xD9J\x14a\tbW\x80c\x7F\xF3j\xB5\x14a\x0B\rW\x80c\x85\xF8\xC2Y\x14a\x02nWa\x01QV[\x80c\x02u\x1C\xEC\x14a\x01\xA3W\x80c\x05MP\xD4\x14a\x02nW\x80c\x18\xCB\xAF\xE5\x14a\x03\x15W\x80c\x1F\0\xCAt\x14a\x05\x10Wa\x01QV[6a\x01QW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01OW\xFE[\0[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FUnknown signature and no fallbac`D\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\x01\xE3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03`\xC0\x81\x10\x15a\x02%WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a\x15mV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[4\x80\x15a\x02\xAEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x03\x03`\x04\x806\x03``\x81\x10\x15a\x02\xF0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x805\x90` \x81\x015\x90`@\x015a\x16\xBEV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x03UWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\x03\x97WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x03\xF9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04HWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\xA6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x16\xD3V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x81\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x04\xFCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xE4V[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05PWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`@\x81\x10\x15a\x05\x92WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x05\xEFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06>WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x06\x9CWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\x1A7\x94PPPPPV[4\x80\x15a\x07\x1AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03a\x01`\x81\x10\x15a\x07]WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x81\x015\x90`\xE0\x81\x015\x15\x15\x90`\xFFa\x01\0\x82\x015\x16\x90a\x01 \x81\x015\x90a\x01@\x015a\x1AmV[4\x80\x15a\x07\xF7WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\x089WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x08\x9BWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x08\xEAWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\tHWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1B\x9EV[4\x80\x15a\t\xA2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\t\xE4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\nFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\n\x95WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\n\xF3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1C\xE9V[a\x04\xC0`\x04\x806\x03`\x80\x81\x10\x15a\x0BNWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x0B\xABWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x0B\xFAWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x0CXWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1EuV[4\x80\x15a\x0C\xB2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`\xA0\x81\x10\x15a\x0C\xF4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\rVWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\r\xA5WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x0E\x03WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\"~V[4\x80\x15a\x0E]WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x0Efa#wV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x0E\xC2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x03\x03`\x04\x806\x03``\x81\x10\x15a\x0F\x04WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x805\x90` \x81\x015\x90`@\x015a#\x9BV[4\x80\x15a\x0FWWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03`\xE0\x81\x10\x15a\x0F\x99WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x015a#\xA8V[4\x80\x15a\x10\x13WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x0Efa&\xB0V[4\x80\x15a\x10\\WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04\xC0`\x04\x806\x03`@\x81\x10\x15a\x10\x9EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x10\xFBWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x11JWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x11\xA8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa&\xD4\x94PPPPPV[4\x80\x15a\x12&WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02U`\x04\x806\x03a\x01@\x81\x10\x15a\x12iWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x81\x015\x15\x15\x90`\xFF`\xE0\x82\x015\x16\x90a\x01\0\x81\x015\x90a\x01 \x015a'\x01V[4\x80\x15a\x12\xF8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` aE\x07\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x13y`\x04\x806\x03a\x01\0\x81\x10\x15a\x13;WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x91`\xC0\x82\x015\x16\x90`\xE0\x015a(LV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x13y`\x04\x806\x03`\xC0\x81\x10\x15a\x13\xD8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a)\xEAV[a\x04\xC0`\x04\x806\x03`\x80\x81\x10\x15a\x14IWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x14\xA6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`*`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Ri\x18]\x18H\x1B\xD9\x99\x9C\xD9]`\xB2\x1B`dR\x90`\x84\x90\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x14\xF5WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aD\xE7\x839\x81Q\x91R`D\x90\x81Rj2\xB0\xB2\x1087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x15SWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`+`$R`\0\x80Q` aDP\x839\x81Q\x91R`D\x90\x81Rj0\xBA0\x9087\xB4\xB7:2\xB9`\xA9\x1B`dR\x90`\x84\x90\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a-\x8AV[`\0\x80\x82B\x81\x10\x15a\x15\xB4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x15\xE3\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A\x8A0\x8Aa#\xA8V[\x90\x93P\x91Pa\x15\xF3\x89\x86\x85a1\xA5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x90WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x16\xA4W=`\0\x80>=`\0\xFD[PPPPa\x16\xB2\x85\x83a3:V[P\x96P\x96\x94PPPPPV[`\0a\x16\xCB\x84\x84\x84a42V[\x94\x93PPPPV[``\x81B\x81\x10\x15a\x17\x19W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x17SW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xACW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x18\n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\"\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x18\x1DW\xFE[` \x02` \x01\x01Q\x10\x15a\x18bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aFD`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x19\0\x86\x86`\0\x81\x81\x10a\x18rW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x163a\x18\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A`\0\x81\x81\x10a\x18\xB4W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8B\x8B`\x01\x81\x81\x10a\x18\xD1W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16a6nV[\x85`\0\x81Q\x81\x10a\x18\xF3W\xFE[` \x02` \x01\x01Qa7.V[a\x19?\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x92Pa8\xB6\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`\x01\x85Q\x03\x81Q\x81\x10a\x19~W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xF3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x1A\x07W=`\0\x80>=`\0\xFD[PPPPa\x1A,\x84\x83`\x01\x85Q\x03\x81Q\x81\x10a\x1A\x1FW\xFE[` \x02` \x01\x01Qa3:V[P\x96\x95PPPPPPV[``a\x1Ad\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a;3V[\x90P[\x92\x91PPV[`\0\x80`\0a\x1A\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8F\x8Fa6nV[\x90P`\0\x87a\x1A\xACW\x8Ca\x1A\xB0V[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x1B]WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x1BqW=`\0\x80>=`\0\xFD[PPPPa\x1B\x84\x8F\x8F\x8F\x8F\x8F\x8F\x8Fa#\xA8V[\x80\x94P\x81\x95PPPPP\x9BP\x9B\x99PPPPPPPPPPV[``\x81B\x81\x10\x15a\x1B\xE4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1CB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\"\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x1CUW\xFE[` \x02` \x01\x01Q\x10\x15a\x1C\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aFD`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x1C\xAA\x86\x86`\0\x81\x81\x10a\x18rW\xFE[a\x1A,\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa8\xB6\x91PPV[``\x81B\x81\x10\x15a\x1D/W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x1DiW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xC2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1E \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;3\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a\x1E0W\xFE[` \x02` \x01\x01Q\x11\x15a\x18bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aE\xB4`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[``\x81B\x81\x10\x15a\x1E\xBBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a\x1E\xF2W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1FKW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1F\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\"\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x1F\xBCW\xFE[` \x02` \x01\x01Q\x10\x15a \x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aFD`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a =W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a \xA7WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a \xBBW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa!=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a! W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8A\x8A`\x01\x81\x81\x10a\x18\xD1W\xFE[\x84`\0\x81Q\x81\x10a!JW\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xD8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a!\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\"-WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQa\"5W\xFE[a\"t\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa8\xB6\x91PPV[P\x95\x94PPPPPV[``\x81B\x81\x10\x15a\"\xC4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a#\"\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;3\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a#2W\xFE[` \x02` \x01\x01Q\x11\x15a\x1C\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aE\xB4`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x16\xCB\x84\x84\x84a<kV[`\0\x80\x82B\x81\x10\x15a#\xEFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a$\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x8Ca6nV[`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01\x81\x90R`D\x82\x01\x8D\x90R\x91Q\x92\x93P\x90\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a$\xAEWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a$\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a%\x03WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PP`@\x80Qc\"k\xF2\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82Q`\0\x93\x84\x93\x92\x86\x16\x92c\x89\xAF\xCBD\x92`$\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x87\x87\x80;\x15\x80\x15a%\x87WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a%\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`@\x81\x10\x15a%\xDCWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90P`\0a%\xF6\x8E\x8Ea=\x17V[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x8E`\x01`\x01`\xA0\x1B\x03\x16\x14a&\x19W\x81\x83a&\x1CV[\x82\x82[\x90\x97P\x95P\x8A\x87\x10\x15a&`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aE\xFB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x89\x86\x10\x15a&\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xC1`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPP\x97P\x97\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``a\x1Ad\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a5\"V[`\0\x80`\0a'Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a6nV[\x90P`\0\x87a'`W\x8Ca'dV[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a(\x11WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a(%W=`\0\x80>=`\0\xFD[PPPPa(7\x8E\x8E\x8E\x8E\x8E\x8Ea\x15mV[\x90\x9F\x90\x9EP\x9CPPPPPPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a(\x95W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a(\xA3\x8C\x8C\x8C\x8C\x8C\x8Ca=\xF5V[\x90\x94P\x92P`\0a(\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x8Ea6nV[\x90Pa(\xE3\x8D3\x83\x88a7.V[a(\xEF\x8C3\x83\x87a7.V[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)~WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a)\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a)\xD3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x94\x9D\x93\x9CP\x93\x9AP\x91\x98PPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a*3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a*a\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B4\x8C\x8Ca=\xF5V[\x90\x94P\x92P`\0a*\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a6nV[\x90Pa*\xC1\x8B3\x83\x88a7.V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a+SWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a+gW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x82\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,#WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a,7W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a,xWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQa,\x80W\xFE[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\x0FWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a-#W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a-dWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ\x92P4\x84\x10\x15a-|Wa-|3\x854\x03a3:V[PP\x96P\x96P\x96\x93PPPPV[``\x81B\x81\x10\x15a-\xD0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aF\xBE\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a.\x07W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a.`W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aE\xDB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a.\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;3\x92PPPV[\x91P4\x82`\0\x81Q\x81\x10a.\xCEW\xFE[` \x02` \x01\x01Q\x11\x15a/\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aE\xB4`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a/OW\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a/\xB9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a/\xCDW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a! W\xFE[\x84`\0\x81Q\x81\x10a0?W\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xCDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a0\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a1\"WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQa1*W\xFE[a1i\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa8\xB6\x91PPV[\x81`\0\x81Q\x81\x10a1vW\xFE[` \x02` \x01\x01Q4\x11\x15a\"tWa\"t3\x83`\0\x81Q\x81\x10a1\x96W\xFE[` \x02` \x01\x01Q4\x03a3:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a2\"W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a2\x03V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a2\x84W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a2\x89V[``\x91P[P\x91P\x91P\x81\x80\x15a2\xE2WP\x80Q\x15\x80a2\xE2WP\x80\x80` \x01\x90Q` \x81\x10\x15a2\xDFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[a33W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FTransferHelper: TRANSFER_FAILED\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a3\x86W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a3gV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a3\xE8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a3\xEDV[``\x91P[PP\x90P\x80a4-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80aF!`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80\x84\x11a4rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aF\x93`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a4\x82WP`\0\x82\x11[a4\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aE'`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a4\xD1\x85a\x03\xE5c\xFF\xFF\xFF\xFFaA-\x16V[\x90P`\0a4\xE5\x82\x85c\xFF\xFF\xFF\xFFaA-\x16V[\x90P`\0a5\x0B\x83a4\xFF\x88a\x03\xE8c\xFF\xFF\xFF\xFFaA-\x16V[\x90c\xFF\xFF\xFF\xFFaA\x90\x16V[\x90P\x80\x82\x81a5\x16W\xFE[\x04\x97\x96PPPPPPPV[```\x02\x82Q\x10\x15a5{W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a5\x93W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\xBDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\0\x81Q\x81\x10a5\xCEW\xFE[` \x02` \x01\x01\x81\x81RPP`\0[`\x01\x83Q\x03\x81\x10\x15a6fW`\0\x80a6 \x87\x86\x85\x81Q\x81\x10a5\xFCW\xFE[` \x02` \x01\x01Q\x87\x86`\x01\x01\x81Q\x81\x10a6\x13W\xFE[` \x02` \x01\x01QaA\xDFV[\x91P\x91Pa6B\x84\x84\x81Q\x81\x10a63W\xFE[` \x02` \x01\x01Q\x83\x83a42V[\x84\x84`\x01\x01\x81Q\x81\x10a6QW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a5\xDDV[P\x93\x92PPPV[`\0\x80`\0a6}\x85\x85a=\x17V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x7F\xB0\xAE1u\xF4\xAE\xF6y\xD8\xE5\xEE\xC9\x1E~\x16\xDE:m\xE1XbwN$\x85\xDBG\x12 P\xF5`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a7\xB3W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a7\x94V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a8\x15W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a8\x1AV[``\x91P[P\x91P\x91P\x81\x80\x15a8sWP\x80Q\x15\x80a8sWP\x80\x80` \x01\x90Q` \x81\x10\x15a8pWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ[a8\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80aFo`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0[`\x01\x83Q\x03\x81\x10\x15a;-W`\0\x80\x84\x83\x81Q\x81\x10a8\xD4W\xFE[` \x02` \x01\x01Q\x85\x84`\x01\x01\x81Q\x81\x10a8\xEBW\xFE[` \x02` \x01\x01Q\x91P\x91P`\0a9\x03\x83\x83a=\x17V[P\x90P`\0\x87\x85`\x01\x01\x81Q\x81\x10a9\x17W\xFE[` \x02` \x01\x01Q\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a9EW\x82`\0a9IV[`\0\x83[\x91P\x91P`\0`\x02\x8AQ\x03\x88\x10a9`W\x88a9\xA1V[a9\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x8C\x8B`\x02\x01\x81Q\x81\x10a9\x94W\xFE[` \x02` \x01\x01Qa6nV[\x90Pa9\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88a6nV[`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x84\x84\x84`\0`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a:\x0BW` \x82\x01\x81\x806\x837\x01\x90P[P`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a:|W\x81\x81\x01Q\x83\x82\x01R` \x01a:dV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a:\xA9W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a;\x02WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a;\x16W=`\0\x80>=`\0\xFD[PP`\x01\x90\x99\x01\x98Pa8\xB9\x97PPPPPPPPV[PPPPV[```\x02\x82Q\x10\x15a;\x8CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a;\xA4W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a;\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\x01\x83Q\x03\x81Q\x81\x10a;\xE2W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81Q`\0\x19\x01[\x80\x15a6fW`\0\x80a<$\x87\x86`\x01\x86\x03\x81Q\x81\x10a<\x10W\xFE[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a6\x13W\xFE[\x91P\x91Pa<F\x84\x84\x81Q\x81\x10a<7W\xFE[` \x02` \x01\x01Q\x83\x83aC\x0FV[\x84`\x01\x85\x03\x81Q\x81\x10a<UW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\0\x19\x01a;\xF4V[`\0\x80\x84\x11a<\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aE\x8F`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a<\xBBWP`\0\x82\x11[a<\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aE'`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82a=\x07\x85\x84c\xFF\xFF\xFF\xFFaA-\x16V[\x81a=\x0EW\xFE[\x04\x94\x93PPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a=kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aD\x9C`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a=\x8BW\x82\x84a=\x8EV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a=\xEEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`@\x80Qc\xE6\xA49\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x91Q`\0\x92\x83\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xE6\xA49\x05\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a>\xA6WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a>\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a>\xFBWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a@\x10W`@\x80Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xC9\xC6S\x96\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a?\xB8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a?\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a@\rWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[PP[`\0\x80a@>\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8BaA\xDFV[\x91P\x91P\x81`\0\x14\x80\x15a@PWP\x80\x15[\x15a@`W\x87\x93P\x86\x92PaA V[`\0a@m\x89\x84\x84a<kV[\x90P\x87\x81\x11a@\xC0W\x85\x81\x10\x15a@\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xC1`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x88\x94P\x92P\x82aA\x1EV[`\0a@\xCD\x89\x84\x86a<kV[\x90P\x89\x81\x11\x15a@\xD9W\xFE[\x87\x81\x10\x15aA\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aE\xFB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x94P\x87\x93P[P[PP\x96P\x96\x94PPPPPV[`\0\x81\x15\x80aAHWPP\x80\x82\x02\x82\x82\x82\x81aAEW\xFE[\x04\x14[a\x1AgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x01\x82\x81\x10\x15a\x1AgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80`\0aA\xEE\x85\x85a=\x17V[P\x90P`\0\x80aA\xFF\x88\x88\x88a6nV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aBnWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` aEO\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15aB\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15aB\xC3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$R`\0\x80Q` aEo\x839\x81Q\x91R`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14aB\xFDW\x80\x82aC\0V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x84\x11aCOW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80aDp`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15aC_WP`\0\x82\x11[aC\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aE'`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0aC\xBEa\x03\xE8aC\xB2\x86\x88c\xFF\xFF\xFF\xFFaA-\x16V[\x90c\xFF\xFF\xFF\xFFaA-\x16V[\x90P`\0aC\xD8a\x03\xE5aC\xB2\x86\x89c\xFF\xFF\xFF\xFFaC\xFF\x16V[\x90PaC\xF5`\x01\x82\x84\x81aC\xE8W\xFE[\x04\x90c\xFF\xFF\xFF\xFFaA\x90\x16V[\x96\x95PPPPPPV[\x80\x82\x03\x82\x81\x11\x15a\x1AgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD\xFEABI calldata decoding: invalid dUniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNTUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Router: INSUFFICIENT_B_AMOUNTABI calldata decoding: invalid hEther sent to non-payable functiUniswapV2Library: INSUFFICIENT_LIQUIDITYTarget contract does not containCalldata too short\0\0\0\0\0\0\0\0\0\0\0\0\0\0UniswapV2Library: INSUFFICIENT_AMOUNTUniswapV2Router: EXCESSIVE_INPUT_AMOUNTUniswapV2Router: INVALID_PATH\0\0\0UniswapV2Router: INSUFFICIENT_A_AMOUNTTransferHelper: ETH_TRANSFER_FAILEDUniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNTTransferHelper: TRANSFER_FROM_FAILEDUniswapV2Library: INSUFFICIENT_INPUT_AMOUNTUniswapV2Router: EXPIRED\0\0\0\0\0\0\0\0\xA2dipfsX\"\x12 D\xA8\xB0 9\xF0\xE4\x94\x9D\xC9\xED\xCD\x02\x16\xC44\x05#h+GI@+B\xD0\x82\xCE>\xB7\xA9\x9EdsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static UNISWAPV2ROUTER01_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct UniswapV2Router01<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV2Router01<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV2Router01<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV2Router01<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV2Router01<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapV2Router01))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV2Router01<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UNISWAPV2ROUTER01_ABI.clone(),
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
                UNISWAPV2ROUTER01_ABI.clone(),
                UNISWAPV2ROUTER01_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidity` (0xe8e33700) function
        pub fn add_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            amount_a_desired: ::ethers::core::types::U256,
            amount_b_desired: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [232, 227, 55, 0],
                    (
                        token_a,
                        token_b,
                        amount_a_desired,
                        amount_b_desired,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidityETH` (0xf305d719) function
        pub fn add_liquidity_eth(
            &self,
            token: ::ethers::core::types::Address,
            amount_token_desired: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [243, 5, 215, 25],
                    (
                        token,
                        amount_token_desired,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountIn` (0x85f8c259) function
        pub fn get_amount_in(
            &self,
            amount_out: ::ethers::core::types::U256,
            reserve_in: ::ethers::core::types::U256,
            reserve_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 248, 194, 89], (amount_out, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x054d50d4) function
        pub fn get_amount_out(
            &self,
            amount_in: ::ethers::core::types::U256,
            reserve_in: ::ethers::core::types::U256,
            reserve_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([5, 77, 80, 212], (amount_in, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountsIn` (0x1f00ca74) function
        pub fn get_amounts_in(
            &self,
            amount_out: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([31, 0, 202, 116], (amount_out, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountsOut` (0xd06ca61f) function
        pub fn get_amounts_out(
            &self,
            amount_in: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([208, 108, 166, 31], (amount_in, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0xad615dec) function
        pub fn quote(
            &self,
            amount_a: ::ethers::core::types::U256,
            reserve_a: ::ethers::core::types::U256,
            reserve_b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 97, 93, 236], (amount_a, reserve_a, reserve_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidity` (0xbaa2abde) function
        pub fn remove_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [186, 162, 171, 222],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETH` (0x02751cec) function
        pub fn remove_liquidity_eth(
            &self,
            token: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [2, 117, 28, 236],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETHWithPermit` (0xded9382a) function
        pub fn remove_liquidity_eth_with_permit(
            &self,
            token: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [222, 217, 56, 42],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityWithPermit` (0x2195995c) function
        pub fn remove_liquidity_with_permit(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [33, 149, 153, 92],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapETHForExactTokens` (0xfb3bdb41) function
        pub fn swap_eth_for_exact_tokens(
            &self,
            amount_out: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([251, 59, 219, 65], (amount_out, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactETHForTokens` (0x7ff36ab5) function
        pub fn swap_exact_eth_for_tokens(
            &self,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([127, 243, 106, 181], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForETH` (0x18cbafe5) function
        pub fn swap_exact_tokens_for_eth(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [24, 203, 175, 229],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokens` (0x38ed1739) function
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [56, 237, 23, 57],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactETH` (0x4a25d94a) function
        pub fn swap_tokens_for_exact_eth(
            &self,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [74, 37, 217, 74],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactTokens` (0x8803dbee) function
        pub fn swap_tokens_for_exact_tokens(
            &self,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [136, 3, 219, 238],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for UniswapV2Router01<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `0xe8e33700`
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
        name = "addLiquidity",
        abi = "addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub amount_a_desired: ::ethers::core::types::U256,
        pub amount_b_desired: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addLiquidityETH` function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0xf305d719`
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
        name = "addLiquidityETH",
        abi = "addLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityETHCall {
        pub token: ::ethers::core::types::Address,
        pub amount_token_desired: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `getAmountIn` function with signature `getAmountIn(uint256,uint256,uint256)` and selector `0x85f8c259`
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
    #[ethcall(name = "getAmountIn", abi = "getAmountIn(uint256,uint256,uint256)")]
    pub struct GetAmountInCall {
        pub amount_out: ::ethers::core::types::U256,
        pub reserve_in: ::ethers::core::types::U256,
        pub reserve_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint256,uint256,uint256)` and selector `0x054d50d4`
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
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint256,uint256,uint256)")]
    pub struct GetAmountOutCall {
        pub amount_in: ::ethers::core::types::U256,
        pub reserve_in: ::ethers::core::types::U256,
        pub reserve_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAmountsIn` function with signature `getAmountsIn(uint256,address[])` and selector `0x1f00ca74`
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
    #[ethcall(name = "getAmountsIn", abi = "getAmountsIn(uint256,address[])")]
    pub struct GetAmountsInCall {
        pub amount_out: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getAmountsOut` function with signature `getAmountsOut(uint256,address[])` and selector `0xd06ca61f`
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
    #[ethcall(name = "getAmountsOut", abi = "getAmountsOut(uint256,address[])")]
    pub struct GetAmountsOutCall {
        pub amount_in: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote(uint256,uint256,uint256)` and selector `0xad615dec`
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
    #[ethcall(name = "quote", abi = "quote(uint256,uint256,uint256)")]
    pub struct QuoteCall {
        pub amount_a: ::ethers::core::types::U256,
        pub reserve_a: ::ethers::core::types::U256,
        pub reserve_b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `0xbaa2abde`
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
        name = "removeLiquidity",
        abi = "removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityETH` function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0x02751cec`
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
        name = "removeLiquidityETH",
        abi = "removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub token: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityETHWithPermit` function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0xded9382a`
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
        name = "removeLiquidityETHWithPermit",
        abi = "removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitCall {
        pub token: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `removeLiquidityWithPermit` function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0x2195995c`
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
        name = "removeLiquidityWithPermit",
        abi = "removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityWithPermitCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `swapETHForExactTokens` function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `0xfb3bdb41`
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
        name = "swapETHForExactTokens",
        abi = "swapETHForExactTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapETHForExactTokensCall {
        pub amount_out: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `0x7ff36ab5`
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
        name = "swapExactETHForTokens",
        abi = "swapExactETHForTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensCall {
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `0x18cbafe5`
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
        name = "swapExactTokensForETH",
        abi = "swapExactTokensForETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `0x38ed1739`
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
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapTokensForExactETH` function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `0x4a25d94a`
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
        name = "swapTokensForExactETH",
        abi = "swapTokensForExactETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactETHCall {
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `0x8803dbee`
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
        name = "swapTokensForExactTokens",
        abi = "swapTokensForExactTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactTokensCall {
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UniswapV2Router01Calls {
        Weth(WethCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityETH(AddLiquidityETHCall),
        Factory(FactoryCall),
        GetAmountIn(GetAmountInCall),
        GetAmountOut(GetAmountOutCall),
        GetAmountsIn(GetAmountsInCall),
        GetAmountsOut(GetAmountsOutCall),
        Quote(QuoteCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        RemoveLiquidityETHWithPermit(RemoveLiquidityETHWithPermitCall),
        RemoveLiquidityWithPermit(RemoveLiquidityWithPermitCall),
        SwapETHForExactTokens(SwapETHForExactTokensCall),
        SwapExactETHForTokens(SwapExactETHForTokensCall),
        SwapExactTokensForETH(SwapExactTokensForETHCall),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapTokensForExactETH(SwapTokensForExactETHCall),
        SwapTokensForExactTokens(SwapTokensForExactTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV2Router01Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityETHCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <GetAmountInCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAmountIn(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetAmountsInCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAmountsIn(decoded));
            }
            if let Ok(decoded) = <GetAmountsOutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAmountsOut(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidityETHWithPermit(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidityWithPermit(decoded));
            }
            if let Ok(decoded) =
                <SwapETHForExactTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapETHForExactTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapExactETHForTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapExactETHForTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapExactTokensForETHCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapExactTokensForETH(decoded));
            }
            if let Ok(decoded) =
                <SwapExactTokensForTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapExactTokensForTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapTokensForExactETHCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapTokensForExactETH(decoded));
            }
            if let Ok(decoded) =
                <SwapTokensForExactTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapTokensForExactTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapV2Router01Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidityETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountIn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountsIn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountsOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidityETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityETHWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::core::fmt::Display for UniswapV2Router01Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountsIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountsOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityETHWithPermit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapETHForExactTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactETHForTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactTokensForETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactTokensForTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapTokensForExactETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapTokensForExactTokens(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<WethCall> for UniswapV2Router01Calls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for UniswapV2Router01Calls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<AddLiquidityETHCall> for UniswapV2Router01Calls {
        fn from(value: AddLiquidityETHCall) -> Self {
            Self::AddLiquidityETH(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for UniswapV2Router01Calls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetAmountInCall> for UniswapV2Router01Calls {
        fn from(value: GetAmountInCall) -> Self {
            Self::GetAmountIn(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for UniswapV2Router01Calls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetAmountsInCall> for UniswapV2Router01Calls {
        fn from(value: GetAmountsInCall) -> Self {
            Self::GetAmountsIn(value)
        }
    }
    impl ::core::convert::From<GetAmountsOutCall> for UniswapV2Router01Calls {
        fn from(value: GetAmountsOutCall) -> Self {
            Self::GetAmountsOut(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for UniswapV2Router01Calls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityCall> for UniswapV2Router01Calls {
        fn from(value: RemoveLiquidityCall) -> Self {
            Self::RemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHCall> for UniswapV2Router01Calls {
        fn from(value: RemoveLiquidityETHCall) -> Self {
            Self::RemoveLiquidityETH(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHWithPermitCall> for UniswapV2Router01Calls {
        fn from(value: RemoveLiquidityETHWithPermitCall) -> Self {
            Self::RemoveLiquidityETHWithPermit(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityWithPermitCall> for UniswapV2Router01Calls {
        fn from(value: RemoveLiquidityWithPermitCall) -> Self {
            Self::RemoveLiquidityWithPermit(value)
        }
    }
    impl ::core::convert::From<SwapETHForExactTokensCall> for UniswapV2Router01Calls {
        fn from(value: SwapETHForExactTokensCall) -> Self {
            Self::SwapETHForExactTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensCall> for UniswapV2Router01Calls {
        fn from(value: SwapExactETHForTokensCall) -> Self {
            Self::SwapExactETHForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHCall> for UniswapV2Router01Calls {
        fn from(value: SwapExactTokensForETHCall) -> Self {
            Self::SwapExactTokensForETH(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensCall> for UniswapV2Router01Calls {
        fn from(value: SwapExactTokensForTokensCall) -> Self {
            Self::SwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<SwapTokensForExactETHCall> for UniswapV2Router01Calls {
        fn from(value: SwapTokensForExactETHCall) -> Self {
            Self::SwapTokensForExactETH(value)
        }
    }
    impl ::core::convert::From<SwapTokensForExactTokensCall> for UniswapV2Router01Calls {
        fn from(value: SwapTokensForExactTokensCall) -> Self {
            Self::SwapTokensForExactTokens(value)
        }
    }
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `0xe8e33700`
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
    pub struct AddLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addLiquidityETH` function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0xf305d719`
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
    pub struct AddLiquidityETHReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
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
    ///Container type for all return fields from the `getAmountIn` function with signature `getAmountIn(uint256,uint256,uint256)` and selector `0x85f8c259`
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
    pub struct GetAmountInReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint256,uint256,uint256)` and selector `0x054d50d4`
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
    pub struct GetAmountOutReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountsIn` function with signature `getAmountsIn(uint256,address[])` and selector `0x1f00ca74`
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
    pub struct GetAmountsInReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getAmountsOut` function with signature `getAmountsOut(uint256,address[])` and selector `0xd06ca61f`
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
    pub struct GetAmountsOutReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `quote` function with signature `quote(uint256,uint256,uint256)` and selector `0xad615dec`
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
    pub struct QuoteReturn {
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidity` function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `0xbaa2abde`
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
    pub struct RemoveLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETH` function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0x02751cec`
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
    pub struct RemoveLiquidityETHReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETHWithPermit` function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0xded9382a`
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
    pub struct RemoveLiquidityETHWithPermitReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityWithPermit` function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0x2195995c`
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
    pub struct RemoveLiquidityWithPermitReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swapETHForExactTokens` function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `0xfb3bdb41`
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
    pub struct SwapETHForExactTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `0x7ff36ab5`
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
    pub struct SwapExactETHForTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `0x18cbafe5`
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
    pub struct SwapExactTokensForETHReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `0x38ed1739`
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
    pub struct SwapExactTokensForTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapTokensForExactETH` function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `0x4a25d94a`
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
    pub struct SwapTokensForExactETHReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `0x8803dbee`
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
    pub struct SwapTokensForExactTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
}
