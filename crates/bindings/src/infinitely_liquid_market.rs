pub use infinitely_liquid_market::*;
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
pub mod infinitely_liquid_market {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceChange\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPrice\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static INFINITELYLIQUIDMARKET_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
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
        199,
        128,
        97,
        0,
        31,
        96,
        0,
        57,
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
        96,
        15,
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
        96,
        40,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        145,
        183,
        245,
        237,
        20,
        96,
        45,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        60,
        96,
        56,
        54,
        96,
        4,
        96,
        121,
        86,
        91,
        96,
        62,
        86,
        91,
        0,
        91,
        96,
        0,
        129,
        144,
        85,
        96,
        64,
        81,
        129,
        129,
        82,
        127,
        243,
        71,
        238,
        153,
        80,
        59,
        241,
        156,
        2,
        139,
        214,
        177,
        143,
        60,
        103,
        110,
        130,
        169,
        187,
        91,
        43,
        181,
        34,
        90,
        235,
        224,
        253,
        98,
        253,
        106,
        13,
        25,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        138,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
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
        208,
        196,
        198,
        230,
        162,
        224,
        215,
        233,
        234,
        89,
        1,
        51,
        159,
        101,
        143,
        190,
        227,
        90,
        238,
        199,
        91,
        223,
        191,
        174,
        111,
        48,
        228,
        246,
        22,
        219,
        187,
        243,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static INFINITELYLIQUIDMARKET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
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
        96,
        15,
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
        96,
        40,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        145,
        183,
        245,
        237,
        20,
        96,
        45,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        60,
        96,
        56,
        54,
        96,
        4,
        96,
        121,
        86,
        91,
        96,
        62,
        86,
        91,
        0,
        91,
        96,
        0,
        129,
        144,
        85,
        96,
        64,
        81,
        129,
        129,
        82,
        127,
        243,
        71,
        238,
        153,
        80,
        59,
        241,
        156,
        2,
        139,
        214,
        177,
        143,
        60,
        103,
        110,
        130,
        169,
        187,
        91,
        43,
        181,
        34,
        90,
        235,
        224,
        253,
        98,
        253,
        106,
        13,
        25,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        138,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
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
        208,
        196,
        198,
        230,
        162,
        224,
        215,
        233,
        234,
        89,
        1,
        51,
        159,
        101,
        143,
        190,
        227,
        90,
        238,
        199,
        91,
        223,
        191,
        174,
        111,
        48,
        228,
        246,
        22,
        219,
        187,
        243,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static INFINITELYLIQUIDMARKET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InfinitelyLiquidMarket<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InfinitelyLiquidMarket<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InfinitelyLiquidMarket<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InfinitelyLiquidMarket<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InfinitelyLiquidMarket<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(InfinitelyLiquidMarket))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InfinitelyLiquidMarket<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INFINITELYLIQUIDMARKET_ABI.clone(),
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
                INFINITELYLIQUIDMARKET_ABI.clone(),
                INFINITELYLIQUIDMARKET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `setPrice` (0x91b7f5ed) function
        pub fn set_price(
            &self,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 183, 245, 237], price)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PriceChange` event
        pub fn price_change_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceChangeFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceChangeFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InfinitelyLiquidMarket<M> {
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
        Hash
    )]
    #[ethevent(name = "PriceChange", abi = "PriceChange(uint256)")]
    pub struct PriceChangeFilter {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPrice` function with signature `setPrice(uint256)` and selector `0x91b7f5ed`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setPrice", abi = "setPrice(uint256)")]
    pub struct SetPriceCall {
        pub price: ::ethers::core::types::U256,
    }
}
