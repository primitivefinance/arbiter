pub use iuniswapv3poolowneractions_mod::*;
#[allow(clippy::too_many_arguments)]
mod iuniswapv3poolowneractions_mod {
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
    #[doc = "IUniswapV3PoolOwnerActions was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IUNISWAPV3POOLOWNERACTIONS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount0Requested\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1Requested\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collectProtocol\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"amount0\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount1\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol0\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"feeProtocol1\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeProtocol\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IUNISWAPV3POOLOWNERACTIONS_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IUniswapV3PoolOwnerActions<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IUniswapV3PoolOwnerActions<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IUniswapV3PoolOwnerActions<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolOwnerActions))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IUniswapV3PoolOwnerActions<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV3POOLOWNERACTIONS_ABI.clone(),
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
                IUNISWAPV3POOLOWNERACTIONS_ABI.clone(),
                IUNISWAPV3POOLOWNERACTIONS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `collectProtocol` (0x85b66729) function"]
        pub fn collect_protocol(
            &self,
            recipient: ethers::core::types::Address,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [133, 182, 103, 41],
                    (recipient, amount_0_requested, amount_1_requested),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeProtocol` (0x8206a4d1) function"]
        pub fn set_fee_protocol(
            &self,
            fee_protocol_0: u8,
            fee_protocol_1: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 6, 164, 209], (fee_protocol_0, fee_protocol_1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IUniswapV3PoolOwnerActions<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `collectProtocol`function with signature `collectProtocol(address,uint128,uint128)` and selector `[133, 182, 103, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "collectProtocol",
        abi = "collectProtocol(address,uint128,uint128)"
    )]
    pub struct CollectProtocolCall {
        pub recipient: ethers::core::types::Address,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    #[doc = "Container type for all input parameters for the `setFeeProtocol`function with signature `setFeeProtocol(uint8,uint8)` and selector `[130, 6, 164, 209]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8,uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol_0: u8,
        pub fee_protocol_1: u8,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IUniswapV3PoolOwnerActionsCalls {
        CollectProtocol(CollectProtocolCall),
        SetFeeProtocol(SetFeeProtocolCall),
    }
    impl ethers::core::abi::AbiDecode for IUniswapV3PoolOwnerActionsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CollectProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolOwnerActionsCalls::CollectProtocol(decoded));
            }
            if let Ok(decoded) =
                <SetFeeProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolOwnerActionsCalls::SetFeeProtocol(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IUniswapV3PoolOwnerActionsCalls {
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
}
