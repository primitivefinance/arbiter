pub use nonfungible_token_position_descriptor::*;
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
pub mod nonfungible_token_position_descriptor {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_WETH9\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_nativeCurrencyLabelBytes\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH9\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"chainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"flipRatio\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nativeCurrencyLabel\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nativeCurrencyLabelBytes\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"chainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenRatioPriority\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract INonfungiblePositionManager\",\"name\":\"positionManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static NONFUNGIBLETOKENPOSITIONDESCRIPTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct NonfungibleTokenPositionDescriptor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NonfungibleTokenPositionDescriptor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NonfungibleTokenPositionDescriptor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NonfungibleTokenPositionDescriptor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NonfungibleTokenPositionDescriptor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(NonfungibleTokenPositionDescriptor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NonfungibleTokenPositionDescriptor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                NONFUNGIBLETOKENPOSITIONDESCRIPTOR_ABI.clone(),
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
        ///Calls the contract's `flipRatio` (0x7e5af771) function
        pub fn flip_ratio(
            &self,
            token_0: ::ethers::core::types::Address,
            token_1: ::ethers::core::types::Address,
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([126, 90, 247, 113], (token_0, token_1, chain_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nativeCurrencyLabel` (0xb7af3cdc) function
        pub fn native_currency_label(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([183, 175, 60, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nativeCurrencyLabelBytes` (0xa18246e2) function
        pub fn native_currency_label_bytes(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([161, 130, 70, 226], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenRatioPriority` (0x9d7b0ea8) function
        pub fn token_ratio_priority(
            &self,
            token: ::ethers::core::types::Address,
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([157, 123, 14, 168], (token, chain_id))
                .expect("method not found (this should never happen)")
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
        for NonfungibleTokenPositionDescriptor<M>
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
    ///Container type for all input parameters for the `flipRatio` function with signature `flipRatio(address,address,uint256)` and selector `0x7e5af771`
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
    #[ethcall(name = "flipRatio", abi = "flipRatio(address,address,uint256)")]
    pub struct FlipRatioCall {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `nativeCurrencyLabel` function with signature `nativeCurrencyLabel()` and selector `0xb7af3cdc`
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
    #[ethcall(name = "nativeCurrencyLabel", abi = "nativeCurrencyLabel()")]
    pub struct NativeCurrencyLabelCall;
    ///Container type for all input parameters for the `nativeCurrencyLabelBytes` function with signature `nativeCurrencyLabelBytes()` and selector `0xa18246e2`
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
    #[ethcall(name = "nativeCurrencyLabelBytes", abi = "nativeCurrencyLabelBytes()")]
    pub struct NativeCurrencyLabelBytesCall;
    ///Container type for all input parameters for the `tokenRatioPriority` function with signature `tokenRatioPriority(address,uint256)` and selector `0x9d7b0ea8`
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
        name = "tokenRatioPriority",
        abi = "tokenRatioPriority(address,uint256)"
    )]
    pub struct TokenRatioPriorityCall {
        pub token: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum NonfungibleTokenPositionDescriptorCalls {
        Weth9(Weth9Call),
        FlipRatio(FlipRatioCall),
        NativeCurrencyLabel(NativeCurrencyLabelCall),
        NativeCurrencyLabelBytes(NativeCurrencyLabelBytesCall),
        TokenRatioPriority(TokenRatioPriorityCall),
        TokenURI(TokenURICall),
    }
    impl ::ethers::core::abi::AbiDecode for NonfungibleTokenPositionDescriptorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <FlipRatioCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FlipRatio(decoded));
            }
            if let Ok(decoded) =
                <NativeCurrencyLabelCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeCurrencyLabel(decoded));
            }
            if let Ok(decoded) =
                <NativeCurrencyLabelBytesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeCurrencyLabelBytes(decoded));
            }
            if let Ok(decoded) =
                <TokenRatioPriorityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenRatioPriority(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NonfungibleTokenPositionDescriptorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FlipRatio(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NativeCurrencyLabel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeCurrencyLabelBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenRatioPriority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for NonfungibleTokenPositionDescriptorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlipRatio(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeCurrencyLabel(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeCurrencyLabelBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenRatioPriority(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for NonfungibleTokenPositionDescriptorCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<FlipRatioCall> for NonfungibleTokenPositionDescriptorCalls {
        fn from(value: FlipRatioCall) -> Self {
            Self::FlipRatio(value)
        }
    }
    impl ::core::convert::From<NativeCurrencyLabelCall> for NonfungibleTokenPositionDescriptorCalls {
        fn from(value: NativeCurrencyLabelCall) -> Self {
            Self::NativeCurrencyLabel(value)
        }
    }
    impl ::core::convert::From<NativeCurrencyLabelBytesCall>
        for NonfungibleTokenPositionDescriptorCalls
    {
        fn from(value: NativeCurrencyLabelBytesCall) -> Self {
            Self::NativeCurrencyLabelBytes(value)
        }
    }
    impl ::core::convert::From<TokenRatioPriorityCall> for NonfungibleTokenPositionDescriptorCalls {
        fn from(value: TokenRatioPriorityCall) -> Self {
            Self::TokenRatioPriority(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for NonfungibleTokenPositionDescriptorCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
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
    ///Container type for all return fields from the `flipRatio` function with signature `flipRatio(address,address,uint256)` and selector `0x7e5af771`
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
    pub struct FlipRatioReturn(pub bool);
    ///Container type for all return fields from the `nativeCurrencyLabel` function with signature `nativeCurrencyLabel()` and selector `0xb7af3cdc`
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
    pub struct NativeCurrencyLabelReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nativeCurrencyLabelBytes` function with signature `nativeCurrencyLabelBytes()` and selector `0xa18246e2`
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
    pub struct NativeCurrencyLabelBytesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `tokenRatioPriority` function with signature `tokenRatioPriority(address,uint256)` and selector `0x9d7b0ea8`
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
    pub struct TokenRatioPriorityReturn(pub ::ethers::core::types::I256);
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
