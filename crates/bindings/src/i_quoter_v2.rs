pub use i_quoter_v2::*;
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
pub mod i_quoter_v2 {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160[]\",\"name\":\"sqrtPriceX96AfterList\",\"type\":\"uint160[]\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"initializedTicksCrossedList\",\"type\":\"uint32[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasEstimate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IQuoterV2.QuoteExactInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96After\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"initializedTicksCrossed\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasEstimate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactOutput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160[]\",\"name\":\"sqrtPriceX96AfterList\",\"type\":\"uint160[]\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"initializedTicksCrossedList\",\"type\":\"uint32[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasEstimate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IQuoterV2.QuoteExactOutputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactOutputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96After\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"initializedTicksCrossed\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasEstimate\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IQUOTERV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IQuoterV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IQuoterV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IQuoterV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IQuoterV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IQuoterV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IQuoterV2)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IQuoterV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IQUOTERV2_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `quoteExactInput` (0xcdca1753) function
        pub fn quote_exact_input(
            &self,
            path: ::ethers::core::types::Bytes,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::std::vec::Vec<u32>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([205, 202, 23, 83], (path, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactInputSingle` (0xc6a5026a) function
        pub fn quote_exact_input_single(
            &self,
            params: QuoteExactInputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u32,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([198, 165, 2, 106], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactOutput` (0x2f80bb1d) function
        pub fn quote_exact_output(
            &self,
            path: ::ethers::core::types::Bytes,
            amount_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::std::vec::Vec<u32>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([47, 128, 187, 29], (path, amount_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactOutputSingle` (0xbd21704a) function
        pub fn quote_exact_output_single(
            &self,
            params: QuoteExactOutputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u32,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([189, 33, 112, 74], (params,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IQuoterV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `quoteExactInput` function with signature `quoteExactInput(bytes,uint256)` and selector `0xcdca1753`
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
    #[ethcall(name = "quoteExactInput", abi = "quoteExactInput(bytes,uint256)")]
    pub struct QuoteExactInputCall {
        pub path: ::ethers::core::types::Bytes,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quoteExactInputSingle` function with signature `quoteExactInputSingle((address,address,uint256,uint24,uint160))` and selector `0xc6a5026a`
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
    #[ethcall(
        name = "quoteExactInputSingle",
        abi = "quoteExactInputSingle((address,address,uint256,uint24,uint160))"
    )]
    pub struct QuoteExactInputSingleCall {
        pub params: QuoteExactInputSingleParams,
    }
    ///Container type for all input parameters for the `quoteExactOutput` function with signature `quoteExactOutput(bytes,uint256)` and selector `0x2f80bb1d`
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
    #[ethcall(name = "quoteExactOutput", abi = "quoteExactOutput(bytes,uint256)")]
    pub struct QuoteExactOutputCall {
        pub path: ::ethers::core::types::Bytes,
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle((address,address,uint256,uint24,uint160))` and selector `0xbd21704a`
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
    #[ethcall(
        name = "quoteExactOutputSingle",
        abi = "quoteExactOutputSingle((address,address,uint256,uint24,uint160))"
    )]
    pub struct QuoteExactOutputSingleCall {
        pub params: QuoteExactOutputSingleParams,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IQuoterV2Calls {
        QuoteExactInput(QuoteExactInputCall),
        QuoteExactInputSingle(QuoteExactInputSingleCall),
        QuoteExactOutput(QuoteExactOutputCall),
        QuoteExactOutputSingle(QuoteExactOutputSingleCall),
    }
    impl ::ethers::core::abi::AbiDecode for IQuoterV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <QuoteExactInputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuoteExactInput(decoded));
            }
            if let Ok(decoded)
                = <QuoteExactInputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::QuoteExactInputSingle(decoded));
            }
            if let Ok(decoded)
                = <QuoteExactOutputCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::QuoteExactOutput(decoded));
            }
            if let Ok(decoded)
                = <QuoteExactOutputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::QuoteExactOutputSingle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IQuoterV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::QuoteExactInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteExactInputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteExactOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteExactOutputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IQuoterV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::QuoteExactInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteExactInputSingle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteExactOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteExactOutputSingle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<QuoteExactInputCall> for IQuoterV2Calls {
        fn from(value: QuoteExactInputCall) -> Self {
            Self::QuoteExactInput(value)
        }
    }
    impl ::core::convert::From<QuoteExactInputSingleCall> for IQuoterV2Calls {
        fn from(value: QuoteExactInputSingleCall) -> Self {
            Self::QuoteExactInputSingle(value)
        }
    }
    impl ::core::convert::From<QuoteExactOutputCall> for IQuoterV2Calls {
        fn from(value: QuoteExactOutputCall) -> Self {
            Self::QuoteExactOutput(value)
        }
    }
    impl ::core::convert::From<QuoteExactOutputSingleCall> for IQuoterV2Calls {
        fn from(value: QuoteExactOutputSingleCall) -> Self {
            Self::QuoteExactOutputSingle(value)
        }
    }
    ///Container type for all return fields from the `quoteExactInput` function with signature `quoteExactInput(bytes,uint256)` and selector `0xcdca1753`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuoteExactInputReturn {
        pub amount_out: ::ethers::core::types::U256,
        pub sqrt_price_x96_after_list: ::std::vec::Vec<::ethers::core::types::U256>,
        pub initialized_ticks_crossed_list: ::std::vec::Vec<u32>,
        pub gas_estimate: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteExactInputSingle` function with signature `quoteExactInputSingle((address,address,uint256,uint24,uint160))` and selector `0xc6a5026a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuoteExactInputSingleReturn {
        pub amount_out: ::ethers::core::types::U256,
        pub sqrt_price_x96_after: ::ethers::core::types::U256,
        pub initialized_ticks_crossed: u32,
        pub gas_estimate: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteExactOutput` function with signature `quoteExactOutput(bytes,uint256)` and selector `0x2f80bb1d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuoteExactOutputReturn {
        pub amount_in: ::ethers::core::types::U256,
        pub sqrt_price_x96_after_list: ::std::vec::Vec<::ethers::core::types::U256>,
        pub initialized_ticks_crossed_list: ::std::vec::Vec<u32>,
        pub gas_estimate: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle((address,address,uint256,uint24,uint160))` and selector `0xbd21704a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuoteExactOutputSingleReturn {
        pub amount_in: ::ethers::core::types::U256,
        pub sqrt_price_x96_after: ::ethers::core::types::U256,
        pub initialized_ticks_crossed: u32,
        pub gas_estimate: ::ethers::core::types::U256,
    }
}
