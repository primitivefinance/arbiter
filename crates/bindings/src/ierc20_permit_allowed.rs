pub use ierc20_permit_allowed::*;
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
pub mod ierc20_permit_allowed {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"holder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC20PERMITALLOWED_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IERC20PermitAllowed<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC20PermitAllowed<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC20PermitAllowed<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC20PermitAllowed<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC20PermitAllowed<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC20PermitAllowed))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC20PermitAllowed<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IERC20PERMITALLOWED_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `permit` (0x8fcbaf0c) function
        pub fn permit(
            &self,
            holder: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            allowed: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [143, 203, 175, 12],
                    (holder, spender, nonce, expiry, allowed, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IERC20PermitAllowed<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,bool,uint8,bytes32,bytes32)` and selector `0x8fcbaf0c`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub holder: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub allowed: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
}
