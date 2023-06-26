pub use weth9::*;
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
pub mod weth9 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approve"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("guy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dst"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("src"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dst"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdraw"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("wad"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Approval"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("src"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("guy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dst"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("src"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dst"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Withdrawal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("src"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static WETH9_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R`\r`\x80\x81\x90Rl+\xB90\xB882\xB2\x10\"\xBA42\xB9`\x99\x1B`\xA0\x90\x81Ra\0.\x91`\0\x91\x90a\0\xC0V[P`@\x80Q\x80\x82\x01\x90\x91R`\x04\x80\x82Rc\n\xE8\xAA\x89`\xE3\x1B` \x90\x92\x01\x91\x82Ra\0Z\x91`\x01\x91a\0\xC0V[P`\x02\x80T`\xFF\x19\x16`\x12\x17\x90U4\x80\x15a\0\xBAWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x01[V[\x82\x80T`\x01\x81`\x01\x16\x15a\x01\0\x02\x03\x16`\x02\x90\x04\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82`\x1F\x10a\x01\x01W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\x01.V[\x82\x80\x01`\x01\x01\x85U\x82\x15a\x01.W\x91\x82\x01[\x82\x81\x11\x15a\x01.W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x01\x13V[Pa\x01:\x92\x91Pa\x01>V[P\x90V[a\x01X\x91\x90[\x80\x82\x11\x15a\x01:W`\0\x81U`\x01\x01a\x01DV[\x90V[a\n\xFC\x80a\x01j`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x9CW`\x005`\xE0\x1C\x80c1<\xE5g\x11a\0dW\x80c1<\xE5g\x14a\x04\x01W\x80cp\xA0\x821\x14a\x04`W\x80c\x95\xD8\x9BA\x14a\x04\xF9W\x80c\xA9\x05\x9C\xBB\x14a\x05BW\x80c\xD0\xE3\r\xB0\x14a\x05\xE1W\x80c\xDDb\xED>\x14a\x05\xE9Wa\0\x9CV[\x80c\x06\xFD\xDE\x03\x14a\0\xFAW\x80c\t^\xA7\xB3\x14a\x01\xB8W\x80c\x18\x16\r\xDD\x14a\x02kW\x80c#\xB8r\xDD\x14a\x02\xC6W\x80c.\x1A}M\x14a\x03oW[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\x01:WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x01Ca\x06\x8AV[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x01}W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01eV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x01\xAAW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02W`\x04\x806\x03`@\x81\x10\x15a\x02AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x07\x18V[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x02\xABWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02\xB4a\x07~V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x03\x06WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02W`\x04\x806\x03``\x81\x10\x15a\x03OWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x07\x82V[4\x80\x15a\x03\xAFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x03\xFF`\x04\x806\x03` \x81\x10\x15a\x03\xF8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P5a\x08\xFAV[\0[4\x80\x15a\x04AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04Ja\t\xB1V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x04\xA0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02\xB4`\x04\x806\x03` \x81\x10\x15a\x04\xE9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\t\xBAV[4\x80\x15a\x059WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x01Ca\t\xCCV[4\x80\x15a\x05\x82WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02W`\x04\x806\x03`@\x81\x10\x15a\x05\xCBWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\n&V[a\x03\xFFa\n:V[4\x80\x15a\x06)WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02\xB4`\x04\x806\x03`@\x81\x10\x15a\x06rWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\n\x89V[`\0\x80T`@\x80Q` `\x02`\x01\x85\x16\x15a\x01\0\x02`\0\x19\x01\x90\x94\x16\x93\x90\x93\x04`\x1F\x81\x01\x84\x90\x04\x84\x02\x82\x01\x84\x01\x90\x92R\x81\x81R\x92\x91\x83\x01\x82\x82\x80\x15a\x07\x10W\x80`\x1F\x10a\x06\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x10V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xF3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x90\x83R\x81\x84 \x86\x90U\x81Q\x86\x81R\x91Q\x93\x94\x90\x93\x90\x92\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x92\x82\x90\x03\x01\x90\xA3P`\x01\x92\x91PPV[G\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 T\x82\x11\x15a\x07\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\0`$\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x163\x14\x80\x15\x90a\x08\x07WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x14\x15[\x15a\x08\x89W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x82\x11\x15a\x08^W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\0`$\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T\x83\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x88\x90\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x83Q\x86\x81R\x93Q\x91\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x90\x81\x90\x03\x90\x91\x01\x90\xA3P`\x01\x93\x92PPPV[3`\0\x90\x81R`\x03` R`@\x90 T\x81\x11\x15a\t8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\0`$\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[3`\0\x81\x81R`\x03` R`@\x80\x82 \x80T\x85\x90\x03\x90UQ\x83\x15a\x08\xFC\x02\x91\x84\x91\x90\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\twW=`\0\x80>=`\0\xFD[P`@\x80Q\x82\x81R\x90Q3\x91\x7F\x7F\xCFS,\x15\xF0\xA6\xDB\x0B\xD6\xD0\xE08\xBE\xA7\x1D0\xD8\x08\xC7\xD9\x8C\xB3\xBFrh\xA9[\xF5\x08\x1Be\x91\x90\x81\x90\x03` \x01\x90\xA2PV[`\x02T`\xFF\x16\x81V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[`\x01\x80T`@\x80Q` `\x02\x84\x86\x16\x15a\x01\0\x02`\0\x19\x01\x90\x94\x16\x93\x90\x93\x04`\x1F\x81\x01\x84\x90\x04\x84\x02\x82\x01\x84\x01\x90\x92R\x81\x81R\x92\x91\x83\x01\x82\x82\x80\x15a\x07\x10W\x80`\x1F\x10a\x06\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x10V[`\0a\n33\x84\x84a\x07\x82V[\x93\x92PPPV[3`\0\x81\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T4\x90\x81\x01\x90\x91U\x82Q\x90\x81R\x91Q\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x92\x81\x90\x03\x90\x91\x01\x90\xA2V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V\xFEEther sent to non-payable functi\xA2dipfsX\"\x12 %S\x0C\x14z\xED\x0Cy\xD74\xE4\x89\xAE\xF6\xEER\xBD\"\xECH\x92\x08\xB8Q\xA6i\xBCJ@L\xF1adsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static WETH9_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x9CW`\x005`\xE0\x1C\x80c1<\xE5g\x11a\0dW\x80c1<\xE5g\x14a\x04\x01W\x80cp\xA0\x821\x14a\x04`W\x80c\x95\xD8\x9BA\x14a\x04\xF9W\x80c\xA9\x05\x9C\xBB\x14a\x05BW\x80c\xD0\xE3\r\xB0\x14a\x05\xE1W\x80c\xDDb\xED>\x14a\x05\xE9Wa\0\x9CV[\x80c\x06\xFD\xDE\x03\x14a\0\xFAW\x80c\t^\xA7\xB3\x14a\x01\xB8W\x80c\x18\x16\r\xDD\x14a\x02kW\x80c#\xB8r\xDD\x14a\x02\xC6W\x80c.\x1A}M\x14a\x03oW[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`5`$R\x7FContract does not have fallback `D\x90\x81Rtnor receive functions`X\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\x01:WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x01Ca\x06\x8AV[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x01}W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01eV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x01\xAAW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02W`\x04\x806\x03`@\x81\x10\x15a\x02AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x07\x18V[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x02\xABWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02\xB4a\x07~V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x03\x06WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02W`\x04\x806\x03``\x81\x10\x15a\x03OWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x07\x82V[4\x80\x15a\x03\xAFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x03\xFF`\x04\x806\x03` \x81\x10\x15a\x03\xF8WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P5a\x08\xFAV[\0[4\x80\x15a\x04AWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x04Ja\t\xB1V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x04\xA0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02\xB4`\x04\x806\x03` \x81\x10\x15a\x04\xE9WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\t\xBAV[4\x80\x15a\x059WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x01Ca\t\xCCV[4\x80\x15a\x05\x82WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02W`\x04\x806\x03`@\x81\x10\x15a\x05\xCBWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\n&V[a\x03\xFFa\n:V[4\x80\x15a\x06)WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R`\0\x80Q` a\n\xA7\x839\x81Q\x91R`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x02\xB4`\x04\x806\x03`@\x81\x10\x15a\x06rWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\n\x89V[`\0\x80T`@\x80Q` `\x02`\x01\x85\x16\x15a\x01\0\x02`\0\x19\x01\x90\x94\x16\x93\x90\x93\x04`\x1F\x81\x01\x84\x90\x04\x84\x02\x82\x01\x84\x01\x90\x92R\x81\x81R\x92\x91\x83\x01\x82\x82\x80\x15a\x07\x10W\x80`\x1F\x10a\x06\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x10V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xF3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x90\x83R\x81\x84 \x86\x90U\x81Q\x86\x81R\x91Q\x93\x94\x90\x93\x90\x92\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x92\x82\x90\x03\x01\x90\xA3P`\x01\x92\x91PPV[G\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 T\x82\x11\x15a\x07\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\0`$\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x163\x14\x80\x15\x90a\x08\x07WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x14\x15[\x15a\x08\x89W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x82\x11\x15a\x08^W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\0`$\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T\x83\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x88\x90\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x83Q\x86\x81R\x93Q\x91\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x90\x81\x90\x03\x90\x91\x01\x90\xA3P`\x01\x93\x92PPPV[3`\0\x90\x81R`\x03` R`@\x90 T\x81\x11\x15a\t8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\0`$\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[3`\0\x81\x81R`\x03` R`@\x80\x82 \x80T\x85\x90\x03\x90UQ\x83\x15a\x08\xFC\x02\x91\x84\x91\x90\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\twW=`\0\x80>=`\0\xFD[P`@\x80Q\x82\x81R\x90Q3\x91\x7F\x7F\xCFS,\x15\xF0\xA6\xDB\x0B\xD6\xD0\xE08\xBE\xA7\x1D0\xD8\x08\xC7\xD9\x8C\xB3\xBFrh\xA9[\xF5\x08\x1Be\x91\x90\x81\x90\x03` \x01\x90\xA2PV[`\x02T`\xFF\x16\x81V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[`\x01\x80T`@\x80Q` `\x02\x84\x86\x16\x15a\x01\0\x02`\0\x19\x01\x90\x94\x16\x93\x90\x93\x04`\x1F\x81\x01\x84\x90\x04\x84\x02\x82\x01\x84\x01\x90\x92R\x81\x81R\x92\x91\x83\x01\x82\x82\x80\x15a\x07\x10W\x80`\x1F\x10a\x06\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x10V[`\0a\n33\x84\x84a\x07\x82V[\x93\x92PPPV[3`\0\x81\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T4\x90\x81\x01\x90\x91U\x82Q\x90\x81R\x91Q\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x92\x81\x90\x03\x90\x91\x01\x90\xA2V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V\xFEEther sent to non-payable functi\xA2dipfsX\"\x12 %S\x0C\x14z\xED\x0Cy\xD74\xE4\x89\xAE\xF6\xEER\xBD\"\xECH\x92\x08\xB8Q\xA6i\xBCJ@L\xF1adsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static WETH9_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct WETH9<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WETH9<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WETH9<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WETH9<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WETH9<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WETH9))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WETH9<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                WETH9_ABI.clone(),
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
                WETH9_ABI.clone(),
                WETH9_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
            guy: ::ethers::core::types::Address,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (guy, wad))
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
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xd0e30db0) function
        pub fn deposit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
            dst: ::ethers::core::types::Address,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, wad))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            src: ::ethers::core::types::Address,
            dst: ::ethers::core::types::Address,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, wad))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], wad)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        ///Gets the contract's `Withdrawal` event
        pub fn withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WETH9Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for WETH9<M> {
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
        Hash,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub src: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub guy: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub src: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(address,uint256)")]
    pub struct WithdrawalFilter {
        #[ethevent(indexed)]
        pub src: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WETH9Events {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        TransferFilter(TransferFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ::ethers::contract::EthLogDecode for WETH9Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(WETH9Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(WETH9Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(WETH9Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(WETH9Events::WithdrawalFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WETH9Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for WETH9Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for WETH9Events {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for WETH9Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalFilter> for WETH9Events {
        fn from(value: WithdrawalFilter) -> Self {
            Self::WithdrawalFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
        Hash,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub guy: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit()` and selector `0xd0e30db0`
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
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
        Hash,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub src: ::ethers::core::types::Address,
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub wad: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WETH9Calls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        Name(NameCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for WETH9Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WETH9Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for WETH9Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for WETH9Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for WETH9Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for WETH9Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for WETH9Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DepositCall> for WETH9Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<NameCall> for WETH9Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for WETH9Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for WETH9Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for WETH9Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for WETH9Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for WETH9Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
        Hash,
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
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
        Hash,
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
        Hash,
    )]
    pub struct TransferFromReturn(pub bool);
}
