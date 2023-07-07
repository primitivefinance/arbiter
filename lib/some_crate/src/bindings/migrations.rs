pub use migrations::*;
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
pub mod migrations {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("lastCompletedMigration"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lastCompletedMigration",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(true),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(true),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCompleted"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setCompleted"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("completed"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::Some(false),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgrade"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upgrade"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newAddress"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::Some(false),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MIGRATIONS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x01\xB3\x80a\x002`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\t\0\xF0\x10\x14a\0QW\x80c\x8D\xA5\xCB[\x14a\0yW\x80c\xFB\xDB\xAD<\x14a\0\x9DW\x80c\xFD\xAC\xD5v\x14a\0\xB7W[`\0\x80\xFD[a\0w`\x04\x806\x03` \x81\x10\x15a\0gW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\0\xD4V[\0[a\0\x81a\x01QV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xA5a\x01`V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0w`\x04\x806\x03` \x81\x10\x15a\0\xCDW`\0\x80\xFD[P5a\x01fV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15a\x01NW`\0\x81\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xAC\xD5v`\x01T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x014W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01HW=`\0\x80>=`\0\xFD[PPPPP[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15a\x01NW`\x01UV\xFE\xA2ebzzr1X B\x1E\xAFU\x19\xF5*\x9D\xAA$\xA9\x94\x04\x15\xE4\xF9\xD9{<\xCB\"\x8D\x7F\x06t\xB9\xF9\x15N\xF8\xFC9dsolcC\0\x05\x10\x002";
    /// The bytecode of the contract.
    pub static MIGRATIONS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\t\0\xF0\x10\x14a\0QW\x80c\x8D\xA5\xCB[\x14a\0yW\x80c\xFB\xDB\xAD<\x14a\0\x9DW\x80c\xFD\xAC\xD5v\x14a\0\xB7W[`\0\x80\xFD[a\0w`\x04\x806\x03` \x81\x10\x15a\0gW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\0\xD4V[\0[a\0\x81a\x01QV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xA5a\x01`V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0w`\x04\x806\x03` \x81\x10\x15a\0\xCDW`\0\x80\xFD[P5a\x01fV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15a\x01NW`\0\x81\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xAC\xD5v`\x01T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x014W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01HW=`\0\x80>=`\0\xFD[PPPPP[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15a\x01NW`\x01UV\xFE\xA2ebzzr1X B\x1E\xAFU\x19\xF5*\x9D\xAA$\xA9\x94\x04\x15\xE4\xF9\xD9{<\xCB\"\x8D\x7F\x06t\xB9\xF9\x15N\xF8\xFC9dsolcC\0\x05\x10\x002";
    /// The deployed bytecode of the contract.
    pub static MIGRATIONS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Migrations<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Migrations<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Migrations<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Migrations<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Migrations<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Migrations))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Migrations<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MIGRATIONS_ABI.clone(),
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
                MIGRATIONS_ABI.clone(),
                MIGRATIONS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `lastCompletedMigration` (0xfbdbad3c) function
        pub fn last_completed_migration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([251, 219, 173, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCompleted` (0xfdacd576) function
        pub fn set_completed(
            &self,
            completed: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 172, 213, 118], completed)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrade` (0x0900f010) function
        pub fn upgrade(
            &self,
            new_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 0, 240, 16], new_address)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Migrations<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `lastCompletedMigration` function with signature `lastCompletedMigration()` and selector `0xfbdbad3c`
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
    #[ethcall(name = "lastCompletedMigration", abi = "lastCompletedMigration()")]
    pub struct LastCompletedMigrationCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `setCompleted` function with signature `setCompleted(uint256)` and selector `0xfdacd576`
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
    #[ethcall(name = "setCompleted", abi = "setCompleted(uint256)")]
    pub struct SetCompletedCall {
        pub completed: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(address)` and selector `0x0900f010`
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
    #[ethcall(name = "upgrade", abi = "upgrade(address)")]
    pub struct UpgradeCall {
        pub new_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MigrationsCalls {
        LastCompletedMigration(LastCompletedMigrationCall),
        Owner(OwnerCall),
        SetCompleted(SetCompletedCall),
        Upgrade(UpgradeCall),
    }
    impl ::ethers::core::abi::AbiDecode for MigrationsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <LastCompletedMigrationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastCompletedMigration(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <SetCompletedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetCompleted(decoded));
            }
            if let Ok(decoded) = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Upgrade(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MigrationsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LastCompletedMigration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCompleted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MigrationsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LastCompletedMigration(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCompleted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LastCompletedMigrationCall> for MigrationsCalls {
        fn from(value: LastCompletedMigrationCall) -> Self {
            Self::LastCompletedMigration(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MigrationsCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetCompletedCall> for MigrationsCalls {
        fn from(value: SetCompletedCall) -> Self {
            Self::SetCompleted(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for MigrationsCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    ///Container type for all return fields from the `lastCompletedMigration` function with signature `lastCompletedMigration()` and selector `0xfbdbad3c`
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
    pub struct LastCompletedMigrationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
