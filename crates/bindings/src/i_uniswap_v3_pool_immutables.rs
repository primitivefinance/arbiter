pub use i_uniswap_v3_pool_immutables::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_uniswap_v3_pool_immutables {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///IUniswapV3PoolImmutables was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fee\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxLiquidityPerTick\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token0\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token1\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static IUNISWAPV3POOLIMMUTABLES_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
    pub struct IUniswapV3PoolImmutables<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV3PoolImmutables<M> {
        fn clone(&self) -> Self {
            IUniswapV3PoolImmutables(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV3PoolImmutables<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV3PoolImmutables<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolImmutables))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3PoolImmutables<M> {
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
                    IUNISWAPV3POOLIMMUTABLES_ABI.clone(),
                    client,
                ),
            )
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
        ///Calls the contract's `fee` (0xddca3f43) function
        pub fn fee(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxLiquidityPerTick` (0x70cf754a) function
        pub fn max_liquidity_per_tick(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 207, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tickSpacing` (0xd0c93a7c) function
        pub fn tick_spacing(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([208, 201, 58, 124], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IUniswapV3PoolImmutables<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `fee` function with signature `fee()` and selector `0xddca3f43`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    ///Container type for all input parameters for the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    ///Container type for all input parameters for the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
    ///Container type for all input parameters for the `token0` function with signature `token0()` and selector `0x0dfe1681`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    ///Container type for all input parameters for the `token1` function with signature `token1()` and selector `0xd21220a7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IUniswapV3PoolImmutablesCalls {
        Factory(FactoryCall),
        Fee(FeeCall),
        MaxLiquidityPerTick(MaxLiquidityPerTickCall),
        TickSpacing(TickSpacingCall),
        Token0(Token0Call),
        Token1(Token1Call),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV3PoolImmutablesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IUniswapV3PoolImmutablesCalls::Factory(decoded));
            }
            if let Ok(decoded)
                = <FeeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolImmutablesCalls::Fee(decoded));
            }
            if let Ok(decoded)
                = <MaxLiquidityPerTickCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded)
                = <TickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IUniswapV3PoolImmutablesCalls::TickSpacing(decoded));
            }
            if let Ok(decoded)
                = <Token0Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolImmutablesCalls::Token0(decoded));
            }
            if let Ok(decoded)
                = <Token1Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolImmutablesCalls::Token1(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV3PoolImmutablesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV3PoolImmutablesCalls::Factory(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::Fee(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(element) => {
                    element.encode()
                }
                IUniswapV3PoolImmutablesCalls::TickSpacing(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::Token0(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::Token1(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolImmutablesCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolImmutablesCalls::Factory(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::Fee(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(element) => {
                    element.fmt(f)
                }
                IUniswapV3PoolImmutablesCalls::TickSpacing(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::Token0(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::Token1(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FactoryCall> for IUniswapV3PoolImmutablesCalls {
        fn from(var: FactoryCall) -> Self {
            IUniswapV3PoolImmutablesCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FeeCall> for IUniswapV3PoolImmutablesCalls {
        fn from(var: FeeCall) -> Self {
            IUniswapV3PoolImmutablesCalls::Fee(var)
        }
    }
    impl ::std::convert::From<MaxLiquidityPerTickCall>
    for IUniswapV3PoolImmutablesCalls {
        fn from(var: MaxLiquidityPerTickCall) -> Self {
            IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(var)
        }
    }
    impl ::std::convert::From<TickSpacingCall> for IUniswapV3PoolImmutablesCalls {
        fn from(var: TickSpacingCall) -> Self {
            IUniswapV3PoolImmutablesCalls::TickSpacing(var)
        }
    }
    impl ::std::convert::From<Token0Call> for IUniswapV3PoolImmutablesCalls {
        fn from(var: Token0Call) -> Self {
            IUniswapV3PoolImmutablesCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for IUniswapV3PoolImmutablesCalls {
        fn from(var: Token1Call) -> Self {
            IUniswapV3PoolImmutablesCalls::Token1(var)
        }
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `fee` function with signature `fee()` and selector `0xddca3f43`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct FeeReturn(pub u32);
    ///Container type for all return fields from the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct MaxLiquidityPerTickReturn(pub u128);
    ///Container type for all return fields from the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TickSpacingReturn(pub i32);
    ///Container type for all return fields from the `token0` function with signature `token0()` and selector `0x0dfe1681`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct Token0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token1` function with signature `token1()` and selector `0xd21220a7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct Token1Return(pub ::ethers::core::types::Address);
}
