pub use i_uniswap_v3_pool_derived_state::*;
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
pub mod i_uniswap_v3_pool_derived_state {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("observe"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("observe"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("secondsAgos"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickCumulatives"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(56usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int56[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "secondsPerLiquidityCumulativeX128s",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160[]"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("snapshotCumulativesInside"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("snapshotCumulativesInside",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickCumulativeInside",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(56usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int56"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "secondsPerLiquidityInsideX128",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("secondsInside"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
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
    pub static IUNISWAPV3POOLDERIVEDSTATE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IUniswapV3PoolDerivedState<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniswapV3PoolDerivedState<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniswapV3PoolDerivedState<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniswapV3PoolDerivedState<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniswapV3PoolDerivedState<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IUniswapV3PoolDerivedState))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3PoolDerivedState<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV3POOLDERIVEDSTATE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `observe` (0x883bdbfd) function
        pub fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<i64>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([136, 59, 219, 253], seconds_agos)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `snapshotCumulativesInside` (0xa38807f2) function
        pub fn snapshot_cumulatives_inside(
            &self,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i64, ::ethers::core::types::U256, u32)>
        {
            self.0
                .method_hash([163, 136, 7, 242], (tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IUniswapV3PoolDerivedState<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
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
    #[ethcall(name = "observe", abi = "observe(uint32[])")]
    pub struct ObserveCall {
        pub seconds_agos: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
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
        name = "snapshotCumulativesInside",
        abi = "snapshotCumulativesInside(int24,int24)"
    )]
    pub struct SnapshotCumulativesInsideCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniswapV3PoolDerivedStateCalls {
        Observe(ObserveCall),
        SnapshotCumulativesInside(SnapshotCumulativesInsideCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV3PoolDerivedStateCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ObserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Observe(decoded));
            }
            if let Ok(decoded) =
                <SnapshotCumulativesInsideCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SnapshotCumulativesInside(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV3PoolDerivedStateCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Observe(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SnapshotCumulativesInside(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IUniswapV3PoolDerivedStateCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Observe(element) => ::core::fmt::Display::fmt(element, f),
                Self::SnapshotCumulativesInside(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ObserveCall> for IUniswapV3PoolDerivedStateCalls {
        fn from(value: ObserveCall) -> Self {
            Self::Observe(value)
        }
    }
    impl ::core::convert::From<SnapshotCumulativesInsideCall> for IUniswapV3PoolDerivedStateCalls {
        fn from(value: SnapshotCumulativesInsideCall) -> Self {
            Self::SnapshotCumulativesInside(value)
        }
    }
    ///Container type for all return fields from the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
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
    pub struct ObserveReturn {
        pub tick_cumulatives: ::std::vec::Vec<i64>,
        pub seconds_per_liquidity_cumulative_x12_8s: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
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
    pub struct SnapshotCumulativesInsideReturn {
        pub tick_cumulative_inside: i64,
        pub seconds_per_liquidity_inside_x128: ::ethers::core::types::U256,
        pub seconds_inside: u32,
    }
}
