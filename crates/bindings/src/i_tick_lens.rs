pub use i_tick_lens::*;
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
pub mod i_tick_lens {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int16\",\"name\":\"tickBitmapIndex\",\"type\":\"int16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPopulatedTicksInWord\",\"outputs\":[{\"internalType\":\"struct ITickLens.PopulatedTick[]\",\"name\":\"populatedTicks\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"liquidityNet\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"liquidityGross\",\"type\":\"uint128\",\"components\":[]}]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ITICKLENS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ITickLens<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ITickLens<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ITickLens<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ITickLens<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ITickLens<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ITickLens))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ITickLens<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ITICKLENS_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getPopulatedTicksInWord` (0x351fb478) function
        pub fn get_populated_ticks_in_word(
            &self,
            pool: ::ethers::core::types::Address,
            tick_bitmap_index: i16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PopulatedTick>> {
            self.0
                .method_hash([53, 31, 180, 120], (pool, tick_bitmap_index))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ITickLens<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getPopulatedTicksInWord` function with signature `getPopulatedTicksInWord(address,int16)` and selector `0x351fb478`
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
        name = "getPopulatedTicksInWord",
        abi = "getPopulatedTicksInWord(address,int16)"
    )]
    pub struct GetPopulatedTicksInWordCall {
        pub pool: ::ethers::core::types::Address,
        pub tick_bitmap_index: i16,
    }
    ///Container type for all return fields from the `getPopulatedTicksInWord` function with signature `getPopulatedTicksInWord(address,int16)` and selector `0x351fb478`
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
    pub struct GetPopulatedTicksInWordReturn {
        pub populated_ticks: ::std::vec::Vec<PopulatedTick>,
    }
}
