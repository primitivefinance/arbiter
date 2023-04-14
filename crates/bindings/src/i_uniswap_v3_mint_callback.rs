pub use i_uniswap_v3_mint_callback::*;
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
pub mod i_uniswap_v3_mint_callback {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0Owed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1Owed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"uniswapV3MintCallback\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUNISWAPV3MINTCALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IUniswapV3MintCallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniswapV3MintCallback<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniswapV3MintCallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniswapV3MintCallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniswapV3MintCallback<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3MintCallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3MintCallback<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IUNISWAPV3MINTCALLBACK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `uniswapV3MintCallback` (0xd3487997) function
        pub fn uniswap_v3_mint_callback(
            &self,
            amount_0_owed: ::ethers::core::types::U256,
            amount_1_owed: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 72, 121, 151], (amount_0_owed, amount_1_owed, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IUniswapV3MintCallback<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `uniswapV3MintCallback` function with signature `uniswapV3MintCallback(uint256,uint256,bytes)` and selector `0xd3487997`
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
        name = "uniswapV3MintCallback",
        abi = "uniswapV3MintCallback(uint256,uint256,bytes)"
    )]
    pub struct UniswapV3MintCallbackCall {
        pub amount_0_owed: ::ethers::core::types::U256,
        pub amount_1_owed: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
}
