pub use i_nonfungible_token_position_descriptor::*;
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
pub mod i_nonfungible_token_position_descriptor {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract INonfungiblePositionManager\",\"name\":\"positionManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static INONFUNGIBLETOKENPOSITIONDESCRIPTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct INonfungibleTokenPositionDescriptor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for INonfungibleTokenPositionDescriptor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for INonfungibleTokenPositionDescriptor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for INonfungibleTokenPositionDescriptor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for INonfungibleTokenPositionDescriptor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(INonfungibleTokenPositionDescriptor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> INonfungibleTokenPositionDescriptor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                INONFUNGIBLETOKENPOSITIONDESCRIPTOR_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `tokenURI` (0xe9dc6375) function
        pub fn token_uri(
            &self,
            position_manager: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([233, 220, 99, 117], (position_manager, token_id))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for INonfungibleTokenPositionDescriptor<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(address,uint256)` and selector `0xe9dc6375`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(address,uint256)")]
    pub struct TokenURICall {
        pub position_manager: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(address,uint256)` and selector `0xe9dc6375`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}
