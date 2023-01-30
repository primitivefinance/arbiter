pub use hello_world::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod hello_world {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///HelloWorld was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"greet\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"_greeting\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"greeting\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static HELLOWORLD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static HELLOWORLD_BYTECODE: ::ethers::contract::Lazy<
        ::ethers::core::types::Bytes,
    > = ::ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b5061011b806100206000396000f3fe6080604052348015600f57600080fd5b506004361060325760003560e01c8063cfae3217146037578063ef690cc014606f575b600080fd5b60408051808201909152600c81526b48656c6c6f20576f726c642160a01b60208201525b604051606691906099565b60405180910390f35b605b6040518060400160405280600c81526020016b48656c6c6f20576f726c642160a01b81525081565b600060208083528351808285015260005b8181101560c45785810183015185820160400152820160aa565b506000604082860101526040601f19601f830116850101925050509291505056fea2646970667358221220bc825a97787473d67ec9151ab3bc989aece73443482cbbfb03d050686936127e64736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct HelloWorld<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for HelloWorld<M> {
        fn clone(&self) -> Self {
            HelloWorld(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for HelloWorld<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for HelloWorld<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(HelloWorld)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HelloWorld<M> {
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
                    HELLOWORLD_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// 1. If there are no constructor arguments, you should pass `()` as the argument.
        /// 1. The default poll duration is 7 seconds.
        /// 1. The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter,"../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                HELLOWORLD_ABI.clone(),
                HELLOWORLD_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `greet` (0xcfae3217) function
        pub fn greet(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([207, 174, 50, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `greeting` (0xef690cc0) function
        pub fn greeting(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([239, 105, 12, 192], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HelloWorld<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `greet` function with signature `greet()` and selector `0xcfae3217`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "greet", abi = "greet()")]
    pub struct GreetCall;
    ///Container type for all input parameters for the `greeting` function with signature `greeting()` and selector `0xef690cc0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "greeting", abi = "greeting()")]
    pub struct GreetingCall;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum HelloWorldCalls {
        Greet(GreetCall),
        Greeting(GreetingCall),
    }
    impl ::ethers::core::abi::AbiDecode for HelloWorldCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <GreetCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(HelloWorldCalls::Greet(decoded));
            }
            if let Ok(decoded)
                = <GreetingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(HelloWorldCalls::Greeting(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HelloWorldCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                HelloWorldCalls::Greet(element) => element.encode(),
                HelloWorldCalls::Greeting(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for HelloWorldCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                HelloWorldCalls::Greet(element) => element.fmt(f),
                HelloWorldCalls::Greeting(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GreetCall> for HelloWorldCalls {
        fn from(var: GreetCall) -> Self {
            HelloWorldCalls::Greet(var)
        }
    }
    impl ::std::convert::From<GreetingCall> for HelloWorldCalls {
        fn from(var: GreetingCall) -> Self {
            HelloWorldCalls::Greeting(var)
        }
    }
    ///Container type for all return fields from the `greet` function with signature `greet()` and selector `0xcfae3217`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GreetReturn {
        pub greeting: String,
    }
    ///Container type for all return fields from the `greeting` function with signature `greeting()` and selector `0xef690cc0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GreetingReturn(pub String);
}
