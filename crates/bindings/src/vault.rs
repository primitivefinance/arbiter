pub use vault::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod vault {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///Vault was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;

    use ::ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::Middleware,
    };
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAuthorizer\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AuthorizerChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExternalBalanceTransfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FlashLoan\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"delta\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InternalBalanceChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PausedStateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"liquidityProvider\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"deltas\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"protocolFeeAmounts\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PoolBalanceChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"assetManager\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"cashDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"managedDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PoolBalanceManaged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"poolAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"specialization\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PoolRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RelayerApprovalChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Swap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokensDeregistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"assetManagers\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokensRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deregisterTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct Vault.ExitPoolRequest\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"a\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"b\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"c\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"d\",\"type\":\"bool\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exitPool\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"userData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"flashLoan\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"selector\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getActionId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAuthorizer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDomainSeparator\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInternalBalance\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPausedState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"pauseWindowEndTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bufferPeriodEndTime\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolTokenInfo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"managed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastChangeBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"assetManager\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolTokens\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastChangeBlock\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProtocolFeesCollector\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasApprovedRelayer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct Vault.JoinPoolRequest\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"a\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"b\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"c\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"d\",\"type\":\"bool\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"joinPool\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"specialization\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPool\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"assetManagers\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAuthorizer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAuthorizer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaused\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRelayerApproval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct Vault.SingleSwap\",\"name\":\"singleSwap\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"a\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"b\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"c\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"d\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"e\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"f\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct Vault.FundManagement\",\"name\":\"funds\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"a\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"b\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"c\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"d\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"limit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountCalculated\",\"type\":\"uint256\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static VAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Vault<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for Vault<M> {
        fn clone(&self) -> Self {
            Vault(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Vault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Vault<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Vault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Vault<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VAULT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterTokens` (0x7d3aeb96) function
        pub fn deregister_tokens(
            &self,
            pool_id: [u8; 32],
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 58, 235, 150], (pool_id, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exitPool` (0x8bdb3913) function
        pub fn exit_pool(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            request: ExitPoolRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 219, 57, 19], (pool_id, sender, recipient, request))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoan` (0x5c38449e) function
        pub fn flash_loan(
            &self,
            recipient: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 56, 68, 158], (recipient, tokens, amounts, user_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionId` (0x851c1bb3) function
        pub fn get_action_id(
            &self,
            selector: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([133, 28, 27, 179], selector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAuthorizer` (0xaaabadc5) function
        pub fn get_authorizer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([170, 171, 173, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDomainSeparator` (0xed24911d) function
        pub fn get_domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([237, 36, 145, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInternalBalance` (0x0f5a6efa) function
        pub fn get_internal_balance(
            &self,
            user: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([15, 90, 110, 250], (user, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextNonce` (0x90193b7c) function
        pub fn get_next_nonce(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 25, 59, 124], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPausedState` (0x1c0de051) function
        pub fn get_paused_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([28, 13, 224, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0xf6c00927) function
        pub fn get_pool(
            &self,
            pool_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::Address, u8)>
        {
            self.0
                .method_hash([246, 192, 9, 39], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolTokenInfo` (0xb05f8e48) function
        pub fn get_pool_token_info(
            &self,
            pool_id: [u8; 32],
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([176, 95, 142, 72], (pool_id, token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolTokens` (0xf94d4668) function
        pub fn get_pool_tokens(
            &self,
            pool_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([249, 77, 70, 104], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProtocolFeesCollector` (0xd2946c2b) function
        pub fn get_protocol_fees_collector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([210, 148, 108, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasApprovedRelayer` (0xfec90d72) function
        pub fn has_approved_relayer(
            &self,
            user: ::ethers::core::types::Address,
            relayer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([254, 201, 13, 114], (user, relayer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `joinPool` (0xb95cac28) function
        pub fn join_pool(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            request: JoinPoolRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 92, 172, 40], (pool_id, sender, recipient, request))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerPool` (0x09b2760f) function
        pub fn register_pool(
            &self,
            specialization: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([9, 178, 118, 15], specialization)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerTokens` (0x66a9c7d2) function
        pub fn register_tokens(
            &self,
            pool_id: [u8; 32],
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            asset_managers: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 169, 199, 210], (pool_id, tokens, asset_managers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuthorizer` (0x058a628f) function
        pub fn set_authorizer(
            &self,
            new_authorizer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 138, 98, 143], new_authorizer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPaused` (0x16c38b3c) function
        pub fn set_paused(
            &self,
            paused: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 195, 139, 60], paused)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRelayerApproval` (0xfa6e671d) function
        pub fn set_relayer_approval(
            &self,
            sender: ::ethers::core::types::Address,
            relayer: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 110, 103, 29], (sender, relayer, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x52bbbe29) function
        pub fn swap(
            &self,
            single_swap: SingleSwap,
            funds: FundManagement,
            limit: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([82, 187, 190, 41], (single_swap, funds, limit, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AuthorizerChanged` event
        pub fn authorizer_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, AuthorizerChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `ExternalBalanceTransfer` event
        pub fn external_balance_transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, ExternalBalanceTransferFilter> {
            self.0.event()
        }
        ///Gets the contract's `FlashLoan` event
        pub fn flash_loan_filter(&self) -> ::ethers::contract::builders::Event<M, FlashLoanFilter> {
            self.0.event()
        }
        ///Gets the contract's `InternalBalanceChanged` event
        pub fn internal_balance_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, InternalBalanceChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PausedStateChanged` event
        pub fn paused_state_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PausedStateChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PoolBalanceChanged` event
        pub fn pool_balance_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PoolBalanceChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PoolBalanceManaged` event
        pub fn pool_balance_managed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PoolBalanceManagedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PoolRegistered` event
        pub fn pool_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PoolRegisteredFilter> {
            self.0.event()
        }
        ///Gets the contract's `RelayerApprovalChanged` event
        pub fn relayer_approval_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, RelayerApprovalChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(&self) -> ::ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokensDeregistered` event
        pub fn tokens_deregistered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TokensDeregisteredFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokensRegistered` event
        pub fn tokens_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TokensRegisteredFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, VaultEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Vault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "AuthorizerChanged", abi = "AuthorizerChanged(address)")]
    pub struct AuthorizerChangedFilter {
        #[ethevent(indexed)]
        pub new_authorizer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExternalBalanceTransfer",
        abi = "ExternalBalanceTransfer(address,address,address,uint256)"
    )]
    pub struct ExternalBalanceTransferFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "FlashLoan", abi = "FlashLoan(address,address,uint256,uint256)")]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub fee_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "InternalBalanceChanged",
        abi = "InternalBalanceChanged(address,address,int256)"
    )]
    pub struct InternalBalanceChangedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub delta: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "PausedStateChanged", abi = "PausedStateChanged(bool)")]
    pub struct PausedStateChangedFilter {
        pub paused: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "PoolBalanceChanged",
        abi = "PoolBalanceChanged(bytes32,address,address[],int256[],uint256[])"
    )]
    pub struct PoolBalanceChangedFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub liquidity_provider: ::ethers::core::types::Address,
        pub tokens: Vec<::ethers::core::types::Address>,
        pub deltas: Vec<::ethers::core::types::I256>,
        pub protocol_fee_amounts: Vec<::ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "PoolBalanceManaged",
        abi = "PoolBalanceManaged(bytes32,address,address,int256,int256)"
    )]
    pub struct PoolBalanceManagedFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub asset_manager: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub cash_delta: ::ethers::core::types::I256,
        pub managed_delta: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "PoolRegistered", abi = "PoolRegistered(bytes32,address,uint8)")]
    pub struct PoolRegisteredFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub pool_address: ::ethers::core::types::Address,
        pub specialization: u8,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "RelayerApprovalChanged",
        abi = "RelayerApprovalChanged(address,address,bool)"
    )]
    pub struct RelayerApprovalChangedFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Swap", abi = "Swap(bytes32,address,address,uint256,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TokensDeregistered",
        abi = "TokensDeregistered(bytes32,address[])"
    )]
    pub struct TokensDeregisteredFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        pub tokens: Vec<::ethers::core::types::Address>,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TokensRegistered",
        abi = "TokensRegistered(bytes32,address[],address[])"
    )]
    pub struct TokensRegisteredFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        pub tokens: Vec<::ethers::core::types::Address>,
        pub asset_managers: Vec<::ethers::core::types::Address>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum VaultEvents {
        AuthorizerChangedFilter(AuthorizerChangedFilter),
        ExternalBalanceTransferFilter(ExternalBalanceTransferFilter),
        FlashLoanFilter(FlashLoanFilter),
        InternalBalanceChangedFilter(InternalBalanceChangedFilter),
        PausedStateChangedFilter(PausedStateChangedFilter),
        PoolBalanceChangedFilter(PoolBalanceChangedFilter),
        PoolBalanceManagedFilter(PoolBalanceManagedFilter),
        PoolRegisteredFilter(PoolRegisteredFilter),
        RelayerApprovalChangedFilter(RelayerApprovalChangedFilter),
        SwapFilter(SwapFilter),
        TokensDeregisteredFilter(TokensDeregisteredFilter),
        TokensRegisteredFilter(TokensRegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for VaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AuthorizerChangedFilter::decode_log(log) {
                return Ok(VaultEvents::AuthorizerChangedFilter(decoded));
            }
            if let Ok(decoded) = ExternalBalanceTransferFilter::decode_log(log) {
                return Ok(VaultEvents::ExternalBalanceTransferFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(VaultEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = InternalBalanceChangedFilter::decode_log(log) {
                return Ok(VaultEvents::InternalBalanceChangedFilter(decoded));
            }
            if let Ok(decoded) = PausedStateChangedFilter::decode_log(log) {
                return Ok(VaultEvents::PausedStateChangedFilter(decoded));
            }
            if let Ok(decoded) = PoolBalanceChangedFilter::decode_log(log) {
                return Ok(VaultEvents::PoolBalanceChangedFilter(decoded));
            }
            if let Ok(decoded) = PoolBalanceManagedFilter::decode_log(log) {
                return Ok(VaultEvents::PoolBalanceManagedFilter(decoded));
            }
            if let Ok(decoded) = PoolRegisteredFilter::decode_log(log) {
                return Ok(VaultEvents::PoolRegisteredFilter(decoded));
            }
            if let Ok(decoded) = RelayerApprovalChangedFilter::decode_log(log) {
                return Ok(VaultEvents::RelayerApprovalChangedFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(VaultEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = TokensDeregisteredFilter::decode_log(log) {
                return Ok(VaultEvents::TokensDeregisteredFilter(decoded));
            }
            if let Ok(decoded) = TokensRegisteredFilter::decode_log(log) {
                return Ok(VaultEvents::TokensRegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for VaultEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VaultEvents::AuthorizerChangedFilter(element) => element.fmt(f),
                VaultEvents::ExternalBalanceTransferFilter(element) => element.fmt(f),
                VaultEvents::FlashLoanFilter(element) => element.fmt(f),
                VaultEvents::InternalBalanceChangedFilter(element) => element.fmt(f),
                VaultEvents::PausedStateChangedFilter(element) => element.fmt(f),
                VaultEvents::PoolBalanceChangedFilter(element) => element.fmt(f),
                VaultEvents::PoolBalanceManagedFilter(element) => element.fmt(f),
                VaultEvents::PoolRegisteredFilter(element) => element.fmt(f),
                VaultEvents::RelayerApprovalChangedFilter(element) => element.fmt(f),
                VaultEvents::SwapFilter(element) => element.fmt(f),
                VaultEvents::TokensDeregisteredFilter(element) => element.fmt(f),
                VaultEvents::TokensRegisteredFilter(element) => element.fmt(f),
            }
        }
    }
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `deregisterTokens` function with signature `deregisterTokens(bytes32,address[])` and selector `0x7d3aeb96`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "deregisterTokens", abi = "deregisterTokens(bytes32,address[])")]
    pub struct DeregisterTokensCall {
        pub pool_id: [u8; 32],
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `exitPool` function with signature `exitPool(bytes32,address,address,(address[],uint256[],bytes,bool))` and selector `0x8bdb3913`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "exitPool",
        abi = "exitPool(bytes32,address,address,(address[],uint256[],bytes,bool))"
    )]
    pub struct ExitPoolCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub request: ExitPoolRequest,
    }
    ///Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address,address[],uint256[],bytes)` and selector `0x5c38449e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "flashLoan",
        abi = "flashLoan(address,address[],uint256[],bytes)"
    )]
    pub struct FlashLoanCall {
        pub recipient: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getActionId` function with signature `getActionId(bytes4)` and selector `0x851c1bb3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getActionId", abi = "getActionId(bytes4)")]
    pub struct GetActionIdCall {
        pub selector: [u8; 4],
    }
    ///Container type for all input parameters for the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAuthorizer", abi = "getAuthorizer()")]
    pub struct GetAuthorizerCall;
    ///Container type for all input parameters for the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getDomainSeparator", abi = "getDomainSeparator()")]
    pub struct GetDomainSeparatorCall;
    ///Container type for all input parameters for the `getInternalBalance` function with signature `getInternalBalance(address,address[])` and selector `0x0f5a6efa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getInternalBalance",
        abi = "getInternalBalance(address,address[])"
    )]
    pub struct GetInternalBalanceCall {
        pub user: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getNextNonce` function with signature `getNextNonce(address)` and selector `0x90193b7c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getNextNonce", abi = "getNextNonce(address)")]
    pub struct GetNextNonceCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPausedState` function with signature `getPausedState()` and selector `0x1c0de051`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPausedState", abi = "getPausedState()")]
    pub struct GetPausedStateCall;
    ///Container type for all input parameters for the `getPool` function with signature `getPool(bytes32)` and selector `0xf6c00927`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPool", abi = "getPool(bytes32)")]
    pub struct GetPoolCall {
        pub pool_id: [u8; 32],
    }
    ///Container type for all input parameters for the `getPoolTokenInfo` function with signature `getPoolTokenInfo(bytes32,address)` and selector `0xb05f8e48`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPoolTokenInfo", abi = "getPoolTokenInfo(bytes32,address)")]
    pub struct GetPoolTokenInfoCall {
        pub pool_id: [u8; 32],
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolTokens` function with signature `getPoolTokens(bytes32)` and selector `0xf94d4668`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPoolTokens", abi = "getPoolTokens(bytes32)")]
    pub struct GetPoolTokensCall {
        pub pool_id: [u8; 32],
    }
    ///Container type for all input parameters for the `getProtocolFeesCollector` function with signature `getProtocolFeesCollector()` and selector `0xd2946c2b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getProtocolFeesCollector", abi = "getProtocolFeesCollector()")]
    pub struct GetProtocolFeesCollectorCall;
    ///Container type for all input parameters for the `hasApprovedRelayer` function with signature `hasApprovedRelayer(address,address)` and selector `0xfec90d72`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "hasApprovedRelayer",
        abi = "hasApprovedRelayer(address,address)"
    )]
    pub struct HasApprovedRelayerCall {
        pub user: ::ethers::core::types::Address,
        pub relayer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `joinPool` function with signature `joinPool(bytes32,address,address,(address[],uint256[],bytes,bool))` and selector `0xb95cac28`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "joinPool",
        abi = "joinPool(bytes32,address,address,(address[],uint256[],bytes,bool))"
    )]
    pub struct JoinPoolCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub request: JoinPoolRequest,
    }
    ///Container type for all input parameters for the `registerPool` function with signature `registerPool(uint8)` and selector `0x09b2760f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "registerPool", abi = "registerPool(uint8)")]
    pub struct RegisterPoolCall {
        pub specialization: u8,
    }
    ///Container type for all input parameters for the `registerTokens` function with signature `registerTokens(bytes32,address[],address[])` and selector `0x66a9c7d2`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "registerTokens",
        abi = "registerTokens(bytes32,address[],address[])"
    )]
    pub struct RegisterTokensCall {
        pub pool_id: [u8; 32],
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub asset_managers: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `setAuthorizer` function with signature `setAuthorizer(address)` and selector `0x058a628f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAuthorizer", abi = "setAuthorizer(address)")]
    pub struct SetAuthorizerCall {
        pub new_authorizer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPaused` function with signature `setPaused(bool)` and selector `0x16c38b3c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPaused", abi = "setPaused(bool)")]
    pub struct SetPausedCall {
        pub paused: bool,
    }
    ///Container type for all input parameters for the `setRelayerApproval` function with signature `setRelayerApproval(address,address,bool)` and selector `0xfa6e671d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setRelayerApproval",
        abi = "setRelayerApproval(address,address,bool)"
    )]
    pub struct SetRelayerApprovalCall {
        pub sender: ::ethers::core::types::Address,
        pub relayer: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap((bytes32,uint8,address,address,uint256,bytes),(address,bool,address,bool),uint256,uint256)` and selector `0x52bbbe29`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "swap",
        abi = "swap((bytes32,uint8,address,address,uint256,bytes),(address,bool,address,bool),uint256,uint256)"
    )]
    pub struct SwapCall {
        pub single_swap: SingleSwap,
        pub funds: FundManagement,
        pub limit: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum VaultCalls {
        Weth(WethCall),
        DeregisterTokens(DeregisterTokensCall),
        ExitPool(ExitPoolCall),
        FlashLoan(FlashLoanCall),
        GetActionId(GetActionIdCall),
        GetAuthorizer(GetAuthorizerCall),
        GetDomainSeparator(GetDomainSeparatorCall),
        GetInternalBalance(GetInternalBalanceCall),
        GetNextNonce(GetNextNonceCall),
        GetPausedState(GetPausedStateCall),
        GetPool(GetPoolCall),
        GetPoolTokenInfo(GetPoolTokenInfoCall),
        GetPoolTokens(GetPoolTokensCall),
        GetProtocolFeesCollector(GetProtocolFeesCollectorCall),
        HasApprovedRelayer(HasApprovedRelayerCall),
        JoinPool(JoinPoolCall),
        RegisterPool(RegisterPoolCall),
        RegisterTokens(RegisterTokensCall),
        SetAuthorizer(SetAuthorizerCall),
        SetPaused(SetPausedCall),
        SetRelayerApproval(SetRelayerApprovalCall),
        Swap(SwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for VaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <DeregisterTokensCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::DeregisterTokens(decoded));
            }
            if let Ok(decoded) =
                <ExitPoolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::ExitPool(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <GetActionIdCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetActionId(decoded));
            }
            if let Ok(decoded) =
                <GetAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetAuthorizer(decoded));
            }
            if let Ok(decoded) =
                <GetDomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetDomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <GetInternalBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetInternalBalance(decoded));
            }
            if let Ok(decoded) =
                <GetNextNonceCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetNextNonce(decoded));
            }
            if let Ok(decoded) =
                <GetPausedStateCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetPausedState(decoded));
            }
            if let Ok(decoded) =
                <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetPool(decoded));
            }
            if let Ok(decoded) =
                <GetPoolTokenInfoCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetPoolTokenInfo(decoded));
            }
            if let Ok(decoded) =
                <GetPoolTokensCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::GetPoolTokens(decoded));
            }
            if let Ok(decoded) =
                <GetProtocolFeesCollectorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VaultCalls::GetProtocolFeesCollector(decoded));
            }
            if let Ok(decoded) =
                <HasApprovedRelayerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::HasApprovedRelayer(decoded));
            }
            if let Ok(decoded) =
                <JoinPoolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::JoinPool(decoded));
            }
            if let Ok(decoded) =
                <RegisterPoolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::RegisterPool(decoded));
            }
            if let Ok(decoded) =
                <RegisterTokensCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::RegisterTokens(decoded));
            }
            if let Ok(decoded) =
                <SetAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::SetAuthorizer(decoded));
            }
            if let Ok(decoded) =
                <SetPausedCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::SetPaused(decoded));
            }
            if let Ok(decoded) =
                <SetRelayerApprovalCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::SetRelayerApproval(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VaultCalls::Swap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VaultCalls::Weth(element) => element.encode(),
                VaultCalls::DeregisterTokens(element) => element.encode(),
                VaultCalls::ExitPool(element) => element.encode(),
                VaultCalls::FlashLoan(element) => element.encode(),
                VaultCalls::GetActionId(element) => element.encode(),
                VaultCalls::GetAuthorizer(element) => element.encode(),
                VaultCalls::GetDomainSeparator(element) => element.encode(),
                VaultCalls::GetInternalBalance(element) => element.encode(),
                VaultCalls::GetNextNonce(element) => element.encode(),
                VaultCalls::GetPausedState(element) => element.encode(),
                VaultCalls::GetPool(element) => element.encode(),
                VaultCalls::GetPoolTokenInfo(element) => element.encode(),
                VaultCalls::GetPoolTokens(element) => element.encode(),
                VaultCalls::GetProtocolFeesCollector(element) => element.encode(),
                VaultCalls::HasApprovedRelayer(element) => element.encode(),
                VaultCalls::JoinPool(element) => element.encode(),
                VaultCalls::RegisterPool(element) => element.encode(),
                VaultCalls::RegisterTokens(element) => element.encode(),
                VaultCalls::SetAuthorizer(element) => element.encode(),
                VaultCalls::SetPaused(element) => element.encode(),
                VaultCalls::SetRelayerApproval(element) => element.encode(),
                VaultCalls::Swap(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VaultCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VaultCalls::Weth(element) => element.fmt(f),
                VaultCalls::DeregisterTokens(element) => element.fmt(f),
                VaultCalls::ExitPool(element) => element.fmt(f),
                VaultCalls::FlashLoan(element) => element.fmt(f),
                VaultCalls::GetActionId(element) => element.fmt(f),
                VaultCalls::GetAuthorizer(element) => element.fmt(f),
                VaultCalls::GetDomainSeparator(element) => element.fmt(f),
                VaultCalls::GetInternalBalance(element) => element.fmt(f),
                VaultCalls::GetNextNonce(element) => element.fmt(f),
                VaultCalls::GetPausedState(element) => element.fmt(f),
                VaultCalls::GetPool(element) => element.fmt(f),
                VaultCalls::GetPoolTokenInfo(element) => element.fmt(f),
                VaultCalls::GetPoolTokens(element) => element.fmt(f),
                VaultCalls::GetProtocolFeesCollector(element) => element.fmt(f),
                VaultCalls::HasApprovedRelayer(element) => element.fmt(f),
                VaultCalls::JoinPool(element) => element.fmt(f),
                VaultCalls::RegisterPool(element) => element.fmt(f),
                VaultCalls::RegisterTokens(element) => element.fmt(f),
                VaultCalls::SetAuthorizer(element) => element.fmt(f),
                VaultCalls::SetPaused(element) => element.fmt(f),
                VaultCalls::SetRelayerApproval(element) => element.fmt(f),
                VaultCalls::Swap(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<WethCall> for VaultCalls {
        fn from(var: WethCall) -> Self {
            VaultCalls::Weth(var)
        }
    }
    impl ::std::convert::From<DeregisterTokensCall> for VaultCalls {
        fn from(var: DeregisterTokensCall) -> Self {
            VaultCalls::DeregisterTokens(var)
        }
    }
    impl ::std::convert::From<ExitPoolCall> for VaultCalls {
        fn from(var: ExitPoolCall) -> Self {
            VaultCalls::ExitPool(var)
        }
    }
    impl ::std::convert::From<FlashLoanCall> for VaultCalls {
        fn from(var: FlashLoanCall) -> Self {
            VaultCalls::FlashLoan(var)
        }
    }
    impl ::std::convert::From<GetActionIdCall> for VaultCalls {
        fn from(var: GetActionIdCall) -> Self {
            VaultCalls::GetActionId(var)
        }
    }
    impl ::std::convert::From<GetAuthorizerCall> for VaultCalls {
        fn from(var: GetAuthorizerCall) -> Self {
            VaultCalls::GetAuthorizer(var)
        }
    }
    impl ::std::convert::From<GetDomainSeparatorCall> for VaultCalls {
        fn from(var: GetDomainSeparatorCall) -> Self {
            VaultCalls::GetDomainSeparator(var)
        }
    }
    impl ::std::convert::From<GetInternalBalanceCall> for VaultCalls {
        fn from(var: GetInternalBalanceCall) -> Self {
            VaultCalls::GetInternalBalance(var)
        }
    }
    impl ::std::convert::From<GetNextNonceCall> for VaultCalls {
        fn from(var: GetNextNonceCall) -> Self {
            VaultCalls::GetNextNonce(var)
        }
    }
    impl ::std::convert::From<GetPausedStateCall> for VaultCalls {
        fn from(var: GetPausedStateCall) -> Self {
            VaultCalls::GetPausedState(var)
        }
    }
    impl ::std::convert::From<GetPoolCall> for VaultCalls {
        fn from(var: GetPoolCall) -> Self {
            VaultCalls::GetPool(var)
        }
    }
    impl ::std::convert::From<GetPoolTokenInfoCall> for VaultCalls {
        fn from(var: GetPoolTokenInfoCall) -> Self {
            VaultCalls::GetPoolTokenInfo(var)
        }
    }
    impl ::std::convert::From<GetPoolTokensCall> for VaultCalls {
        fn from(var: GetPoolTokensCall) -> Self {
            VaultCalls::GetPoolTokens(var)
        }
    }
    impl ::std::convert::From<GetProtocolFeesCollectorCall> for VaultCalls {
        fn from(var: GetProtocolFeesCollectorCall) -> Self {
            VaultCalls::GetProtocolFeesCollector(var)
        }
    }
    impl ::std::convert::From<HasApprovedRelayerCall> for VaultCalls {
        fn from(var: HasApprovedRelayerCall) -> Self {
            VaultCalls::HasApprovedRelayer(var)
        }
    }
    impl ::std::convert::From<JoinPoolCall> for VaultCalls {
        fn from(var: JoinPoolCall) -> Self {
            VaultCalls::JoinPool(var)
        }
    }
    impl ::std::convert::From<RegisterPoolCall> for VaultCalls {
        fn from(var: RegisterPoolCall) -> Self {
            VaultCalls::RegisterPool(var)
        }
    }
    impl ::std::convert::From<RegisterTokensCall> for VaultCalls {
        fn from(var: RegisterTokensCall) -> Self {
            VaultCalls::RegisterTokens(var)
        }
    }
    impl ::std::convert::From<SetAuthorizerCall> for VaultCalls {
        fn from(var: SetAuthorizerCall) -> Self {
            VaultCalls::SetAuthorizer(var)
        }
    }
    impl ::std::convert::From<SetPausedCall> for VaultCalls {
        fn from(var: SetPausedCall) -> Self {
            VaultCalls::SetPaused(var)
        }
    }
    impl ::std::convert::From<SetRelayerApprovalCall> for VaultCalls {
        fn from(var: SetRelayerApprovalCall) -> Self {
            VaultCalls::SetRelayerApproval(var)
        }
    }
    impl ::std::convert::From<SwapCall> for VaultCalls {
        fn from(var: SwapCall) -> Self {
            VaultCalls::Swap(var)
        }
    }
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getActionId` function with signature `getActionId(bytes4)` and selector `0x851c1bb3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetActionIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetAuthorizerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetDomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getInternalBalance` function with signature `getInternalBalance(address,address[])` and selector `0x0f5a6efa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetInternalBalanceReturn {
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getNextNonce` function with signature `getNextNonce(address)` and selector `0x90193b7c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetNextNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPausedState` function with signature `getPausedState()` and selector `0x1c0de051`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetPausedStateReturn {
        pub paused: bool,
        pub pause_window_end_time: ::ethers::core::types::U256,
        pub buffer_period_end_time: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getPool` function with signature `getPool(bytes32)` and selector `0xf6c00927`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetPoolReturn(pub ::ethers::core::types::Address, pub u8);
    ///Container type for all return fields from the `getPoolTokenInfo` function with signature `getPoolTokenInfo(bytes32,address)` and selector `0xb05f8e48`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetPoolTokenInfoReturn {
        pub cash: ::ethers::core::types::U256,
        pub managed: ::ethers::core::types::U256,
        pub last_change_block: ::ethers::core::types::U256,
        pub asset_manager: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getPoolTokens` function with signature `getPoolTokens(bytes32)` and selector `0xf94d4668`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetPoolTokensReturn {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub last_change_block: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getProtocolFeesCollector` function with signature `getProtocolFeesCollector()` and selector `0xd2946c2b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetProtocolFeesCollectorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasApprovedRelayer` function with signature `hasApprovedRelayer(address,address)` and selector `0xfec90d72`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct HasApprovedRelayerReturn(pub bool);
    ///Container type for all return fields from the `registerPool` function with signature `registerPool(uint8)` and selector `0x09b2760f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RegisterPoolReturn(pub [u8; 32]);
    ///Container type for all return fields from the `swap` function with signature `swap((bytes32,uint8,address,address,uint256,bytes),(address,bool,address,bool),uint256,uint256)` and selector `0x52bbbe29`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SwapReturn {
        pub amount_calculated: ::ethers::core::types::U256,
    }
    ///`ExitPoolRequest(address[],uint256[],bytes,bool)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct ExitPoolRequest {
        pub a: Vec<::ethers::core::types::Address>,
        pub b: Vec<::ethers::core::types::U256>,
        pub c: ::ethers::core::types::Bytes,
        pub d: bool,
    }
    ///`FundManagement(address,bool,address,bool)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct FundManagement {
        pub a: ::ethers::core::types::Address,
        pub b: bool,
        pub c: ::ethers::core::types::Address,
        pub d: bool,
    }
    ///`JoinPoolRequest(address[],uint256[],bytes,bool)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct JoinPoolRequest {
        pub a: Vec<::ethers::core::types::Address>,
        pub b: Vec<::ethers::core::types::U256>,
        pub c: ::ethers::core::types::Bytes,
        pub d: bool,
    }
    ///`SingleSwap(bytes32,uint8,address,address,uint256,bytes)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct SingleSwap {
        pub a: [u8; 32],
        pub b: u8,
        pub c: ::ethers::core::types::Address,
        pub d: ::ethers::core::types::Address,
        pub e: ::ethers::core::types::U256,
        pub f: ::ethers::core::types::Bytes,
    }
}
