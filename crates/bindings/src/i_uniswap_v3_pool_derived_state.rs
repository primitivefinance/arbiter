pub use i_uniswap_v3_pool_derived_state::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_uniswap_v3_pool_derived_state {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///IUniswapV3PoolDerivedState was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    use ::ethers::core::{
        abi::{Abi, Token, Detokenize, InvalidOutputType, Tokenizable},
        types::*,
    };
    use ::ethers::contract::{
        Contract, builders::{ContractCall, Event},
        Lazy,
    };
    use ::ethers::providers::Middleware;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint32[]\",\"name\":\"secondsAgos\",\"type\":\"uint32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"observe\",\"outputs\":[{\"internalType\":\"int56[]\",\"name\":\"tickCumulatives\",\"type\":\"int56[]\",\"components\":[]},{\"internalType\":\"uint160[]\",\"name\":\"secondsPerLiquidityCumulativeX128s\",\"type\":\"uint160[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"snapshotCumulativesInside\",\"outputs\":[{\"internalType\":\"int56\",\"name\":\"tickCumulativeInside\",\"type\":\"int56\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"secondsPerLiquidityInsideX128\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"secondsInside\",\"type\":\"uint32\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static IUNISWAPV3POOLDERIVEDSTATE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
    pub struct IUniswapV3PoolDerivedState<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV3PoolDerivedState<M> {
        fn clone(&self) -> Self {
            IUniswapV3PoolDerivedState(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV3PoolDerivedState<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV3PoolDerivedState<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolDerivedState))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3PoolDerivedState<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IUNISWAPV3POOLDERIVEDSTATE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `observe` (0x883bdbfd) function
        pub fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<i64>, ::std::vec::Vec<::ethers::core::types::U256>),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (i64, ::ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([163, 136, 7, 242], (tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IUniswapV3PoolDerivedState<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "observe", abi = "observe(uint32[])")]
    pub struct ObserveCall {
        pub seconds_agos: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "snapshotCumulativesInside",
        abi = "snapshotCumulativesInside(int24,int24)"
    )]
    pub struct SnapshotCumulativesInsideCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IUniswapV3PoolDerivedStateCalls {
        Observe(ObserveCall),
        SnapshotCumulativesInside(SnapshotCumulativesInsideCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV3PoolDerivedStateCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <ObserveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IUniswapV3PoolDerivedStateCalls::Observe(decoded));
            }
            if let Ok(decoded)
                = <SnapshotCumulativesInsideCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    IUniswapV3PoolDerivedStateCalls::SnapshotCumulativesInside(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV3PoolDerivedStateCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV3PoolDerivedStateCalls::Observe(element) => element.encode(),
                IUniswapV3PoolDerivedStateCalls::SnapshotCumulativesInside(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolDerivedStateCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolDerivedStateCalls::Observe(element) => element.fmt(f),
                IUniswapV3PoolDerivedStateCalls::SnapshotCumulativesInside(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<ObserveCall> for IUniswapV3PoolDerivedStateCalls {
        fn from(var: ObserveCall) -> Self {
            IUniswapV3PoolDerivedStateCalls::Observe(var)
        }
    }
    impl ::std::convert::From<SnapshotCumulativesInsideCall>
    for IUniswapV3PoolDerivedStateCalls {
        fn from(var: SnapshotCumulativesInsideCall) -> Self {
            IUniswapV3PoolDerivedStateCalls::SnapshotCumulativesInside(var)
        }
    }
    ///Container type for all return fields from the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct ObserveReturn {
        pub tick_cumulatives: ::std::vec::Vec<i64>,
        pub seconds_per_liquidity_cumulative_x12_8s: ::std::vec::Vec<
            ::ethers::core::types::U256,
        >,
    }
    ///Container type for all return fields from the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct SnapshotCumulativesInsideReturn {
        pub tick_cumulative_inside: i64,
        pub seconds_per_liquidity_inside_x128: ::ethers::core::types::U256,
        pub seconds_inside: u32,
    }
}
