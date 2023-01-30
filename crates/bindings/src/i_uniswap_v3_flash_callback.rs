pub use i_uniswap_v3_flash_callback::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_uniswap_v3_flash_callback {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///IUniswapV3FlashCallback was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    use ::ethers::core::{
        abi::{Abi, Token, Detokenize, InvalidOutputType, Tokenizable},
        types::*,
    };
    use ::ethers::contract::{
        Contract, builders::{ContractCall, Event},
        Lazy,
    };
    use ::ethers::providers::Middleware;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"uniswapV3FlashCallback\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static IUNISWAPV3FLASHCALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
    pub struct IUniswapV3FlashCallback<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV3FlashCallback<M> {
        fn clone(&self) -> Self {
            IUniswapV3FlashCallback(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV3FlashCallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV3FlashCallback<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3FlashCallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3FlashCallback<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IUNISWAPV3FLASHCALLBACK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `uniswapV3FlashCallback` (0xe9cbafb0) function
        pub fn uniswap_v3_flash_callback(
            &self,
            fee_0: ::ethers::core::types::U256,
            fee_1: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 203, 175, 176], (fee_0, fee_1, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IUniswapV3FlashCallback<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `uniswapV3FlashCallback` function with signature `uniswapV3FlashCallback(uint256,uint256,bytes)` and selector `0xe9cbafb0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "uniswapV3FlashCallback",
        abi = "uniswapV3FlashCallback(uint256,uint256,bytes)"
    )]
    pub struct UniswapV3FlashCallbackCall {
        pub fee_0: ::ethers::core::types::U256,
        pub fee_1: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
}
