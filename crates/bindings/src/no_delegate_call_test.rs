pub use no_delegate_call_test::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod no_delegate_call_test {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"callsIntoNoDelegateCallFunction\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"canBeDelegateCalled\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cannotBeDelegateCalled\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getGasCostOfCanBeDelegateCalled\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getGasCostOfCannotBeDelegateCalled\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static NODELEGATECALLTEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        160,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        48,
        96,
        96,
        129,
        144,
        27,
        96,
        128,
        82,
        97,
        1,
        84,
        97,
        0,
        48,
        96,
        0,
        57,
        128,
        96,
        236,
        82,
        80,
        97,
        1,
        84,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        87,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        48,
        69,
        166,
        224,
        20,
        97,
        0,
        92,
        87,
        128,
        99,
        60,
        124,
        141,
        208,
        20,
        97,
        0,
        118,
        87,
        128,
        99,
        66,
        62,
        203,
        5,
        20,
        97,
        0,
        126,
        87,
        128,
        99,
        100,
        39,
        1,
        100,
        20,
        97,
        0,
        134,
        87,
        128,
        99,
        244,
        95,
        65,
        110,
        20,
        97,
        0,
        144,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        100,
        97,
        0,
        152,
        86,
        91,
        96,
        64,
        128,
        81,
        145,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        97,
        0,
        100,
        97,
        0,
        175,
        86,
        91,
        97,
        0,
        100,
        97,
        0,
        185,
        86,
        91,
        97,
        0,
        142,
        97,
        0,
        205,
        86,
        91,
        0,
        91,
        97,
        0,
        100,
        97,
        0,
        215,
        86,
        91,
        96,
        0,
        128,
        90,
        144,
        80,
        97,
        0,
        166,
        97,
        0,
        215,
        86,
        91,
        80,
        90,
        144,
        3,
        144,
        80,
        144,
        86,
        91,
        96,
        0,
        128,
        90,
        144,
        80,
        97,
        0,
        166,
        91,
        96,
        0,
        97,
        0,
        195,
        97,
        0,
        225,
        86,
        91,
        96,
        5,
        66,
        91,
        4,
        144,
        80,
        144,
        86,
        91,
        97,
        0,
        213,
        97,
        1,
        22,
        86,
        91,
        86,
        91,
        96,
        0,
        96,
        5,
        66,
        97,
        0,
        199,
        86,
        91,
        48,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        22,
        20,
        97,
        0,
        213,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        213,
        97,
        0,
        225,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        186,
        30,
        105,
        52,
        45,
        249,
        210,
        226,
        197,
        251,
        250,
        86,
        98,
        2,
        255,
        239,
        64,
        191,
        76,
        215,
        254,
        115,
        136,
        117,
        235,
        245,
        114,
        78,
        23,
        19,
        249,
        135,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        7,
        6,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static NODELEGATECALLTEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        87,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        48,
        69,
        166,
        224,
        20,
        97,
        0,
        92,
        87,
        128,
        99,
        60,
        124,
        141,
        208,
        20,
        97,
        0,
        118,
        87,
        128,
        99,
        66,
        62,
        203,
        5,
        20,
        97,
        0,
        126,
        87,
        128,
        99,
        100,
        39,
        1,
        100,
        20,
        97,
        0,
        134,
        87,
        128,
        99,
        244,
        95,
        65,
        110,
        20,
        97,
        0,
        144,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        100,
        97,
        0,
        152,
        86,
        91,
        96,
        64,
        128,
        81,
        145,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        97,
        0,
        100,
        97,
        0,
        175,
        86,
        91,
        97,
        0,
        100,
        97,
        0,
        185,
        86,
        91,
        97,
        0,
        142,
        97,
        0,
        205,
        86,
        91,
        0,
        91,
        97,
        0,
        100,
        97,
        0,
        215,
        86,
        91,
        96,
        0,
        128,
        90,
        144,
        80,
        97,
        0,
        166,
        97,
        0,
        215,
        86,
        91,
        80,
        90,
        144,
        3,
        144,
        80,
        144,
        86,
        91,
        96,
        0,
        128,
        90,
        144,
        80,
        97,
        0,
        166,
        91,
        96,
        0,
        97,
        0,
        195,
        97,
        0,
        225,
        86,
        91,
        96,
        5,
        66,
        91,
        4,
        144,
        80,
        144,
        86,
        91,
        97,
        0,
        213,
        97,
        1,
        22,
        86,
        91,
        86,
        91,
        96,
        0,
        96,
        5,
        66,
        97,
        0,
        199,
        86,
        91,
        48,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        22,
        20,
        97,
        0,
        213,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        213,
        97,
        0,
        225,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        186,
        30,
        105,
        52,
        45,
        249,
        210,
        226,
        197,
        251,
        250,
        86,
        98,
        2,
        255,
        239,
        64,
        191,
        76,
        215,
        254,
        115,
        136,
        117,
        235,
        245,
        114,
        78,
        23,
        19,
        249,
        135,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        7,
        6,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static NODELEGATECALLTEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct NoDelegateCallTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NoDelegateCallTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NoDelegateCallTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NoDelegateCallTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NoDelegateCallTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(NoDelegateCallTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NoDelegateCallTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                NODELEGATECALLTEST_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
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
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                NODELEGATECALLTEST_ABI.clone(),
                NODELEGATECALLTEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `callsIntoNoDelegateCallFunction` (0x64270164) function
        pub fn calls_into_no_delegate_call_function(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 39, 1, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `canBeDelegateCalled` (0xf45f416e) function
        pub fn can_be_delegate_called(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 95, 65, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cannotBeDelegateCalled` (0x423ecb05) function
        pub fn cannot_be_delegate_called(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 62, 203, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGasCostOfCanBeDelegateCalled` (0x3045a6e0) function
        pub fn get_gas_cost_of_can_be_delegate_called(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 69, 166, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGasCostOfCannotBeDelegateCalled` (0x3c7c8dd0) function
        pub fn get_gas_cost_of_cannot_be_delegate_called(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 124, 141, 208], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for NoDelegateCallTest<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `callsIntoNoDelegateCallFunction` function with signature `callsIntoNoDelegateCallFunction()` and selector `0x64270164`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "callsIntoNoDelegateCallFunction",
        abi = "callsIntoNoDelegateCallFunction()"
    )]
    pub struct CallsIntoNoDelegateCallFunctionCall;
    ///Container type for all input parameters for the `canBeDelegateCalled` function with signature `canBeDelegateCalled()` and selector `0xf45f416e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "canBeDelegateCalled", abi = "canBeDelegateCalled()")]
    pub struct CanBeDelegateCalledCall;
    ///Container type for all input parameters for the `cannotBeDelegateCalled` function with signature `cannotBeDelegateCalled()` and selector `0x423ecb05`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "cannotBeDelegateCalled", abi = "cannotBeDelegateCalled()")]
    pub struct CannotBeDelegateCalledCall;
    ///Container type for all input parameters for the `getGasCostOfCanBeDelegateCalled` function with signature `getGasCostOfCanBeDelegateCalled()` and selector `0x3045a6e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getGasCostOfCanBeDelegateCalled",
        abi = "getGasCostOfCanBeDelegateCalled()"
    )]
    pub struct GetGasCostOfCanBeDelegateCalledCall;
    ///Container type for all input parameters for the `getGasCostOfCannotBeDelegateCalled` function with signature `getGasCostOfCannotBeDelegateCalled()` and selector `0x3c7c8dd0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getGasCostOfCannotBeDelegateCalled",
        abi = "getGasCostOfCannotBeDelegateCalled()"
    )]
    pub struct GetGasCostOfCannotBeDelegateCalledCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum NoDelegateCallTestCalls {
        CallsIntoNoDelegateCallFunction(CallsIntoNoDelegateCallFunctionCall),
        CanBeDelegateCalled(CanBeDelegateCalledCall),
        CannotBeDelegateCalled(CannotBeDelegateCalledCall),
        GetGasCostOfCanBeDelegateCalled(GetGasCostOfCanBeDelegateCalledCall),
        GetGasCostOfCannotBeDelegateCalled(GetGasCostOfCannotBeDelegateCalledCall),
    }
    impl ::ethers::core::abi::AbiDecode for NoDelegateCallTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CallsIntoNoDelegateCallFunctionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CallsIntoNoDelegateCallFunction(decoded));
            }
            if let Ok(decoded) =
                <CanBeDelegateCalledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CanBeDelegateCalled(decoded));
            }
            if let Ok(decoded) =
                <CannotBeDelegateCalledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotBeDelegateCalled(decoded));
            }
            if let Ok(decoded) =
                <GetGasCostOfCanBeDelegateCalledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetGasCostOfCanBeDelegateCalled(decoded));
            }
            if let Ok(decoded) =
                <GetGasCostOfCannotBeDelegateCalledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetGasCostOfCannotBeDelegateCalled(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NoDelegateCallTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CallsIntoNoDelegateCallFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanBeDelegateCalled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotBeDelegateCalled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGasCostOfCanBeDelegateCalled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGasCostOfCannotBeDelegateCalled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for NoDelegateCallTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallsIntoNoDelegateCallFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanBeDelegateCalled(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotBeDelegateCalled(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGasCostOfCanBeDelegateCalled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGasCostOfCannotBeDelegateCalled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CallsIntoNoDelegateCallFunctionCall> for NoDelegateCallTestCalls {
        fn from(value: CallsIntoNoDelegateCallFunctionCall) -> Self {
            Self::CallsIntoNoDelegateCallFunction(value)
        }
    }
    impl ::core::convert::From<CanBeDelegateCalledCall> for NoDelegateCallTestCalls {
        fn from(value: CanBeDelegateCalledCall) -> Self {
            Self::CanBeDelegateCalled(value)
        }
    }
    impl ::core::convert::From<CannotBeDelegateCalledCall> for NoDelegateCallTestCalls {
        fn from(value: CannotBeDelegateCalledCall) -> Self {
            Self::CannotBeDelegateCalled(value)
        }
    }
    impl ::core::convert::From<GetGasCostOfCanBeDelegateCalledCall> for NoDelegateCallTestCalls {
        fn from(value: GetGasCostOfCanBeDelegateCalledCall) -> Self {
            Self::GetGasCostOfCanBeDelegateCalled(value)
        }
    }
    impl ::core::convert::From<GetGasCostOfCannotBeDelegateCalledCall> for NoDelegateCallTestCalls {
        fn from(value: GetGasCostOfCannotBeDelegateCalledCall) -> Self {
            Self::GetGasCostOfCannotBeDelegateCalled(value)
        }
    }
    ///Container type for all return fields from the `canBeDelegateCalled` function with signature `canBeDelegateCalled()` and selector `0xf45f416e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CanBeDelegateCalledReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `cannotBeDelegateCalled` function with signature `cannotBeDelegateCalled()` and selector `0x423ecb05`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CannotBeDelegateCalledReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getGasCostOfCanBeDelegateCalled` function with signature `getGasCostOfCanBeDelegateCalled()` and selector `0x3045a6e0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetGasCostOfCanBeDelegateCalledReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getGasCostOfCannotBeDelegateCalled` function with signature `getGasCostOfCannotBeDelegateCalled()` and selector `0x3c7c8dd0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetGasCostOfCannotBeDelegateCalledReturn(pub ::ethers::core::types::U256);
}
