pub use i_swap_router::*;
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
pub mod i_swap_router {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactInputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactOutputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactOutput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactOutputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactOutputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0Delta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1Delta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"uniswapV3SwapCallback\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ISWAPROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ISwapRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISwapRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISwapRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISwapRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISwapRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ISwapRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISwapRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISWAPROUTER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `exactInput` (0xc04b8d59) function
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 75, 141, 89], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInputSingle` (0x414bf389) function
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 75, 243, 137], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactOutput` (0xf28c0498) function
        pub fn exact_output(
            &self,
            params: ExactOutputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 140, 4, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactOutputSingle` (0xdb3e2198) function
        pub fn exact_output_single(
            &self,
            params: ExactOutputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 62, 33, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ISwapRouter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `0xc04b8d59`
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
        name = "exactInput",
        abi = "exactInput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    ///Container type for all input parameters for the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0x414bf389`
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
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }
    ///Container type for all input parameters for the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `0xf28c0498`
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
        name = "exactOutput",
        abi = "exactOutput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactOutputCall {
        pub params: ExactOutputParams,
    }
    ///Container type for all input parameters for the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0xdb3e2198`
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
        name = "exactOutputSingle",
        abi = "exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactOutputSingleCall {
        pub params: ExactOutputSingleParams,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISwapRouterCalls {
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        ExactOutput(ExactOutputCall),
        ExactOutputSingle(ExactOutputSingleCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISwapRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExactInputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExactInput(decoded));
            }
            if let Ok(decoded) =
                <ExactInputSingleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExactInputSingle(decoded));
            }
            if let Ok(decoded) = <ExactOutputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExactOutput(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputSingleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExactOutputSingle(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISwapRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExactInput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactInputSingle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactOutput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactOutputSingle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ISwapRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExactInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInputSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactOutputSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExactInputCall> for ISwapRouterCalls {
        fn from(value: ExactInputCall) -> Self {
            Self::ExactInput(value)
        }
    }
    impl ::core::convert::From<ExactInputSingleCall> for ISwapRouterCalls {
        fn from(value: ExactInputSingleCall) -> Self {
            Self::ExactInputSingle(value)
        }
    }
    impl ::core::convert::From<ExactOutputCall> for ISwapRouterCalls {
        fn from(value: ExactOutputCall) -> Self {
            Self::ExactOutput(value)
        }
    }
    impl ::core::convert::From<ExactOutputSingleCall> for ISwapRouterCalls {
        fn from(value: ExactOutputSingleCall) -> Self {
            Self::ExactOutputSingle(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for ISwapRouterCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `0xc04b8d59`
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
    pub struct ExactInputReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0x414bf389`
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
    pub struct ExactInputSingleReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `0xf28c0498`
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
    pub struct ExactOutputReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0xdb3e2198`
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
    pub struct ExactOutputSingleReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
}
