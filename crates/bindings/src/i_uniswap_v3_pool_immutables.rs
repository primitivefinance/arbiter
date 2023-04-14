pub use i_uniswap_v3_pool_immutables::*;
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
pub mod i_uniswap_v3_pool_immutables {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fee\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxLiquidityPerTick\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token0\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token1\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUNISWAPV3POOLIMMUTABLES_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IUniswapV3PoolImmutables<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniswapV3PoolImmutables<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniswapV3PoolImmutables<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniswapV3PoolImmutables<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniswapV3PoolImmutables<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolImmutables))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3PoolImmutables<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
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
    ///Container type for all input parameters for the `fee` function with signature `fee()` and selector `0xddca3f43`
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
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    ///Container type for all input parameters for the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
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
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    ///Container type for all input parameters for the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
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
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
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
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded)
                = <FeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fee(decoded));
            }
            if let Ok(decoded)
                = <MaxLiquidityPerTickCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded)
                = <TickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TickSpacing(decoded));
            }
            if let Ok(decoded)
                = <Token0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token0(decoded));
            }
            if let Ok(decoded)
                = <Token1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token1(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV3PoolImmutablesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxLiquidityPerTick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickSpacing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token1(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IUniswapV3PoolImmutablesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxLiquidityPerTick(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TickSpacing(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token1(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FactoryCall> for IUniswapV3PoolImmutablesCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<FeeCall> for IUniswapV3PoolImmutablesCalls {
        fn from(value: FeeCall) -> Self {
            Self::Fee(value)
        }
    }
    impl ::core::convert::From<MaxLiquidityPerTickCall>
    for IUniswapV3PoolImmutablesCalls {
        fn from(value: MaxLiquidityPerTickCall) -> Self {
            Self::MaxLiquidityPerTick(value)
        }
    }
    impl ::core::convert::From<TickSpacingCall> for IUniswapV3PoolImmutablesCalls {
        fn from(value: TickSpacingCall) -> Self {
            Self::TickSpacing(value)
        }
    }
    impl ::core::convert::From<Token0Call> for IUniswapV3PoolImmutablesCalls {
        fn from(value: Token0Call) -> Self {
            Self::Token0(value)
        }
    }
    impl ::core::convert::From<Token1Call> for IUniswapV3PoolImmutablesCalls {
        fn from(value: Token1Call) -> Self {
            Self::Token1(value)
        }
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
        Hash
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `fee` function with signature `fee()` and selector `0xddca3f43`
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
    pub struct FeeReturn(pub u32);
    ///Container type for all return fields from the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
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
    pub struct MaxLiquidityPerTickReturn(pub u128);
    ///Container type for all return fields from the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
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
    pub struct TickSpacingReturn(pub i32);
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
}
