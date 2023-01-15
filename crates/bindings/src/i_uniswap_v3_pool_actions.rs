pub use i_uniswap_v3_pool_actions::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_uniswap_v3_pool_actions {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    #[doc = "IUniswapV3PoolActions was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;

    use ethers::{
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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount0Requested\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1Requested\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collect\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"amount0\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"flash\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"observationCardinalityNext\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseObservationCardinalityNext\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"zeroForOne\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amountSpecified\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IUNISWAPV3POOLACTIONS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IUniswapV3PoolActions<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV3PoolActions<M> {
        fn clone(&self) -> Self {
            IUniswapV3PoolActions(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV3PoolActions<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV3PoolActions<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolActions))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IUniswapV3PoolActions<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV3POOLACTIONS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `burn` (0xa34123a7) function"]
        pub fn burn(
            &self,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([163, 65, 35, 167], (tick_lower, tick_upper, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collect` (0x4f1eb3d8) function"]
        pub fn collect(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [79, 30, 179, 216],
                    (
                        recipient,
                        tick_lower,
                        tick_upper,
                        amount_0_requested,
                        amount_1_requested,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flash` (0x490e6cbc) function"]
        pub fn flash(
            &self,
            recipient: ethers::core::types::Address,
            amount_0: ethers::core::types::U256,
            amount_1: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 14, 108, 188], (recipient, amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseObservationCardinalityNext` (0x32148f67) function"]
        pub fn increase_observation_cardinality_next(
            &self,
            observation_cardinality_next: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 20, 143, 103], observation_cardinality_next)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xf637731d) function"]
        pub fn initialize(
            &self,
            sqrt_price_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 55, 115, 29], sqrt_price_x96)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x3c8a7d8d) function"]
        pub fn mint(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [60, 138, 125, 141],
                    (recipient, tick_lower, tick_upper, amount, data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x128acb08) function"]
        pub fn swap(
            &self,
            recipient: ethers::core::types::Address,
            zero_for_one: bool,
            amount_specified: I256,
            sqrt_price_limit_x96: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, (I256, I256)> {
            self.0
                .method_hash(
                    [18, 138, 203, 8],
                    (
                        recipient,
                        zero_for_one,
                        amount_specified,
                        sqrt_price_limit_x96,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IUniswapV3PoolActions<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(int24,int24,uint128)` and selector `[163, 65, 35, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(int24,int24,uint128)")]
    pub struct BurnCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `[79, 30, 179, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "collect", abi = "collect(address,int24,int24,uint128,uint128)")]
    pub struct CollectCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    #[doc = "Container type for all input parameters for the `flash` function with signature `flash(address,uint256,uint256,bytes)` and selector `[73, 14, 108, 188]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "flash", abi = "flash(address,uint256,uint256,bytes)")]
    pub struct FlashCall {
        pub recipient: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `increaseObservationCardinalityNext` function with signature `increaseObservationCardinalityNext(uint16)` and selector `[50, 20, 143, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "increaseObservationCardinalityNext",
        abi = "increaseObservationCardinalityNext(uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextCall {
        pub observation_cardinality_next: u16,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint160)` and selector `[246, 55, 115, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint160)")]
    pub struct InitializeCall {
        pub sqrt_price_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `[60, 138, 125, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address,int24,int24,uint128,bytes)")]
    pub struct MintCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `[18, 138, 203, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swap", abi = "swap(address,bool,int256,uint160,bytes)")]
    pub struct SwapCall {
        pub recipient: ethers::core::types::Address,
        pub zero_for_one: bool,
        pub amount_specified: I256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IUniswapV3PoolActionsCalls {
        Burn(BurnCall),
        Collect(CollectCall),
        Flash(FlashCall),
        IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        Swap(SwapCall),
    }
    impl ethers::core::abi::AbiDecode for IUniswapV3PoolActionsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolActionsCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <CollectCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolActionsCalls::Collect(decoded));
            }
            if let Ok(decoded) = <FlashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolActionsCalls::Flash(decoded));
            }
            if let Ok(decoded) =
                <IncreaseObservationCardinalityNextCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniswapV3PoolActionsCalls::IncreaseObservationCardinalityNext(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolActionsCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolActionsCalls::Mint(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolActionsCalls::Swap(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IUniswapV3PoolActionsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV3PoolActionsCalls::Burn(element) => element.encode(),
                IUniswapV3PoolActionsCalls::Collect(element) => element.encode(),
                IUniswapV3PoolActionsCalls::Flash(element) => element.encode(),
                IUniswapV3PoolActionsCalls::IncreaseObservationCardinalityNext(element) => {
                    element.encode()
                }
                IUniswapV3PoolActionsCalls::Initialize(element) => element.encode(),
                IUniswapV3PoolActionsCalls::Mint(element) => element.encode(),
                IUniswapV3PoolActionsCalls::Swap(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolActionsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolActionsCalls::Burn(element) => element.fmt(f),
                IUniswapV3PoolActionsCalls::Collect(element) => element.fmt(f),
                IUniswapV3PoolActionsCalls::Flash(element) => element.fmt(f),
                IUniswapV3PoolActionsCalls::IncreaseObservationCardinalityNext(element) => {
                    element.fmt(f)
                }
                IUniswapV3PoolActionsCalls::Initialize(element) => element.fmt(f),
                IUniswapV3PoolActionsCalls::Mint(element) => element.fmt(f),
                IUniswapV3PoolActionsCalls::Swap(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BurnCall> for IUniswapV3PoolActionsCalls {
        fn from(var: BurnCall) -> Self {
            IUniswapV3PoolActionsCalls::Burn(var)
        }
    }
    impl ::std::convert::From<CollectCall> for IUniswapV3PoolActionsCalls {
        fn from(var: CollectCall) -> Self {
            IUniswapV3PoolActionsCalls::Collect(var)
        }
    }
    impl ::std::convert::From<FlashCall> for IUniswapV3PoolActionsCalls {
        fn from(var: FlashCall) -> Self {
            IUniswapV3PoolActionsCalls::Flash(var)
        }
    }
    impl ::std::convert::From<IncreaseObservationCardinalityNextCall> for IUniswapV3PoolActionsCalls {
        fn from(var: IncreaseObservationCardinalityNextCall) -> Self {
            IUniswapV3PoolActionsCalls::IncreaseObservationCardinalityNext(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IUniswapV3PoolActionsCalls {
        fn from(var: InitializeCall) -> Self {
            IUniswapV3PoolActionsCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MintCall> for IUniswapV3PoolActionsCalls {
        fn from(var: MintCall) -> Self {
            IUniswapV3PoolActionsCalls::Mint(var)
        }
    }
    impl ::std::convert::From<SwapCall> for IUniswapV3PoolActionsCalls {
        fn from(var: SwapCall) -> Self {
            IUniswapV3PoolActionsCalls::Swap(var)
        }
    }
    #[doc = "Container type for all return fields from the `burn` function with signature `burn(int24,int24,uint128)` and selector `[163, 65, 35, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BurnReturn {
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `[79, 30, 179, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CollectReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `[60, 138, 125, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintReturn {
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `[18, 138, 203, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapReturn {
        pub amount_0: I256,
        pub amount_1: I256,
    }
}
