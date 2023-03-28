pub use i_uniswap_v3_pool_owner_actions::*;
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
pub mod i_uniswap_v3_pool_owner_actions {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount0Requested\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1Requested\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collectProtocol\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"amount0\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol0\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"feeProtocol1\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeProtocol\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUNISWAPV3POOLOWNERACTIONS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IUniswapV3PoolOwnerActions<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniswapV3PoolOwnerActions<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniswapV3PoolOwnerActions<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniswapV3PoolOwnerActions<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniswapV3PoolOwnerActions<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolOwnerActions))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3PoolOwnerActions<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IUNISWAPV3POOLOWNERACTIONS_ABI.clone(),
                    client,
                ),
            )
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
    for IUniswapV3PoolOwnerActions<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
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
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8,uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol_0: u8,
        pub fee_protocol_1: u8,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniswapV3PoolOwnerActionsCalls {
        CollectProtocol(CollectProtocolCall),
        SetFeeProtocol(SetFeeProtocolCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV3PoolOwnerActionsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CollectProtocolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CollectProtocol(decoded));
            }
            if let Ok(decoded)
                = <SetFeeProtocolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeProtocol(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV3PoolOwnerActionsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollectProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IUniswapV3PoolOwnerActionsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectProtocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeProtocol(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CollectProtocolCall> for IUniswapV3PoolOwnerActionsCalls {
        fn from(value: CollectProtocolCall) -> Self {
            Self::CollectProtocol(value)
        }
    }
    impl ::core::convert::From<SetFeeProtocolCall> for IUniswapV3PoolOwnerActionsCalls {
        fn from(value: SetFeeProtocolCall) -> Self {
            Self::SetFeeProtocol(value)
        }
    }
    ///Container type for all return fields from the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
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
    pub struct CollectProtocolReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
}
