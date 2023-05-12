pub use position_key::*;
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
pub mod position_key {
    #[rustfmt::skip]
    const __ABI: &str = "[]";
    ///The parsed JSON ABI of the contract.
    pub static POSITIONKEY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        86,
        96,
        35,
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
        22,
        87,
        254,
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
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
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
        204,
        222,
        68,
        153,
        227,
        185,
        58,
        173,
        122,
        24,
        6,
        28,
        223,
        230,
        50,
        34,
        255,
        208,
        87,
        27,
        185,
        206,
        138,
        38,
        181,
        251,
        212,
        95,
        168,
        142,
        51,
        125,
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
    pub static POSITIONKEY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
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
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
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
        204,
        222,
        68,
        153,
        227,
        185,
        58,
        173,
        122,
        24,
        6,
        28,
        223,
        230,
        50,
        34,
        255,
        208,
        87,
        27,
        185,
        206,
        138,
        38,
        181,
        251,
        212,
        95,
        168,
        142,
        51,
        125,
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
    pub static POSITIONKEY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PositionKey<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PositionKey<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PositionKey<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PositionKey<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PositionKey<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PositionKey))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PositionKey<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                POSITIONKEY_ABI.clone(),
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
                POSITIONKEY_ABI.clone(),
                POSITIONKEY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PositionKey<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
