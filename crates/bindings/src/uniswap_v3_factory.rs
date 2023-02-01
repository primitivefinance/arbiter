pub use uniswap_v3_factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod uniswap_v3_factory {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ::ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ::ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ::ethers::providers::Middleware;
    ///UniswapV3Factory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FeeAmountEnabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PoolCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableFeeAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeAmountTickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"parameters\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static UNISWAPV3FACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct UniswapV3Factory<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for UniswapV3Factory<M> {
        fn clone(&self) -> Self {
            UniswapV3Factory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for UniswapV3Factory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for UniswapV3Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV3Factory<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UNISWAPV3FACTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `createPool` (0xa1671295) function
        pub fn create_pool(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            fee: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([161, 103, 18, 149], (token_a, token_b, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableFeeAmount` (0x8a7c195f) function
        pub fn enable_fee_amount(
            &self,
            fee: u32,
            tick_spacing: i32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 124, 25, 95], (fee, tick_spacing))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeAmountTickSpacing` (0x22afcccb) function
        pub fn fee_amount_tick_spacing(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([34, 175, 204, 203], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x1698ee82) function
        pub fn get_pool(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([22, 152, 238, 130], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parameters` (0x89035730) function
        pub fn parameters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                u32,
                i32,
            ),
        > {
            self.0
                .method_hash([137, 3, 87, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOwner` (0x13af4035) function
        pub fn set_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FeeAmountEnabled` event
        pub fn fee_amount_enabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, FeeAmountEnabledFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnerChanged` event
        pub fn owner_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, OwnerChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PoolCreated` event
        pub fn pool_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PoolCreatedFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, UniswapV3FactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for UniswapV3Factory<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "FeeAmountEnabled", abi = "FeeAmountEnabled(uint24,int24)")]
    pub struct FeeAmountEnabledFilter {
        #[ethevent(indexed)]
        pub fee: u32,
        #[ethevent(indexed)]
        pub tick_spacing: i32,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "OwnerChanged", abi = "OwnerChanged(address,address)")]
    pub struct OwnerChangedFilter {
        #[ethevent(indexed)]
        pub old_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "PoolCreated",
        abi = "PoolCreated(address,address,uint24,int24,address)"
    )]
    pub struct PoolCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub fee: u32,
        pub tick_spacing: i32,
        pub pool: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum UniswapV3FactoryEvents {
        FeeAmountEnabledFilter(FeeAmountEnabledFilter),
        OwnerChangedFilter(OwnerChangedFilter),
        PoolCreatedFilter(PoolCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for UniswapV3FactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FeeAmountEnabledFilter::decode_log(log) {
                return Ok(UniswapV3FactoryEvents::FeeAmountEnabledFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(UniswapV3FactoryEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = PoolCreatedFilter::decode_log(log) {
                return Ok(UniswapV3FactoryEvents::PoolCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UniswapV3FactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3FactoryEvents::FeeAmountEnabledFilter(element) => element.fmt(f),
                UniswapV3FactoryEvents::OwnerChangedFilter(element) => element.fmt(f),
                UniswapV3FactoryEvents::PoolCreatedFilter(element) => element.fmt(f),
            }
        }
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(address,address,uint24)` and selector `0xa1671295`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "createPool", abi = "createPool(address,address,uint24)")]
    pub struct CreatePoolCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub fee: u32,
    }
    ///Container type for all input parameters for the `enableFeeAmount` function with signature `enableFeeAmount(uint24,int24)` and selector `0x8a7c195f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "enableFeeAmount", abi = "enableFeeAmount(uint24,int24)")]
    pub struct EnableFeeAmountCall {
        pub fee: u32,
        pub tick_spacing: i32,
    }
    ///Container type for all input parameters for the `feeAmountTickSpacing` function with signature `feeAmountTickSpacing(uint24)` and selector `0x22afcccb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeAmountTickSpacing", abi = "feeAmountTickSpacing(uint24)")]
    pub struct FeeAmountTickSpacingCall(pub u32);
    ///Container type for all input parameters for the `getPool` function with signature `getPool(address,address,uint24)` and selector `0x1698ee82`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPool", abi = "getPool(address,address,uint24)")]
    pub struct GetPoolCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub u32,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `parameters` function with signature `parameters()` and selector `0x89035730`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "parameters", abi = "parameters()")]
    pub struct ParametersCall;
    ///Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `0x13af4035`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum UniswapV3FactoryCalls {
        CreatePool(CreatePoolCall),
        EnableFeeAmount(EnableFeeAmountCall),
        FeeAmountTickSpacing(FeeAmountTickSpacingCall),
        GetPool(GetPoolCall),
        Owner(OwnerCall),
        Parameters(ParametersCall),
        SetOwner(SetOwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV3FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3FactoryCalls::CreatePool(decoded));
            }
            if let Ok(decoded) =
                <EnableFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3FactoryCalls::EnableFeeAmount(decoded));
            }
            if let Ok(decoded) =
                <FeeAmountTickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3FactoryCalls::FeeAmountTickSpacing(decoded));
            }
            if let Ok(decoded) =
                <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3FactoryCalls::GetPool(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3FactoryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <ParametersCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3FactoryCalls::Parameters(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3FactoryCalls::SetOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapV3FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV3FactoryCalls::CreatePool(element) => element.encode(),
                UniswapV3FactoryCalls::EnableFeeAmount(element) => element.encode(),
                UniswapV3FactoryCalls::FeeAmountTickSpacing(element) => element.encode(),
                UniswapV3FactoryCalls::GetPool(element) => element.encode(),
                UniswapV3FactoryCalls::Owner(element) => element.encode(),
                UniswapV3FactoryCalls::Parameters(element) => element.encode(),
                UniswapV3FactoryCalls::SetOwner(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV3FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3FactoryCalls::CreatePool(element) => element.fmt(f),
                UniswapV3FactoryCalls::EnableFeeAmount(element) => element.fmt(f),
                UniswapV3FactoryCalls::FeeAmountTickSpacing(element) => element.fmt(f),
                UniswapV3FactoryCalls::GetPool(element) => element.fmt(f),
                UniswapV3FactoryCalls::Owner(element) => element.fmt(f),
                UniswapV3FactoryCalls::Parameters(element) => element.fmt(f),
                UniswapV3FactoryCalls::SetOwner(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreatePoolCall> for UniswapV3FactoryCalls {
        fn from(var: CreatePoolCall) -> Self {
            UniswapV3FactoryCalls::CreatePool(var)
        }
    }
    impl ::std::convert::From<EnableFeeAmountCall> for UniswapV3FactoryCalls {
        fn from(var: EnableFeeAmountCall) -> Self {
            UniswapV3FactoryCalls::EnableFeeAmount(var)
        }
    }
    impl ::std::convert::From<FeeAmountTickSpacingCall> for UniswapV3FactoryCalls {
        fn from(var: FeeAmountTickSpacingCall) -> Self {
            UniswapV3FactoryCalls::FeeAmountTickSpacing(var)
        }
    }
    impl ::std::convert::From<GetPoolCall> for UniswapV3FactoryCalls {
        fn from(var: GetPoolCall) -> Self {
            UniswapV3FactoryCalls::GetPool(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for UniswapV3FactoryCalls {
        fn from(var: OwnerCall) -> Self {
            UniswapV3FactoryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<ParametersCall> for UniswapV3FactoryCalls {
        fn from(var: ParametersCall) -> Self {
            UniswapV3FactoryCalls::Parameters(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for UniswapV3FactoryCalls {
        fn from(var: SetOwnerCall) -> Self {
            UniswapV3FactoryCalls::SetOwner(var)
        }
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(address,address,uint24)` and selector `0xa1671295`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct CreatePoolReturn {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `feeAmountTickSpacing` function with signature `feeAmountTickSpacing(uint24)` and selector `0x22afcccb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct FeeAmountTickSpacingReturn(pub i32);
    ///Container type for all return fields from the `getPool` function with signature `getPool(address,address,uint24)` and selector `0x1698ee82`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `parameters` function with signature `parameters()` and selector `0x89035730`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct ParametersReturn {
        pub factory: ::ethers::core::types::Address,
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub tick_spacing: i32,
    }
}
