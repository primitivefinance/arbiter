pub use periphery_payments_with_fee::*;
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
pub mod periphery_payments_with_fee {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH9\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"refundETH\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sweepToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sweepTokenWithFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"unwrapWETH9\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"unwrapWETH9WithFee\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PERIPHERYPAYMENTSWITHFEE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct PeripheryPaymentsWithFee<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PeripheryPaymentsWithFee<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PeripheryPaymentsWithFee<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PeripheryPaymentsWithFee<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PeripheryPaymentsWithFee<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PeripheryPaymentsWithFee))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PeripheryPaymentsWithFee<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PERIPHERYPAYMENTSWITHFEE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundETH` (0x12210e8a) function
        pub fn refund_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepToken` (0xdf2ab5bb) function
        pub fn sweep_token(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepTokenWithFee` (0xe0e189a0) function
        pub fn sweep_token_with_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 225, 137, 160],
                    (token, amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9` (0x49404b7c) function
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9WithFee` (0x9b2c0a37) function
        pub fn unwrap_weth9_with_fee(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 44, 10, 55],
                    (amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PeripheryPaymentsWithFee<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `refundETH` function with signature `refundETH()` and selector `0x12210e8a`
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
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;
    ///Container type for all input parameters for the `sweepToken` function with signature `sweepToken(address,uint256,address)` and selector `0xdf2ab5bb`
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
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sweepTokenWithFee` function with signature `sweepTokenWithFee(address,uint256,address,uint256,address)` and selector `0xe0e189a0`
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
        name = "sweepTokenWithFee",
        abi = "sweepTokenWithFee(address,uint256,address,uint256,address)"
    )]
    pub struct SweepTokenWithFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapWETH9` function with signature `unwrapWETH9(uint256,address)` and selector `0x49404b7c`
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
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256,address)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapWETH9WithFee` function with signature `unwrapWETH9WithFee(uint256,address,uint256,address)` and selector `0x9b2c0a37`
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
        name = "unwrapWETH9WithFee",
        abi = "unwrapWETH9WithFee(uint256,address,uint256,address)"
    )]
    pub struct UnwrapWETH9WithFeeCall {
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PeripheryPaymentsWithFeeCalls {
        Weth9(Weth9Call),
        Factory(FactoryCall),
        RefundETH(RefundETHCall),
        SweepToken(SweepTokenCall),
        SweepTokenWithFee(SweepTokenWithFeeCall),
        UnwrapWETH9(UnwrapWETH9Call),
        UnwrapWETH9WithFee(UnwrapWETH9WithFeeCall),
    }
    impl ::ethers::core::abi::AbiDecode for PeripheryPaymentsWithFeeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <RefundETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RefundETH(decoded));
            }
            if let Ok(decoded) = <SweepTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SweepToken(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenWithFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SweepTokenWithFee(decoded));
            }
            if let Ok(decoded) = <UnwrapWETH9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnwrapWETH9(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWETH9WithFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnwrapWETH9WithFee(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PeripheryPaymentsWithFeeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SweepToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SweepTokenWithFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnwrapWETH9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnwrapWETH9WithFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PeripheryPaymentsWithFeeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepTokenWithFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH9(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH9WithFee(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for PeripheryPaymentsWithFeeCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for PeripheryPaymentsWithFeeCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<RefundETHCall> for PeripheryPaymentsWithFeeCalls {
        fn from(value: RefundETHCall) -> Self {
            Self::RefundETH(value)
        }
    }
    impl ::core::convert::From<SweepTokenCall> for PeripheryPaymentsWithFeeCalls {
        fn from(value: SweepTokenCall) -> Self {
            Self::SweepToken(value)
        }
    }
    impl ::core::convert::From<SweepTokenWithFeeCall> for PeripheryPaymentsWithFeeCalls {
        fn from(value: SweepTokenWithFeeCall) -> Self {
            Self::SweepTokenWithFee(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9Call> for PeripheryPaymentsWithFeeCalls {
        fn from(value: UnwrapWETH9Call) -> Self {
            Self::UnwrapWETH9(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9WithFeeCall> for PeripheryPaymentsWithFeeCalls {
        fn from(value: UnwrapWETH9WithFeeCall) -> Self {
            Self::UnwrapWETH9WithFee(value)
        }
    }
    ///Container type for all return fields from the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
    pub struct Weth9Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
}
