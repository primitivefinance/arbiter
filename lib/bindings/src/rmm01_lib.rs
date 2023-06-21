pub use rmm01_lib::*;
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
pub mod rmm01_lib {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"wad\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"OverflowWad\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UndefinedPrice\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static RMM01LIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        178,
        96,
        55,
        96,
        11,
        130,
        130,
        130,
        57,
        128,
        81,
        96,
        0,
        26,
        96,
        115,
        20,
        96,
        42,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        0,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        48,
        96,
        0,
        82,
        96,
        115,
        129,
        83,
        130,
        129,
        243,
        254,
        115,
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
        96,
        128,
        96,
        64,
        129,
        144,
        82,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        132,
        144,
        129,
        82,
        96,
        53,
        96,
        164,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        196,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        228,
        82,
        48,
        147,
        144,
        147,
        20,
        146,
        144,
        130,
        253,
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
        224,
        86,
        89,
        233,
        133,
        39,
        86,
        110,
        133,
        217,
        169,
        231,
        169,
        79,
        102,
        175,
        3,
        15,
        128,
        45,
        17,
        86,
        125,
        2,
        241,
        190,
        134,
        175,
        76,
        200,
        140,
        93,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static RMM01LIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        115,
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
        96,
        128,
        96,
        64,
        129,
        144,
        82,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        132,
        144,
        129,
        82,
        96,
        53,
        96,
        164,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        196,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        228,
        82,
        48,
        147,
        144,
        147,
        20,
        146,
        144,
        130,
        253,
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
        224,
        86,
        89,
        233,
        133,
        39,
        86,
        110,
        133,
        217,
        169,
        231,
        169,
        79,
        102,
        175,
        3,
        15,
        128,
        45,
        17,
        86,
        125,
        2,
        241,
        190,
        134,
        175,
        76,
        200,
        140,
        93,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static RMM01LIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RMM01Lib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RMM01Lib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RMM01Lib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RMM01Lib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RMM01Lib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(RMM01Lib)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RMM01Lib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RMM01LIB_ABI.clone(),
                    client,
                ),
            )
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
                RMM01LIB_ABI.clone(),
                RMM01LIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RMM01Lib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `OverflowWad` with signature `OverflowWad(int256)` and selector `0xb11558df`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OverflowWad", abi = "OverflowWad(int256)")]
    pub struct OverflowWad {
        pub wad: ::ethers::core::types::I256,
    }
    ///Custom Error type `UndefinedPrice` with signature `UndefinedPrice()` and selector `0x22053363`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UndefinedPrice", abi = "UndefinedPrice()")]
    pub struct UndefinedPrice;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RMM01LibErrors {
        OverflowWad(OverflowWad),
        UndefinedPrice(UndefinedPrice),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RMM01LibErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <OverflowWad as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OverflowWad(decoded));
            }
            if let Ok(decoded)
                = <UndefinedPrice as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UndefinedPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RMM01LibErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::OverflowWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UndefinedPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RMM01LibErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <OverflowWad as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UndefinedPrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RMM01LibErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OverflowWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::UndefinedPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RMM01LibErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<OverflowWad> for RMM01LibErrors {
        fn from(value: OverflowWad) -> Self {
            Self::OverflowWad(value)
        }
    }
    impl ::core::convert::From<UndefinedPrice> for RMM01LibErrors {
        fn from(value: UndefinedPrice) -> Self {
            Self::UndefinedPrice(value)
        }
    }
}
