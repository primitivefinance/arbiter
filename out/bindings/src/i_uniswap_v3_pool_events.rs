pub use iuniswapv3poolevents_mod::*;
#[allow(clippy::too_many_arguments)]
mod iuniswapv3poolevents_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IUniswapV3PoolEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IUNISWAPV3POOLEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount0\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"amount1\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Collect\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount0\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"amount1\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollectProtocol\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"paid0\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"paid1\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Flash\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"observationCardinalityNextOld\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"observationCardinalityNextNew\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IncreaseObservationCardinalityNext\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialize\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol0Old\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"feeProtocol1Old\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"feeProtocol0New\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"feeProtocol1New\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetFeeProtocol\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Swap\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IUNISWAPV3POOLEVENTS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IUniswapV3PoolEvents<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IUniswapV3PoolEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IUniswapV3PoolEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IUniswapV3PoolEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV3POOLEVENTS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                IUNISWAPV3POOLEVENTS_ABI.clone(),
                IUNISWAPV3POOLEVENTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Collect` event"]
        pub fn collect_filter(&self) -> ethers::contract::builders::Event<M, CollectFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollectProtocol` event"]
        pub fn collect_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollectProtocolFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Flash` event"]
        pub fn flash_filter(&self) -> ethers::contract::builders::Event<M, FlashFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreaseObservationCardinalityNext` event"]
        pub fn increase_observation_cardinality_next_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IncreaseObservationCardinalityNextFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialize` event"]
        pub fn initialize_filter(&self) -> ethers::contract::builders::Event<M, InitializeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetFeeProtocol` event"]
        pub fn set_fee_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetFeeProtocolFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IUniswapV3PoolEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IUniswapV3PoolEvents<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "Burn",
        abi = "Burn(address,int24,int24,uint128,uint256,uint256)"
    )]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "Collect",
        abi = "Collect(address,address,int24,int24,uint128,uint128)"
    )]
    pub struct CollectFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "CollectProtocol",
        abi = "CollectProtocol(address,address,uint128,uint128)"
    )]
    pub struct CollectProtocolFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "Flash",
        abi = "Flash(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct FlashFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub paid_0: ethers::core::types::U256,
        pub paid_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "IncreaseObservationCardinalityNext",
        abi = "IncreaseObservationCardinalityNext(uint16,uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextFilter {
        pub observation_cardinality_next_old: u16,
        pub observation_cardinality_next_new: u16,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Initialize", abi = "Initialize(uint160,int24)")]
    pub struct InitializeFilter {
        pub sqrt_price_x96: ethers::core::types::U256,
        pub tick: i32,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "Mint",
        abi = "Mint(address,address,int24,int24,uint128,uint256,uint256)"
    )]
    pub struct MintFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "SetFeeProtocol",
        abi = "SetFeeProtocol(uint8,uint8,uint8,uint8)"
    )]
    pub struct SetFeeProtocolFilter {
        pub fee_protocol_0_old: u8,
        pub fee_protocol_1_old: u8,
        pub fee_protocol_0_new: u8,
        pub fee_protocol_1_new: u8,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,int256,int256,uint160,uint128,int24)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: I256,
        pub amount_1: I256,
        pub sqrt_price_x96: ethers::core::types::U256,
        pub liquidity: u128,
        pub tick: i32,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IUniswapV3PoolEventsEvents {
        BurnFilter(BurnFilter),
        CollectFilter(CollectFilter),
        CollectProtocolFilter(CollectProtocolFilter),
        FlashFilter(FlashFilter),
        IncreaseObservationCardinalityNextFilter(IncreaseObservationCardinalityNextFilter),
        InitializeFilter(InitializeFilter),
        MintFilter(MintFilter),
        SetFeeProtocolFilter(SetFeeProtocolFilter),
        SwapFilter(SwapFilter),
    }
    impl ethers::contract::EthLogDecode for IUniswapV3PoolEventsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = CollectFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::CollectFilter(decoded));
            }
            if let Ok(decoded) = CollectProtocolFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::CollectProtocolFilter(decoded));
            }
            if let Ok(decoded) = FlashFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::FlashFilter(decoded));
            }
            if let Ok(decoded) = IncreaseObservationCardinalityNextFilter::decode_log(log) {
                return Ok(
                    IUniswapV3PoolEventsEvents::IncreaseObservationCardinalityNextFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::InitializeFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SetFeeProtocolFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::SetFeeProtocolFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEventsEvents::SwapFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolEventsEvents::BurnFilter(element) => element.fmt(f),
                IUniswapV3PoolEventsEvents::CollectFilter(element) => element.fmt(f),
                IUniswapV3PoolEventsEvents::CollectProtocolFilter(element) => element.fmt(f),
                IUniswapV3PoolEventsEvents::FlashFilter(element) => element.fmt(f),
                IUniswapV3PoolEventsEvents::IncreaseObservationCardinalityNextFilter(element) => {
                    element.fmt(f)
                }
                IUniswapV3PoolEventsEvents::InitializeFilter(element) => element.fmt(f),
                IUniswapV3PoolEventsEvents::MintFilter(element) => element.fmt(f),
                IUniswapV3PoolEventsEvents::SetFeeProtocolFilter(element) => element.fmt(f),
                IUniswapV3PoolEventsEvents::SwapFilter(element) => element.fmt(f),
            }
        }
    }
}
