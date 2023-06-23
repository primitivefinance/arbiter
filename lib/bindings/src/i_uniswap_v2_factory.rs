pub use i_uniswap_v2_factory::*;
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
pub mod i_uniswap_v2_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allPairs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allPairs"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("pair"),
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
                    ::std::borrow::ToOwned::to_owned("allPairsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allPairsLength"),
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
                    ::std::borrow::ToOwned::to_owned("createPair"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createPair"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeTo"),
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
                    ::std::borrow::ToOwned::to_owned("feeToSetter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeToSetter"),
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
                    ::std::borrow::ToOwned::to_owned("getPair"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPair"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pair"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("setFeeToSetter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeToSetter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PairCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PairCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
    pub static IUNISWAPV2FACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IUniswapV2Factory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniswapV2Factory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniswapV2Factory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniswapV2Factory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniswapV2Factory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IUniswapV2Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV2Factory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IUNISWAPV2FACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `allPairs` (0x1e3dd18b) function
        pub fn all_pairs(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([30, 61, 209, 139], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allPairsLength` (0x574f2ba3) function
        pub fn all_pairs_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([87, 79, 43, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPair` (0xc9c65396) function
        pub fn create_pair(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([201, 198, 83, 150], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeTo` (0x017e7e58) function
        pub fn fee_to(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([1, 126, 126, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeToSetter` (0x094b7415) function
        pub fn fee_to_setter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([9, 75, 116, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPair` (0xe6a43905) function
        pub fn get_pair(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([230, 164, 57, 5], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeTo` (0xf46901ed) function
        pub fn set_fee_to(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 105, 1, 237], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeToSetter` (0xa2e74af6) function
        pub fn set_fee_to_setter(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 231, 74, 246], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PairCreated` event
        pub fn pair_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PairCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PairCreatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IUniswapV2Factory<M> {
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
    #[ethevent(
        name = "PairCreated",
        abi = "PairCreated(address,address,address,uint256)"
    )]
    pub struct PairCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ::ethers::core::types::Address,
        pub pair: ::ethers::core::types::Address,
        pub p3: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allPairs` function with signature `allPairs(uint256)` and selector `0x1e3dd18b`
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
    #[ethcall(name = "allPairs", abi = "allPairs(uint256)")]
    pub struct AllPairsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `allPairsLength` function with signature `allPairsLength()` and selector `0x574f2ba3`
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
    #[ethcall(name = "allPairsLength", abi = "allPairsLength()")]
    pub struct AllPairsLengthCall;
    ///Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
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
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `feeTo` function with signature `feeTo()` and selector `0x017e7e58`
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
    #[ethcall(name = "feeTo", abi = "feeTo()")]
    pub struct FeeToCall;
    ///Container type for all input parameters for the `feeToSetter` function with signature `feeToSetter()` and selector `0x094b7415`
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
    #[ethcall(name = "feeToSetter", abi = "feeToSetter()")]
    pub struct FeeToSetterCall;
    ///Container type for all input parameters for the `getPair` function with signature `getPair(address,address)` and selector `0xe6a43905`
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
    #[ethcall(name = "getPair", abi = "getPair(address,address)")]
    pub struct GetPairCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeTo` function with signature `setFeeTo(address)` and selector `0xf46901ed`
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
    #[ethcall(name = "setFeeTo", abi = "setFeeTo(address)")]
    pub struct SetFeeToCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `setFeeToSetter` function with signature `setFeeToSetter(address)` and selector `0xa2e74af6`
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
    #[ethcall(name = "setFeeToSetter", abi = "setFeeToSetter(address)")]
    pub struct SetFeeToSetterCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniswapV2FactoryCalls {
        AllPairs(AllPairsCall),
        AllPairsLength(AllPairsLengthCall),
        CreatePair(CreatePairCall),
        FeeTo(FeeToCall),
        FeeToSetter(FeeToSetterCall),
        GetPair(GetPairCall),
        SetFeeTo(SetFeeToCall),
        SetFeeToSetter(SetFeeToSetterCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV2FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AllPairsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllPairs(decoded));
            }
            if let Ok(decoded)
                = <AllPairsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllPairsLength(decoded));
            }
            if let Ok(decoded)
                = <CreatePairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePair(decoded));
            }
            if let Ok(decoded)
                = <FeeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeTo(decoded));
            }
            if let Ok(decoded)
                = <FeeToSetterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeToSetter(decoded));
            }
            if let Ok(decoded)
                = <GetPairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPair(decoded));
            }
            if let Ok(decoded)
                = <SetFeeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeTo(decoded));
            }
            if let Ok(decoded)
                = <SetFeeToSetterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeToSetter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV2FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllPairs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllPairsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePair(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeToSetter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPair(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeToSetter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IUniswapV2FactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllPairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllPairsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePair(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeToSetter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPair(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeToSetter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllPairsCall> for IUniswapV2FactoryCalls {
        fn from(value: AllPairsCall) -> Self {
            Self::AllPairs(value)
        }
    }
    impl ::core::convert::From<AllPairsLengthCall> for IUniswapV2FactoryCalls {
        fn from(value: AllPairsLengthCall) -> Self {
            Self::AllPairsLength(value)
        }
    }
    impl ::core::convert::From<CreatePairCall> for IUniswapV2FactoryCalls {
        fn from(value: CreatePairCall) -> Self {
            Self::CreatePair(value)
        }
    }
    impl ::core::convert::From<FeeToCall> for IUniswapV2FactoryCalls {
        fn from(value: FeeToCall) -> Self {
            Self::FeeTo(value)
        }
    }
    impl ::core::convert::From<FeeToSetterCall> for IUniswapV2FactoryCalls {
        fn from(value: FeeToSetterCall) -> Self {
            Self::FeeToSetter(value)
        }
    }
    impl ::core::convert::From<GetPairCall> for IUniswapV2FactoryCalls {
        fn from(value: GetPairCall) -> Self {
            Self::GetPair(value)
        }
    }
    impl ::core::convert::From<SetFeeToCall> for IUniswapV2FactoryCalls {
        fn from(value: SetFeeToCall) -> Self {
            Self::SetFeeTo(value)
        }
    }
    impl ::core::convert::From<SetFeeToSetterCall> for IUniswapV2FactoryCalls {
        fn from(value: SetFeeToSetterCall) -> Self {
            Self::SetFeeToSetter(value)
        }
    }
    ///Container type for all return fields from the `allPairs` function with signature `allPairs(uint256)` and selector `0x1e3dd18b`
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
    pub struct AllPairsReturn {
        pub pair: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `allPairsLength` function with signature `allPairsLength()` and selector `0x574f2ba3`
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
    pub struct AllPairsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
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
    pub struct CreatePairReturn {
        pub pair: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `feeTo` function with signature `feeTo()` and selector `0x017e7e58`
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
    pub struct FeeToReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeToSetter` function with signature `feeToSetter()` and selector `0x094b7415`
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
    pub struct FeeToSetterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPair` function with signature `getPair(address,address)` and selector `0xe6a43905`
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
    pub struct GetPairReturn {
        pub pair: ::ethers::core::types::Address,
    }
}
