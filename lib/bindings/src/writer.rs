pub use writer::*;
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
pub mod writer {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("echoString"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("echoString"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("test_string"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::string::String::new(),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("WasWritten"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("WasWritten"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                        name: ::std::borrow::ToOwned::to_owned("test_string"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        indexed: false,
                    },],
                    anonymous: false,
                },],
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static WRITER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x05\xCE\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c\r~/\xCE\x14a\0\xDDW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\0\xF0a\0\xEB6`\x04a\x03.V[a\x01\x06V[`@Qa\0\xFD\x91\x90a\x04\x01V[`@Q\x80\x91\x03\x90\xF3[```\0a\x01\x14\x83\x82a\x04\xD8V[P\x7F\x96o\xEB\x7F\x160\xFC\xBC\x94\x9Aye\xB9\x8B\xCB\xCB\x98#p\x9Ft\xD9\xEC\nT\x82\xB0\xC30\x94\x8B\x94\x82`@Qa\x01D\x91\x90a\x04\x01V[`@Q\x80\x91\x03\x90\xA1`\0\x80Ta\x01Y\x90a\x04OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\x85\x90a\x04OV[\x80\x15a\x01\xD2W\x80`\x1F\x10a\x01\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xD2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x02eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x80Wa\x02\x80a\x01\xDEV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x02\xA8Wa\x02\xA8a\x01\xDEV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x03\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[a\x03$\x84` \x83\x01` \x89\x01a\x01\xF4V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x03\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[a\x03\xF9\x84\x82\x85\x01a\x02\0V[\x94\x93PPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x04.W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\x12V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04cW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\x83WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x04\xD3W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x04\xB0WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04\xCFW\x82\x81U`\x01\x01a\x04\xBCV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xF2Wa\x04\xF2a\x01\xDEV[a\x05\x06\x81a\x05\0\x84Ta\x04OV[\x84a\x04\x89V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x05;W`\0\x84\x15a\x05#WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x04\xCFV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x05jW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x05KV[P\x85\x82\x10\x15a\x05\x88W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x90\x1F\xAF(-\xBA>\x92\x07\0\xF6\xF7\xA6i\x84\xCCl\x13\xAFE?\xD0\xEF7\x13\x1E\xC0e~Jg\x0BdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static WRITER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c\r~/\xCE\x14a\0\xDDW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\0\xF0a\0\xEB6`\x04a\x03.V[a\x01\x06V[`@Qa\0\xFD\x91\x90a\x04\x01V[`@Q\x80\x91\x03\x90\xF3[```\0a\x01\x14\x83\x82a\x04\xD8V[P\x7F\x96o\xEB\x7F\x160\xFC\xBC\x94\x9Aye\xB9\x8B\xCB\xCB\x98#p\x9Ft\xD9\xEC\nT\x82\xB0\xC30\x94\x8B\x94\x82`@Qa\x01D\x91\x90a\x04\x01V[`@Q\x80\x91\x03\x90\xA1`\0\x80Ta\x01Y\x90a\x04OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\x85\x90a\x04OV[\x80\x15a\x01\xD2W\x80`\x1F\x10a\x01\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xD2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x02eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x80Wa\x02\x80a\x01\xDEV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x02\xA8Wa\x02\xA8a\x01\xDEV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x03\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[a\x03$\x84` \x83\x01` \x89\x01a\x01\xF4V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x03\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[a\x03\xF9\x84\x82\x85\x01a\x02\0V[\x94\x93PPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x04.W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\x12V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04cW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\x83WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x04\xD3W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x04\xB0WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04\xCFW\x82\x81U`\x01\x01a\x04\xBCV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xF2Wa\x04\xF2a\x01\xDEV[a\x05\x06\x81a\x05\0\x84Ta\x04OV[\x84a\x04\x89V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x05;W`\0\x84\x15a\x05#WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x04\xCFV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x05jW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x05KV[P\x85\x82\x10\x15a\x05\x88W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x90\x1F\xAF(-\xBA>\x92\x07\0\xF6\xF7\xA6i\x84\xCCl\x13\xAFE?\xD0\xEF7\x13\x1E\xC0e~Jg\x0BdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static WRITER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Writer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Writer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Writer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Writer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Writer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Writer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Writer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                WRITER_ABI.clone(),
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
                WRITER_ABI.clone(),
                WRITER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `echoString` (0x0d7e2fce) function
        pub fn echo_string(
            &self,
            test_string: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([13, 126, 47, 206], test_string)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `WasWritten` event
        pub fn was_written_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WasWrittenFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WasWrittenFilter> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Writer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "WasWritten", abi = "WasWritten(string)")]
    pub struct WasWrittenFilter {
        pub test_string: ::std::string::String,
    }
    ///Container type for all input parameters for the `echoString` function with signature `echoString(string)` and selector `0x0d7e2fce`
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
    #[ethcall(name = "echoString", abi = "echoString(string)")]
    pub struct EchoStringCall {
        pub test_string: ::std::string::String,
    }
    ///Container type for all return fields from the `echoString` function with signature `echoString(string)` and selector `0x0d7e2fce`
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
    pub struct EchoStringReturn(pub ::std::string::String);
}
