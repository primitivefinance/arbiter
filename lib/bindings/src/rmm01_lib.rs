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
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDifference"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidDifference"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("difference"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidQuotient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidQuotient"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quotient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OverflowWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OverflowWad"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UndefinedPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UndefinedPrice"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RMM01LIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xB2`7`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`*WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80`@\x81\x90RbF\x1B\xCD`\xE5\x1B\x81R` `\x84\x90\x81R`5`\xA4R\x7FContract does not have fallback `\xC4\x90\x81Rtnor receive functions`X\x1B`\xE4R0\x93\x90\x93\x14\x92\x90\x82\xFD\xFE\xA2dipfsX\"\x12 \xD3\x04\xDA\xDC\x8A\xB2\xF2z(\xF3\xE9\xB0\x9D\xE0\0N\xB5\xC0ZI`\xD6ue\xF1\xD2\xAE\xB8i6n\xBCdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static RMM01LIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80`@\x81\x90RbF\x1B\xCD`\xE5\x1B\x81R` `\x84\x90\x81R`5`\xA4R\x7FContract does not have fallback `\xC4\x90\x81Rtnor receive functions`X\x1B`\xE4R0\x93\x90\x93\x14\x92\x90\x82\xFD\xFE\xA2dipfsX\"\x12 \xD3\x04\xDA\xDC\x8A\xB2\xF2z(\xF3\xE9\xB0\x9D\xE0\0N\xB5\xC0ZI`\xD6ue\xF1\xD2\xAE\xB8i6n\xBCdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
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
            f.debug_tuple(::core::stringify!(RMM01Lib)).field(&self.address()).finish()
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
    ///Custom Error type `InvalidDifference` with signature `InvalidDifference(uint256)` and selector `0x8676c25a`
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
    #[etherror(name = "InvalidDifference", abi = "InvalidDifference(uint256)")]
    pub struct InvalidDifference {
        pub difference: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidQuotient` with signature `InvalidQuotient(uint256)` and selector `0xca88d259`
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
    #[etherror(name = "InvalidQuotient", abi = "InvalidQuotient(uint256)")]
    pub struct InvalidQuotient {
        pub quotient: ::ethers::core::types::U256,
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
        InvalidDifference(InvalidDifference),
        InvalidQuotient(InvalidQuotient),
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
                = <InvalidDifference as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidDifference(decoded));
            }
            if let Ok(decoded)
                = <InvalidQuotient as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidQuotient(decoded));
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
                Self::InvalidDifference(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidQuotient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                    == <InvalidDifference as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidQuotient as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                Self::InvalidDifference(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidQuotient(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<InvalidDifference> for RMM01LibErrors {
        fn from(value: InvalidDifference) -> Self {
            Self::InvalidDifference(value)
        }
    }
    impl ::core::convert::From<InvalidQuotient> for RMM01LibErrors {
        fn from(value: InvalidQuotient) -> Self {
            Self::InvalidQuotient(value)
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
