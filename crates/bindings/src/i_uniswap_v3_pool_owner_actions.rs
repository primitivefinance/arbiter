pub use i_uniswap_v3_pool_owner_actions::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_uniswap_v3_pool_owner_actions {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///IUniswapV3PoolOwnerActions was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;

    use ::ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::Middleware,
    };
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount0Requested\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1Requested\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collectProtocol\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"amount0\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol0\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"feeProtocol1\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeProtocol\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static IUNISWAPV3POOLOWNERACTIONS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IUniswapV3PoolOwnerActions<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV3PoolOwnerActions<M> {
        fn clone(&self) -> Self {
            IUniswapV3PoolOwnerActions(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV3PoolOwnerActions<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV3PoolOwnerActions<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolOwnerActions))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3PoolOwnerActions<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV3POOLOWNERACTIONS_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `collectProtocol` (0x85b66729) function
        pub fn collect_protocol(
            &self,
            recipient: ::ethers::core::types::Address,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [133, 182, 103, 41],
                    (recipient, amount_0_requested, amount_1_requested),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeProtocol` (0x8206a4d1) function
        pub fn set_fee_protocol(
            &self,
            fee_protocol_0: u8,
            fee_protocol_1: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 6, 164, 209], (fee_protocol_0, fee_protocol_1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IUniswapV3PoolOwnerActions<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "collectProtocol",
        abi = "collectProtocol(address,uint128,uint128)"
    )]
    pub struct CollectProtocolCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    ///Container type for all input parameters for the `setFeeProtocol` function with signature `setFeeProtocol(uint8,uint8)` and selector `0x8206a4d1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8,uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol_0: u8,
        pub fee_protocol_1: u8,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IUniswapV3PoolOwnerActionsCalls {
        CollectProtocol(CollectProtocolCall),
        SetFeeProtocol(SetFeeProtocolCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV3PoolOwnerActionsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CollectProtocolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolOwnerActionsCalls::CollectProtocol(decoded));
            }
            if let Ok(decoded) =
                <SetFeeProtocolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolOwnerActionsCalls::SetFeeProtocol(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV3PoolOwnerActionsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV3PoolOwnerActionsCalls::CollectProtocol(element) => element.encode(),
                IUniswapV3PoolOwnerActionsCalls::SetFeeProtocol(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolOwnerActionsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolOwnerActionsCalls::CollectProtocol(element) => element.fmt(f),
                IUniswapV3PoolOwnerActionsCalls::SetFeeProtocol(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CollectProtocolCall> for IUniswapV3PoolOwnerActionsCalls {
        fn from(var: CollectProtocolCall) -> Self {
            IUniswapV3PoolOwnerActionsCalls::CollectProtocol(var)
        }
    }
    impl ::std::convert::From<SetFeeProtocolCall> for IUniswapV3PoolOwnerActionsCalls {
        fn from(var: SetFeeProtocolCall) -> Self {
            IUniswapV3PoolOwnerActionsCalls::SetFeeProtocol(var)
        }
    }
    ///Container type for all return fields from the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct CollectProtocolReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
}
