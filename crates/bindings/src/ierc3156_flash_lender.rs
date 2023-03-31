pub use ierc3156_flash_lender::*;
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
pub mod ierc3156_flash_lender {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"flashFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC3156FlashBorrower\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"flashLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxFlashLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC3156FLASHLENDER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IERC3156FlashLender<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC3156FlashLender<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC3156FlashLender<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC3156FlashLender<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC3156FlashLender<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC3156FlashLender))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC3156FlashLender<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IERC3156FLASHLENDER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `flashFee` (0xd9d98ce4) function
        pub fn flash_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([217, 217, 140, 228], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoan` (0x5cffe9de) function
        pub fn flash_loan(
            &self,
            receiver: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 255, 233, 222], (receiver, token, amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxFlashLoan` (0x613255ab) function
        pub fn max_flash_loan(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 50, 85, 171], token)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IERC3156FlashLender<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `flashFee` function with signature `flashFee(address,uint256)` and selector `0xd9d98ce4`
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
    #[ethcall(name = "flashFee", abi = "flashFee(address,uint256)")]
    pub struct FlashFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address,address,uint256,bytes)` and selector `0x5cffe9de`
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
    #[ethcall(name = "flashLoan", abi = "flashLoan(address,address,uint256,bytes)")]
    pub struct FlashLoanCall {
        pub receiver: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `maxFlashLoan` function with signature `maxFlashLoan(address)` and selector `0x613255ab`
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
    #[ethcall(name = "maxFlashLoan", abi = "maxFlashLoan(address)")]
    pub struct MaxFlashLoanCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC3156FlashLenderCalls {
        FlashFee(FlashFeeCall),
        FlashLoan(FlashLoanCall),
        MaxFlashLoan(MaxFlashLoanCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC3156FlashLenderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FlashFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FlashFee(decoded));
            }
            if let Ok(decoded) = <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FlashLoan(decoded));
            }
            if let Ok(decoded) = <MaxFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxFlashLoan(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC3156FlashLenderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FlashFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FlashLoan(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxFlashLoan(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IERC3156FlashLenderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FlashFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FlashFeeCall> for IERC3156FlashLenderCalls {
        fn from(value: FlashFeeCall) -> Self {
            Self::FlashFee(value)
        }
    }
    impl ::core::convert::From<FlashLoanCall> for IERC3156FlashLenderCalls {
        fn from(value: FlashLoanCall) -> Self {
            Self::FlashLoan(value)
        }
    }
    impl ::core::convert::From<MaxFlashLoanCall> for IERC3156FlashLenderCalls {
        fn from(value: MaxFlashLoanCall) -> Self {
            Self::MaxFlashLoan(value)
        }
    }
    ///Container type for all return fields from the `flashFee` function with signature `flashFee(address,uint256)` and selector `0xd9d98ce4`
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
    pub struct FlashFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `flashLoan` function with signature `flashLoan(address,address,uint256,bytes)` and selector `0x5cffe9de`
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
    pub struct FlashLoanReturn(pub bool);
    ///Container type for all return fields from the `maxFlashLoan` function with signature `maxFlashLoan(address)` and selector `0x613255ab`
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
    pub struct MaxFlashLoanReturn(pub ::ethers::core::types::U256);
}
