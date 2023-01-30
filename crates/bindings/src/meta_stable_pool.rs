pub use meta_stable_pool::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod meta_stable_pool {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///MetaStablePool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"startValue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"endValue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AmpUpdateStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"currentValue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AmpUpdateStopped\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleEnabledChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PausedStateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceRateCacheUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"provider\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"cacheDuration\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceRateProviderSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"swapFeePercentage\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SwapFeePercentageChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"selector\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getActionId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmplificationParameter\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isUpdating\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"precision\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAuthorizer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getLargestSafeQueryWindow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastInvariant\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lastInvariant\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastInvariantAmp\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"variable\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOracleMiscData\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"logInvariant\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"logTotalSupply\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"oracleSampleCreationTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"oracleIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"oracleEnabled\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPausedState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"pauseWindowEndTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bufferPeriodEndTime\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceRateCache\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"duration\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expires\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateProviders\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"providers\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSample\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"logPairPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"accLogPairPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"logBptPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"accLogBptPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"logInvariant\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"accLogInvariant\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getScalingFactors\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSwapFeePercentage\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getTotalSamples\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVault\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastChangeBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolSwapFeePercentage\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"userData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onExitPool\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amountsOut\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"dueProtocolFeeAmounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastChangeBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolSwapFeePercentage\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"userData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onJoinPool\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amountsIn\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"dueProtocolFeeAmounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct MetaStablePool.SwapRequest\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"a\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"b\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"c\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"d\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"e\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"f\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"g\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"h\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"i\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"indexIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"indexOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onSwap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct MetaStablePool.SwapRequest\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"a\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"b\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"c\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"d\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"e\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"f\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"g\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"h\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"i\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"balanceTokenIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"balanceTokenOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onSwap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastChangeBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolSwapFeePercentage\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"userData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queryExit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"bptIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amountsOut\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"poolId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastChangeBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolSwapFeePercentage\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"userData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queryJoin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"bptOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amountsIn\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"poolConfig\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssetManagerPoolConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaused\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"duration\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPriceRateCacheDuration\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"swapFeePercentage\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSwapFeePercentage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rawEndValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startAmplificationParameterUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stopAmplificationParameterUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePriceRateCache\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static METASTABLEPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    pub struct MetaStablePool<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for MetaStablePool<M> {
        fn clone(&self) -> Self {
            MetaStablePool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MetaStablePool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MetaStablePool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MetaStablePool)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MetaStablePool<M> {
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
                    METASTABLEPOOL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableOracle` (0x292c914a) function
        pub fn enable_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 44, 145, 74], ())
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
        ///Calls the contract's `getAmplificationParameter` (0x6daccffa) function
        pub fn get_amplification_parameter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([109, 172, 207, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAuthorizer` (0xaaabadc5) function
        pub fn get_authorizer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([170, 171, 173, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLargestSafeQueryWindow` (0xffd088eb) function
        pub fn get_largest_safe_query_window(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([255, 208, 136, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLastInvariant` (0x9b02cdde) function
        pub fn get_last_invariant(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([155, 2, 205, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatest` (0xb10be739) function
        pub fn get_latest(
            &self,
            variable: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([177, 11, 231, 57], variable)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOracleMiscData` (0x1ed4eddc) function
        pub fn get_oracle_misc_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([30, 212, 237, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOwner` (0x893d20e8) function
        pub fn get_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([137, 61, 32, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPausedState` (0x1c0de051) function
        pub fn get_paused_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([28, 13, 224, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolId` (0x38fff2d0) function
        pub fn get_pool_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([56, 255, 242, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceRateCache` (0xb867ee5a) function
        pub fn get_price_rate_cache(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([184, 103, 238, 90], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRate` (0x679aefce) function
        pub fn get_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 154, 239, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRateProviders` (0x238a2d59) function
        pub fn get_rate_providers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([35, 138, 45, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSample` (0x60d1507c) function
        pub fn get_sample(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([96, 209, 80, 124], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getScalingFactors` (0x1dd746ea) function
        pub fn get_scaling_factors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([29, 215, 70, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapFeePercentage` (0x55c67628) function
        pub fn get_swap_fee_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 198, 118, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalSamples` (0xb48b5b40) function
        pub fn get_total_samples(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 139, 91, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVault` (0x8d928af8) function
        pub fn get_vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 146, 138, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onExitPool` (0x74f3b009) function
        pub fn on_exit_pool(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            balances: ::std::vec::Vec<::ethers::core::types::U256>,
            last_change_block: ::ethers::core::types::U256,
            protocol_swap_fee_percentage: ::ethers::core::types::U256,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash(
                    [116, 243, 176, 9],
                    (
                        pool_id,
                        sender,
                        recipient,
                        balances,
                        last_change_block,
                        protocol_swap_fee_percentage,
                        user_data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onJoinPool` (0xd5c096c4) function
        pub fn on_join_pool(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            balances: ::std::vec::Vec<::ethers::core::types::U256>,
            last_change_block: ::ethers::core::types::U256,
            protocol_swap_fee_percentage: ::ethers::core::types::U256,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash(
                    [213, 192, 150, 196],
                    (
                        pool_id,
                        sender,
                        recipient,
                        balances,
                        last_change_block,
                        protocol_swap_fee_percentage,
                        user_data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onSwap` (0x01ec954a) function
        pub fn on_swap_with_request_and_balances_and_index_in(
            &self,
            request: SwapRequest,
            balances: ::std::vec::Vec<::ethers::core::types::U256>,
            index_in: ::ethers::core::types::U256,
            index_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 236, 149, 74], (request, balances, index_in, index_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onSwap` (0x9d2c110c) function
        pub fn on_swap(
            &self,
            request: SwapRequest,
            balance_token_in: ::ethers::core::types::U256,
            balance_token_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [157, 44, 17, 12],
                    (request, balance_token_in, balance_token_out),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryExit` (0x6028bfd4) function
        pub fn query_exit(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            balances: ::std::vec::Vec<::ethers::core::types::U256>,
            last_change_block: ::ethers::core::types::U256,
            protocol_swap_fee_percentage: ::ethers::core::types::U256,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::std::vec::Vec<::ethers::core::types::U256>),
        > {
            self.0
                .method_hash(
                    [96, 40, 191, 212],
                    (
                        pool_id,
                        sender,
                        recipient,
                        balances,
                        last_change_block,
                        protocol_swap_fee_percentage,
                        user_data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryJoin` (0x87ec6817) function
        pub fn query_join(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            balances: ::std::vec::Vec<::ethers::core::types::U256>,
            last_change_block: ::ethers::core::types::U256,
            protocol_swap_fee_percentage: ::ethers::core::types::U256,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::std::vec::Vec<::ethers::core::types::U256>),
        > {
            self.0
                .method_hash(
                    [135, 236, 104, 23],
                    (
                        pool_id,
                        sender,
                        recipient,
                        balances,
                        last_change_block,
                        protocol_swap_fee_percentage,
                        user_data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAssetManagerPoolConfig` (0x50dd6ed9) function
        pub fn set_asset_manager_pool_config(
            &self,
            token: ::ethers::core::types::Address,
            pool_config: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 221, 110, 217], (token, pool_config))
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
        ///Calls the contract's `setPriceRateCacheDuration` (0xb7710251) function
        pub fn set_price_rate_cache_duration(
            &self,
            token: ::ethers::core::types::Address,
            duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 113, 2, 81], (token, duration))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSwapFeePercentage` (0x38e9922e) function
        pub fn set_swap_fee_percentage(
            &self,
            swap_fee_percentage: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 233, 146, 46], swap_fee_percentage)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startAmplificationParameterUpdate` (0x2f1a0bc9) function
        pub fn start_amplification_parameter_update(
            &self,
            raw_end_value: ::ethers::core::types::U256,
            end_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 26, 11, 201], (raw_end_value, end_time))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopAmplificationParameterUpdate` (0xeb0f24d6) function
        pub fn stop_amplification_parameter_update(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 15, 36, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePriceRateCache` (0xa0daaed0) function
        pub fn update_price_rate_cache(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 218, 174, 208], token)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AmpUpdateStarted` event
        pub fn amp_update_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, AmpUpdateStartedFilter> {
            self.0.event()
        }
        ///Gets the contract's `AmpUpdateStopped` event
        pub fn amp_update_stopped_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, AmpUpdateStoppedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `OracleEnabledChanged` event
        pub fn oracle_enabled_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, OracleEnabledChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PausedStateChanged` event
        pub fn paused_state_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PausedStateChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PriceRateCacheUpdated` event
        pub fn price_rate_cache_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PriceRateCacheUpdatedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PriceRateProviderSet` event
        pub fn price_rate_provider_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PriceRateProviderSetFilter> {
            self.0.event()
        }
        ///Gets the contract's `SwapFeePercentageChanged` event
        pub fn swap_fee_percentage_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, SwapFeePercentageChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<M, MetaStablePoolEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MetaStablePool<M> {
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
    )]
    #[derive(Default)]
    #[ethevent(
        name = "AmpUpdateStarted",
        abi = "AmpUpdateStarted(uint256,uint256,uint256,uint256)"
    )]
    pub struct AmpUpdateStartedFilter {
        pub start_value: ::ethers::core::types::U256,
        pub end_value: ::ethers::core::types::U256,
        pub start_time: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "AmpUpdateStopped", abi = "AmpUpdateStopped(uint256)")]
    pub struct AmpUpdateStoppedFilter {
        pub current_value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "OracleEnabledChanged", abi = "OracleEnabledChanged(bool)")]
    pub struct OracleEnabledChangedFilter {
        pub enabled: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
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
    )]
    #[derive(Default)]
    #[ethevent(
        name = "PriceRateCacheUpdated",
        abi = "PriceRateCacheUpdated(address,uint256)"
    )]
    pub struct PriceRateCacheUpdatedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub rate: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "PriceRateProviderSet",
        abi = "PriceRateProviderSet(address,address,uint256)"
    )]
    pub struct PriceRateProviderSetFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub provider: ::ethers::core::types::Address,
        pub cache_duration: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "SwapFeePercentageChanged",
        abi = "SwapFeePercentageChanged(uint256)"
    )]
    pub struct SwapFeePercentageChangedFilter {
        pub swap_fee_percentage: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum MetaStablePoolEvents {
        AmpUpdateStartedFilter(AmpUpdateStartedFilter),
        AmpUpdateStoppedFilter(AmpUpdateStoppedFilter),
        ApprovalFilter(ApprovalFilter),
        OracleEnabledChangedFilter(OracleEnabledChangedFilter),
        PausedStateChangedFilter(PausedStateChangedFilter),
        PriceRateCacheUpdatedFilter(PriceRateCacheUpdatedFilter),
        PriceRateProviderSetFilter(PriceRateProviderSetFilter),
        SwapFeePercentageChangedFilter(SwapFeePercentageChangedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MetaStablePoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AmpUpdateStartedFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::AmpUpdateStartedFilter(decoded));
            }
            if let Ok(decoded) = AmpUpdateStoppedFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::AmpUpdateStoppedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = OracleEnabledChangedFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::OracleEnabledChangedFilter(decoded));
            }
            if let Ok(decoded) = PausedStateChangedFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::PausedStateChangedFilter(decoded));
            }
            if let Ok(decoded) = PriceRateCacheUpdatedFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::PriceRateCacheUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceRateProviderSetFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::PriceRateProviderSetFilter(decoded));
            }
            if let Ok(decoded) = SwapFeePercentageChangedFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::SwapFeePercentageChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MetaStablePoolEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MetaStablePoolEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MetaStablePoolEvents::AmpUpdateStartedFilter(element) => element.fmt(f),
                MetaStablePoolEvents::AmpUpdateStoppedFilter(element) => element.fmt(f),
                MetaStablePoolEvents::ApprovalFilter(element) => element.fmt(f),
                MetaStablePoolEvents::OracleEnabledChangedFilter(element) => {
                    element.fmt(f)
                }
                MetaStablePoolEvents::PausedStateChangedFilter(element) => element.fmt(f),
                MetaStablePoolEvents::PriceRateCacheUpdatedFilter(element) => {
                    element.fmt(f)
                }
                MetaStablePoolEvents::PriceRateProviderSetFilter(element) => {
                    element.fmt(f)
                }
                MetaStablePoolEvents::SwapFeePercentageChangedFilter(element) => {
                    element.fmt(f)
                }
                MetaStablePoolEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `enableOracle` function with signature `enableOracle()` and selector `0x292c914a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "enableOracle", abi = "enableOracle()")]
    pub struct EnableOracleCall;
    ///Container type for all input parameters for the `getActionId` function with signature `getActionId(bytes4)` and selector `0x851c1bb3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getActionId", abi = "getActionId(bytes4)")]
    pub struct GetActionIdCall {
        pub selector: [u8; 4],
    }
    ///Container type for all input parameters for the `getAmplificationParameter` function with signature `getAmplificationParameter()` and selector `0x6daccffa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getAmplificationParameter", abi = "getAmplificationParameter()")]
    pub struct GetAmplificationParameterCall;
    ///Container type for all input parameters for the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getAuthorizer", abi = "getAuthorizer()")]
    pub struct GetAuthorizerCall;
    ///Container type for all input parameters for the `getLargestSafeQueryWindow` function with signature `getLargestSafeQueryWindow()` and selector `0xffd088eb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getLargestSafeQueryWindow", abi = "getLargestSafeQueryWindow()")]
    pub struct GetLargestSafeQueryWindowCall;
    ///Container type for all input parameters for the `getLastInvariant` function with signature `getLastInvariant()` and selector `0x9b02cdde`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getLastInvariant", abi = "getLastInvariant()")]
    pub struct GetLastInvariantCall;
    ///Container type for all input parameters for the `getLatest` function with signature `getLatest(uint8)` and selector `0xb10be739`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getLatest", abi = "getLatest(uint8)")]
    pub struct GetLatestCall {
        pub variable: u8,
    }
    ///Container type for all input parameters for the `getOracleMiscData` function with signature `getOracleMiscData()` and selector `0x1ed4eddc`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getOracleMiscData", abi = "getOracleMiscData()")]
    pub struct GetOracleMiscDataCall;
    ///Container type for all input parameters for the `getOwner` function with signature `getOwner()` and selector `0x893d20e8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getOwner", abi = "getOwner()")]
    pub struct GetOwnerCall;
    ///Container type for all input parameters for the `getPausedState` function with signature `getPausedState()` and selector `0x1c0de051`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getPausedState", abi = "getPausedState()")]
    pub struct GetPausedStateCall;
    ///Container type for all input parameters for the `getPoolId` function with signature `getPoolId()` and selector `0x38fff2d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getPoolId", abi = "getPoolId()")]
    pub struct GetPoolIdCall;
    ///Container type for all input parameters for the `getPriceRateCache` function with signature `getPriceRateCache(address)` and selector `0xb867ee5a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getPriceRateCache", abi = "getPriceRateCache(address)")]
    pub struct GetPriceRateCacheCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getRate` function with signature `getRate()` and selector `0x679aefce`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getRate", abi = "getRate()")]
    pub struct GetRateCall;
    ///Container type for all input parameters for the `getRateProviders` function with signature `getRateProviders()` and selector `0x238a2d59`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getRateProviders", abi = "getRateProviders()")]
    pub struct GetRateProvidersCall;
    ///Container type for all input parameters for the `getSample` function with signature `getSample(uint256)` and selector `0x60d1507c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getSample", abi = "getSample(uint256)")]
    pub struct GetSampleCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getScalingFactors` function with signature `getScalingFactors()` and selector `0x1dd746ea`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getScalingFactors", abi = "getScalingFactors()")]
    pub struct GetScalingFactorsCall;
    ///Container type for all input parameters for the `getSwapFeePercentage` function with signature `getSwapFeePercentage()` and selector `0x55c67628`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getSwapFeePercentage", abi = "getSwapFeePercentage()")]
    pub struct GetSwapFeePercentageCall;
    ///Container type for all input parameters for the `getTotalSamples` function with signature `getTotalSamples()` and selector `0xb48b5b40`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getTotalSamples", abi = "getTotalSamples()")]
    pub struct GetTotalSamplesCall;
    ///Container type for all input parameters for the `getVault` function with signature `getVault()` and selector `0x8d928af8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getVault", abi = "getVault()")]
    pub struct GetVaultCall;
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `onExitPool` function with signature `onExitPool(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0x74f3b009`
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
        name = "onExitPool",
        abi = "onExitPool(bytes32,address,address,uint256[],uint256,uint256,bytes)"
    )]
    pub struct OnExitPoolCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub last_change_block: ::ethers::core::types::U256,
        pub protocol_swap_fee_percentage: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `onJoinPool` function with signature `onJoinPool(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0xd5c096c4`
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
        name = "onJoinPool",
        abi = "onJoinPool(bytes32,address,address,uint256[],uint256,uint256,bytes)"
    )]
    pub struct OnJoinPoolCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub last_change_block: ::ethers::core::types::U256,
        pub protocol_swap_fee_percentage: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `onSwap` function with signature `onSwap((uint8,address,address,uint256,bytes32,uint256,address,address,bytes),uint256[],uint256,uint256)` and selector `0x01ec954a`
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
        name = "onSwap",
        abi = "onSwap((uint8,address,address,uint256,bytes32,uint256,address,address,bytes),uint256[],uint256,uint256)"
    )]
    pub struct OnSwapWithRequestAndBalancesAndIndexInCall {
        pub request: SwapRequest,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub index_in: ::ethers::core::types::U256,
        pub index_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `onSwap` function with signature `onSwap((uint8,address,address,uint256,bytes32,uint256,address,address,bytes),uint256,uint256)` and selector `0x9d2c110c`
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
        name = "onSwap",
        abi = "onSwap((uint8,address,address,uint256,bytes32,uint256,address,address,bytes),uint256,uint256)"
    )]
    pub struct OnSwapCall {
        pub request: SwapRequest,
        pub balance_token_in: ::ethers::core::types::U256,
        pub balance_token_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `queryExit` function with signature `queryExit(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0x6028bfd4`
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
        name = "queryExit",
        abi = "queryExit(bytes32,address,address,uint256[],uint256,uint256,bytes)"
    )]
    pub struct QueryExitCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub last_change_block: ::ethers::core::types::U256,
        pub protocol_swap_fee_percentage: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `queryJoin` function with signature `queryJoin(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0x87ec6817`
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
        name = "queryJoin",
        abi = "queryJoin(bytes32,address,address,uint256[],uint256,uint256,bytes)"
    )]
    pub struct QueryJoinCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub last_change_block: ::ethers::core::types::U256,
        pub protocol_swap_fee_percentage: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setAssetManagerPoolConfig` function with signature `setAssetManagerPoolConfig(address,bytes)` and selector `0x50dd6ed9`
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
        name = "setAssetManagerPoolConfig",
        abi = "setAssetManagerPoolConfig(address,bytes)"
    )]
    pub struct SetAssetManagerPoolConfigCall {
        pub token: ::ethers::core::types::Address,
        pub pool_config: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setPaused` function with signature `setPaused(bool)` and selector `0x16c38b3c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "setPaused", abi = "setPaused(bool)")]
    pub struct SetPausedCall {
        pub paused: bool,
    }
    ///Container type for all input parameters for the `setPriceRateCacheDuration` function with signature `setPriceRateCacheDuration(address,uint256)` and selector `0xb7710251`
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
        name = "setPriceRateCacheDuration",
        abi = "setPriceRateCacheDuration(address,uint256)"
    )]
    pub struct SetPriceRateCacheDurationCall {
        pub token: ::ethers::core::types::Address,
        pub duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setSwapFeePercentage` function with signature `setSwapFeePercentage(uint256)` and selector `0x38e9922e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "setSwapFeePercentage", abi = "setSwapFeePercentage(uint256)")]
    pub struct SetSwapFeePercentageCall {
        pub swap_fee_percentage: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `startAmplificationParameterUpdate` function with signature `startAmplificationParameterUpdate(uint256,uint256)` and selector `0x2f1a0bc9`
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
        name = "startAmplificationParameterUpdate",
        abi = "startAmplificationParameterUpdate(uint256,uint256)"
    )]
    pub struct StartAmplificationParameterUpdateCall {
        pub raw_end_value: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stopAmplificationParameterUpdate` function with signature `stopAmplificationParameterUpdate()` and selector `0xeb0f24d6`
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
        name = "stopAmplificationParameterUpdate",
        abi = "stopAmplificationParameterUpdate()"
    )]
    pub struct StopAmplificationParameterUpdateCall;
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updatePriceRateCache` function with signature `updatePriceRateCache(address)` and selector `0xa0daaed0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "updatePriceRateCache", abi = "updatePriceRateCache(address)")]
    pub struct UpdatePriceRateCacheCall {
        pub token: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum MetaStablePoolCalls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        EnableOracle(EnableOracleCall),
        GetActionId(GetActionIdCall),
        GetAmplificationParameter(GetAmplificationParameterCall),
        GetAuthorizer(GetAuthorizerCall),
        GetLargestSafeQueryWindow(GetLargestSafeQueryWindowCall),
        GetLastInvariant(GetLastInvariantCall),
        GetLatest(GetLatestCall),
        GetOracleMiscData(GetOracleMiscDataCall),
        GetOwner(GetOwnerCall),
        GetPausedState(GetPausedStateCall),
        GetPoolId(GetPoolIdCall),
        GetPriceRateCache(GetPriceRateCacheCall),
        GetRate(GetRateCall),
        GetRateProviders(GetRateProvidersCall),
        GetSample(GetSampleCall),
        GetScalingFactors(GetScalingFactorsCall),
        GetSwapFeePercentage(GetSwapFeePercentageCall),
        GetTotalSamples(GetTotalSamplesCall),
        GetVault(GetVaultCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Name(NameCall),
        Nonces(NoncesCall),
        OnExitPool(OnExitPoolCall),
        OnJoinPool(OnJoinPoolCall),
        OnSwapWithRequestAndBalancesAndIndexIn(
            OnSwapWithRequestAndBalancesAndIndexInCall,
        ),
        OnSwap(OnSwapCall),
        Permit(PermitCall),
        QueryExit(QueryExitCall),
        QueryJoin(QueryJoinCall),
        SetAssetManagerPoolConfig(SetAssetManagerPoolConfigCall),
        SetPaused(SetPausedCall),
        SetPriceRateCacheDuration(SetPriceRateCacheDurationCall),
        SetSwapFeePercentage(SetSwapFeePercentageCall),
        StartAmplificationParameterUpdate(StartAmplificationParameterUpdateCall),
        StopAmplificationParameterUpdate(StopAmplificationParameterUpdateCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        UpdatePriceRateCache(UpdatePriceRateCacheCall),
    }
    impl ::ethers::core::abi::AbiDecode for MetaStablePoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <EnableOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::EnableOracle(decoded));
            }
            if let Ok(decoded)
                = <GetActionIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetActionId(decoded));
            }
            if let Ok(decoded)
                = <GetAmplificationParameterCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetAmplificationParameter(decoded));
            }
            if let Ok(decoded)
                = <GetAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetAuthorizer(decoded));
            }
            if let Ok(decoded)
                = <GetLargestSafeQueryWindowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetLargestSafeQueryWindow(decoded));
            }
            if let Ok(decoded)
                = <GetLastInvariantCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetLastInvariant(decoded));
            }
            if let Ok(decoded)
                = <GetLatestCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetLatest(decoded));
            }
            if let Ok(decoded)
                = <GetOracleMiscDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetOracleMiscData(decoded));
            }
            if let Ok(decoded)
                = <GetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetOwner(decoded));
            }
            if let Ok(decoded)
                = <GetPausedStateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetPausedState(decoded));
            }
            if let Ok(decoded)
                = <GetPoolIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetPoolId(decoded));
            }
            if let Ok(decoded)
                = <GetPriceRateCacheCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetPriceRateCache(decoded));
            }
            if let Ok(decoded)
                = <GetRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetRate(decoded));
            }
            if let Ok(decoded)
                = <GetRateProvidersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetRateProviders(decoded));
            }
            if let Ok(decoded)
                = <GetSampleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetSample(decoded));
            }
            if let Ok(decoded)
                = <GetScalingFactorsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetScalingFactors(decoded));
            }
            if let Ok(decoded)
                = <GetSwapFeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetSwapFeePercentage(decoded));
            }
            if let Ok(decoded)
                = <GetTotalSamplesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetTotalSamples(decoded));
            }
            if let Ok(decoded)
                = <GetVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::GetVault(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MetaStablePoolCalls::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MetaStablePoolCalls::Nonces(decoded));
            }
            if let Ok(decoded)
                = <OnExitPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::OnExitPool(decoded));
            }
            if let Ok(decoded)
                = <OnJoinPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::OnJoinPool(decoded));
            }
            if let Ok(decoded)
                = <OnSwapWithRequestAndBalancesAndIndexInCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    MetaStablePoolCalls::OnSwapWithRequestAndBalancesAndIndexIn(decoded),
                );
            }
            if let Ok(decoded)
                = <OnSwapCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MetaStablePoolCalls::OnSwap(decoded));
            }
            if let Ok(decoded)
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MetaStablePoolCalls::Permit(decoded));
            }
            if let Ok(decoded)
                = <QueryExitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::QueryExit(decoded));
            }
            if let Ok(decoded)
                = <QueryJoinCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::QueryJoin(decoded));
            }
            if let Ok(decoded)
                = <SetAssetManagerPoolConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::SetAssetManagerPoolConfig(decoded));
            }
            if let Ok(decoded)
                = <SetPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::SetPaused(decoded));
            }
            if let Ok(decoded)
                = <SetPriceRateCacheDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::SetPriceRateCacheDuration(decoded));
            }
            if let Ok(decoded)
                = <SetSwapFeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::SetSwapFeePercentage(decoded));
            }
            if let Ok(decoded)
                = <StartAmplificationParameterUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    MetaStablePoolCalls::StartAmplificationParameterUpdate(decoded),
                );
            }
            if let Ok(decoded)
                = <StopAmplificationParameterUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    MetaStablePoolCalls::StopAmplificationParameterUpdate(decoded),
                );
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MetaStablePoolCalls::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <UpdatePriceRateCacheCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MetaStablePoolCalls::UpdatePriceRateCache(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MetaStablePoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MetaStablePoolCalls::DomainSeparator(element) => element.encode(),
                MetaStablePoolCalls::Allowance(element) => element.encode(),
                MetaStablePoolCalls::Approve(element) => element.encode(),
                MetaStablePoolCalls::BalanceOf(element) => element.encode(),
                MetaStablePoolCalls::Decimals(element) => element.encode(),
                MetaStablePoolCalls::DecreaseAllowance(element) => element.encode(),
                MetaStablePoolCalls::EnableOracle(element) => element.encode(),
                MetaStablePoolCalls::GetActionId(element) => element.encode(),
                MetaStablePoolCalls::GetAmplificationParameter(element) => {
                    element.encode()
                }
                MetaStablePoolCalls::GetAuthorizer(element) => element.encode(),
                MetaStablePoolCalls::GetLargestSafeQueryWindow(element) => {
                    element.encode()
                }
                MetaStablePoolCalls::GetLastInvariant(element) => element.encode(),
                MetaStablePoolCalls::GetLatest(element) => element.encode(),
                MetaStablePoolCalls::GetOracleMiscData(element) => element.encode(),
                MetaStablePoolCalls::GetOwner(element) => element.encode(),
                MetaStablePoolCalls::GetPausedState(element) => element.encode(),
                MetaStablePoolCalls::GetPoolId(element) => element.encode(),
                MetaStablePoolCalls::GetPriceRateCache(element) => element.encode(),
                MetaStablePoolCalls::GetRate(element) => element.encode(),
                MetaStablePoolCalls::GetRateProviders(element) => element.encode(),
                MetaStablePoolCalls::GetSample(element) => element.encode(),
                MetaStablePoolCalls::GetScalingFactors(element) => element.encode(),
                MetaStablePoolCalls::GetSwapFeePercentage(element) => element.encode(),
                MetaStablePoolCalls::GetTotalSamples(element) => element.encode(),
                MetaStablePoolCalls::GetVault(element) => element.encode(),
                MetaStablePoolCalls::IncreaseAllowance(element) => element.encode(),
                MetaStablePoolCalls::Name(element) => element.encode(),
                MetaStablePoolCalls::Nonces(element) => element.encode(),
                MetaStablePoolCalls::OnExitPool(element) => element.encode(),
                MetaStablePoolCalls::OnJoinPool(element) => element.encode(),
                MetaStablePoolCalls::OnSwapWithRequestAndBalancesAndIndexIn(element) => {
                    element.encode()
                }
                MetaStablePoolCalls::OnSwap(element) => element.encode(),
                MetaStablePoolCalls::Permit(element) => element.encode(),
                MetaStablePoolCalls::QueryExit(element) => element.encode(),
                MetaStablePoolCalls::QueryJoin(element) => element.encode(),
                MetaStablePoolCalls::SetAssetManagerPoolConfig(element) => {
                    element.encode()
                }
                MetaStablePoolCalls::SetPaused(element) => element.encode(),
                MetaStablePoolCalls::SetPriceRateCacheDuration(element) => {
                    element.encode()
                }
                MetaStablePoolCalls::SetSwapFeePercentage(element) => element.encode(),
                MetaStablePoolCalls::StartAmplificationParameterUpdate(element) => {
                    element.encode()
                }
                MetaStablePoolCalls::StopAmplificationParameterUpdate(element) => {
                    element.encode()
                }
                MetaStablePoolCalls::Symbol(element) => element.encode(),
                MetaStablePoolCalls::TotalSupply(element) => element.encode(),
                MetaStablePoolCalls::Transfer(element) => element.encode(),
                MetaStablePoolCalls::TransferFrom(element) => element.encode(),
                MetaStablePoolCalls::UpdatePriceRateCache(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MetaStablePoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MetaStablePoolCalls::DomainSeparator(element) => element.fmt(f),
                MetaStablePoolCalls::Allowance(element) => element.fmt(f),
                MetaStablePoolCalls::Approve(element) => element.fmt(f),
                MetaStablePoolCalls::BalanceOf(element) => element.fmt(f),
                MetaStablePoolCalls::Decimals(element) => element.fmt(f),
                MetaStablePoolCalls::DecreaseAllowance(element) => element.fmt(f),
                MetaStablePoolCalls::EnableOracle(element) => element.fmt(f),
                MetaStablePoolCalls::GetActionId(element) => element.fmt(f),
                MetaStablePoolCalls::GetAmplificationParameter(element) => element.fmt(f),
                MetaStablePoolCalls::GetAuthorizer(element) => element.fmt(f),
                MetaStablePoolCalls::GetLargestSafeQueryWindow(element) => element.fmt(f),
                MetaStablePoolCalls::GetLastInvariant(element) => element.fmt(f),
                MetaStablePoolCalls::GetLatest(element) => element.fmt(f),
                MetaStablePoolCalls::GetOracleMiscData(element) => element.fmt(f),
                MetaStablePoolCalls::GetOwner(element) => element.fmt(f),
                MetaStablePoolCalls::GetPausedState(element) => element.fmt(f),
                MetaStablePoolCalls::GetPoolId(element) => element.fmt(f),
                MetaStablePoolCalls::GetPriceRateCache(element) => element.fmt(f),
                MetaStablePoolCalls::GetRate(element) => element.fmt(f),
                MetaStablePoolCalls::GetRateProviders(element) => element.fmt(f),
                MetaStablePoolCalls::GetSample(element) => element.fmt(f),
                MetaStablePoolCalls::GetScalingFactors(element) => element.fmt(f),
                MetaStablePoolCalls::GetSwapFeePercentage(element) => element.fmt(f),
                MetaStablePoolCalls::GetTotalSamples(element) => element.fmt(f),
                MetaStablePoolCalls::GetVault(element) => element.fmt(f),
                MetaStablePoolCalls::IncreaseAllowance(element) => element.fmt(f),
                MetaStablePoolCalls::Name(element) => element.fmt(f),
                MetaStablePoolCalls::Nonces(element) => element.fmt(f),
                MetaStablePoolCalls::OnExitPool(element) => element.fmt(f),
                MetaStablePoolCalls::OnJoinPool(element) => element.fmt(f),
                MetaStablePoolCalls::OnSwapWithRequestAndBalancesAndIndexIn(element) => {
                    element.fmt(f)
                }
                MetaStablePoolCalls::OnSwap(element) => element.fmt(f),
                MetaStablePoolCalls::Permit(element) => element.fmt(f),
                MetaStablePoolCalls::QueryExit(element) => element.fmt(f),
                MetaStablePoolCalls::QueryJoin(element) => element.fmt(f),
                MetaStablePoolCalls::SetAssetManagerPoolConfig(element) => element.fmt(f),
                MetaStablePoolCalls::SetPaused(element) => element.fmt(f),
                MetaStablePoolCalls::SetPriceRateCacheDuration(element) => element.fmt(f),
                MetaStablePoolCalls::SetSwapFeePercentage(element) => element.fmt(f),
                MetaStablePoolCalls::StartAmplificationParameterUpdate(element) => {
                    element.fmt(f)
                }
                MetaStablePoolCalls::StopAmplificationParameterUpdate(element) => {
                    element.fmt(f)
                }
                MetaStablePoolCalls::Symbol(element) => element.fmt(f),
                MetaStablePoolCalls::TotalSupply(element) => element.fmt(f),
                MetaStablePoolCalls::Transfer(element) => element.fmt(f),
                MetaStablePoolCalls::TransferFrom(element) => element.fmt(f),
                MetaStablePoolCalls::UpdatePriceRateCache(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for MetaStablePoolCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            MetaStablePoolCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for MetaStablePoolCalls {
        fn from(var: AllowanceCall) -> Self {
            MetaStablePoolCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for MetaStablePoolCalls {
        fn from(var: ApproveCall) -> Self {
            MetaStablePoolCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for MetaStablePoolCalls {
        fn from(var: BalanceOfCall) -> Self {
            MetaStablePoolCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for MetaStablePoolCalls {
        fn from(var: DecimalsCall) -> Self {
            MetaStablePoolCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for MetaStablePoolCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            MetaStablePoolCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<EnableOracleCall> for MetaStablePoolCalls {
        fn from(var: EnableOracleCall) -> Self {
            MetaStablePoolCalls::EnableOracle(var)
        }
    }
    impl ::std::convert::From<GetActionIdCall> for MetaStablePoolCalls {
        fn from(var: GetActionIdCall) -> Self {
            MetaStablePoolCalls::GetActionId(var)
        }
    }
    impl ::std::convert::From<GetAmplificationParameterCall> for MetaStablePoolCalls {
        fn from(var: GetAmplificationParameterCall) -> Self {
            MetaStablePoolCalls::GetAmplificationParameter(var)
        }
    }
    impl ::std::convert::From<GetAuthorizerCall> for MetaStablePoolCalls {
        fn from(var: GetAuthorizerCall) -> Self {
            MetaStablePoolCalls::GetAuthorizer(var)
        }
    }
    impl ::std::convert::From<GetLargestSafeQueryWindowCall> for MetaStablePoolCalls {
        fn from(var: GetLargestSafeQueryWindowCall) -> Self {
            MetaStablePoolCalls::GetLargestSafeQueryWindow(var)
        }
    }
    impl ::std::convert::From<GetLastInvariantCall> for MetaStablePoolCalls {
        fn from(var: GetLastInvariantCall) -> Self {
            MetaStablePoolCalls::GetLastInvariant(var)
        }
    }
    impl ::std::convert::From<GetLatestCall> for MetaStablePoolCalls {
        fn from(var: GetLatestCall) -> Self {
            MetaStablePoolCalls::GetLatest(var)
        }
    }
    impl ::std::convert::From<GetOracleMiscDataCall> for MetaStablePoolCalls {
        fn from(var: GetOracleMiscDataCall) -> Self {
            MetaStablePoolCalls::GetOracleMiscData(var)
        }
    }
    impl ::std::convert::From<GetOwnerCall> for MetaStablePoolCalls {
        fn from(var: GetOwnerCall) -> Self {
            MetaStablePoolCalls::GetOwner(var)
        }
    }
    impl ::std::convert::From<GetPausedStateCall> for MetaStablePoolCalls {
        fn from(var: GetPausedStateCall) -> Self {
            MetaStablePoolCalls::GetPausedState(var)
        }
    }
    impl ::std::convert::From<GetPoolIdCall> for MetaStablePoolCalls {
        fn from(var: GetPoolIdCall) -> Self {
            MetaStablePoolCalls::GetPoolId(var)
        }
    }
    impl ::std::convert::From<GetPriceRateCacheCall> for MetaStablePoolCalls {
        fn from(var: GetPriceRateCacheCall) -> Self {
            MetaStablePoolCalls::GetPriceRateCache(var)
        }
    }
    impl ::std::convert::From<GetRateCall> for MetaStablePoolCalls {
        fn from(var: GetRateCall) -> Self {
            MetaStablePoolCalls::GetRate(var)
        }
    }
    impl ::std::convert::From<GetRateProvidersCall> for MetaStablePoolCalls {
        fn from(var: GetRateProvidersCall) -> Self {
            MetaStablePoolCalls::GetRateProviders(var)
        }
    }
    impl ::std::convert::From<GetSampleCall> for MetaStablePoolCalls {
        fn from(var: GetSampleCall) -> Self {
            MetaStablePoolCalls::GetSample(var)
        }
    }
    impl ::std::convert::From<GetScalingFactorsCall> for MetaStablePoolCalls {
        fn from(var: GetScalingFactorsCall) -> Self {
            MetaStablePoolCalls::GetScalingFactors(var)
        }
    }
    impl ::std::convert::From<GetSwapFeePercentageCall> for MetaStablePoolCalls {
        fn from(var: GetSwapFeePercentageCall) -> Self {
            MetaStablePoolCalls::GetSwapFeePercentage(var)
        }
    }
    impl ::std::convert::From<GetTotalSamplesCall> for MetaStablePoolCalls {
        fn from(var: GetTotalSamplesCall) -> Self {
            MetaStablePoolCalls::GetTotalSamples(var)
        }
    }
    impl ::std::convert::From<GetVaultCall> for MetaStablePoolCalls {
        fn from(var: GetVaultCall) -> Self {
            MetaStablePoolCalls::GetVault(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for MetaStablePoolCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            MetaStablePoolCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<NameCall> for MetaStablePoolCalls {
        fn from(var: NameCall) -> Self {
            MetaStablePoolCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for MetaStablePoolCalls {
        fn from(var: NoncesCall) -> Self {
            MetaStablePoolCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<OnExitPoolCall> for MetaStablePoolCalls {
        fn from(var: OnExitPoolCall) -> Self {
            MetaStablePoolCalls::OnExitPool(var)
        }
    }
    impl ::std::convert::From<OnJoinPoolCall> for MetaStablePoolCalls {
        fn from(var: OnJoinPoolCall) -> Self {
            MetaStablePoolCalls::OnJoinPool(var)
        }
    }
    impl ::std::convert::From<OnSwapWithRequestAndBalancesAndIndexInCall>
    for MetaStablePoolCalls {
        fn from(var: OnSwapWithRequestAndBalancesAndIndexInCall) -> Self {
            MetaStablePoolCalls::OnSwapWithRequestAndBalancesAndIndexIn(var)
        }
    }
    impl ::std::convert::From<OnSwapCall> for MetaStablePoolCalls {
        fn from(var: OnSwapCall) -> Self {
            MetaStablePoolCalls::OnSwap(var)
        }
    }
    impl ::std::convert::From<PermitCall> for MetaStablePoolCalls {
        fn from(var: PermitCall) -> Self {
            MetaStablePoolCalls::Permit(var)
        }
    }
    impl ::std::convert::From<QueryExitCall> for MetaStablePoolCalls {
        fn from(var: QueryExitCall) -> Self {
            MetaStablePoolCalls::QueryExit(var)
        }
    }
    impl ::std::convert::From<QueryJoinCall> for MetaStablePoolCalls {
        fn from(var: QueryJoinCall) -> Self {
            MetaStablePoolCalls::QueryJoin(var)
        }
    }
    impl ::std::convert::From<SetAssetManagerPoolConfigCall> for MetaStablePoolCalls {
        fn from(var: SetAssetManagerPoolConfigCall) -> Self {
            MetaStablePoolCalls::SetAssetManagerPoolConfig(var)
        }
    }
    impl ::std::convert::From<SetPausedCall> for MetaStablePoolCalls {
        fn from(var: SetPausedCall) -> Self {
            MetaStablePoolCalls::SetPaused(var)
        }
    }
    impl ::std::convert::From<SetPriceRateCacheDurationCall> for MetaStablePoolCalls {
        fn from(var: SetPriceRateCacheDurationCall) -> Self {
            MetaStablePoolCalls::SetPriceRateCacheDuration(var)
        }
    }
    impl ::std::convert::From<SetSwapFeePercentageCall> for MetaStablePoolCalls {
        fn from(var: SetSwapFeePercentageCall) -> Self {
            MetaStablePoolCalls::SetSwapFeePercentage(var)
        }
    }
    impl ::std::convert::From<StartAmplificationParameterUpdateCall>
    for MetaStablePoolCalls {
        fn from(var: StartAmplificationParameterUpdateCall) -> Self {
            MetaStablePoolCalls::StartAmplificationParameterUpdate(var)
        }
    }
    impl ::std::convert::From<StopAmplificationParameterUpdateCall>
    for MetaStablePoolCalls {
        fn from(var: StopAmplificationParameterUpdateCall) -> Self {
            MetaStablePoolCalls::StopAmplificationParameterUpdate(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for MetaStablePoolCalls {
        fn from(var: SymbolCall) -> Self {
            MetaStablePoolCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for MetaStablePoolCalls {
        fn from(var: TotalSupplyCall) -> Self {
            MetaStablePoolCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for MetaStablePoolCalls {
        fn from(var: TransferCall) -> Self {
            MetaStablePoolCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for MetaStablePoolCalls {
        fn from(var: TransferFromCall) -> Self {
            MetaStablePoolCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UpdatePriceRateCacheCall> for MetaStablePoolCalls {
        fn from(var: UpdatePriceRateCacheCall) -> Self {
            MetaStablePoolCalls::UpdatePriceRateCache(var)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `getActionId` function with signature `getActionId(bytes4)` and selector `0x851c1bb3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetActionIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getAmplificationParameter` function with signature `getAmplificationParameter()` and selector `0x6daccffa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetAmplificationParameterReturn {
        pub value: ::ethers::core::types::U256,
        pub is_updating: bool,
        pub precision: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetAuthorizerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getLargestSafeQueryWindow` function with signature `getLargestSafeQueryWindow()` and selector `0xffd088eb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetLargestSafeQueryWindowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLastInvariant` function with signature `getLastInvariant()` and selector `0x9b02cdde`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetLastInvariantReturn {
        pub last_invariant: ::ethers::core::types::U256,
        pub last_invariant_amp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getLatest` function with signature `getLatest(uint8)` and selector `0xb10be739`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetLatestReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getOracleMiscData` function with signature `getOracleMiscData()` and selector `0x1ed4eddc`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetOracleMiscDataReturn {
        pub log_invariant: ::ethers::core::types::I256,
        pub log_total_supply: ::ethers::core::types::I256,
        pub oracle_sample_creation_timestamp: ::ethers::core::types::U256,
        pub oracle_index: ::ethers::core::types::U256,
        pub oracle_enabled: bool,
    }
    ///Container type for all return fields from the `getOwner` function with signature `getOwner()` and selector `0x893d20e8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPausedState` function with signature `getPausedState()` and selector `0x1c0de051`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetPausedStateReturn {
        pub paused: bool,
        pub pause_window_end_time: ::ethers::core::types::U256,
        pub buffer_period_end_time: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getPoolId` function with signature `getPoolId()` and selector `0x38fff2d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetPoolIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getPriceRateCache` function with signature `getPriceRateCache(address)` and selector `0xb867ee5a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetPriceRateCacheReturn {
        pub rate: ::ethers::core::types::U256,
        pub duration: ::ethers::core::types::U256,
        pub expires: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getRate` function with signature `getRate()` and selector `0x679aefce`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRateProviders` function with signature `getRateProviders()` and selector `0x238a2d59`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetRateProvidersReturn {
        pub providers: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getSample` function with signature `getSample(uint256)` and selector `0x60d1507c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetSampleReturn {
        pub log_pair_price: ::ethers::core::types::I256,
        pub acc_log_pair_price: ::ethers::core::types::I256,
        pub log_bpt_price: ::ethers::core::types::I256,
        pub acc_log_bpt_price: ::ethers::core::types::I256,
        pub log_invariant: ::ethers::core::types::I256,
        pub acc_log_invariant: ::ethers::core::types::I256,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getScalingFactors` function with signature `getScalingFactors()` and selector `0x1dd746ea`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetScalingFactorsReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getSwapFeePercentage` function with signature `getSwapFeePercentage()` and selector `0x55c67628`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetSwapFeePercentageReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalSamples` function with signature `getTotalSamples()` and selector `0xb48b5b40`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetTotalSamplesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVault` function with signature `getVault()` and selector `0x8d928af8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetVaultReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct NameReturn(pub String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `onExitPool` function with signature `onExitPool(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0x74f3b009`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct OnExitPoolReturn {
        pub amounts_out: ::std::vec::Vec<::ethers::core::types::U256>,
        pub due_protocol_fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `onJoinPool` function with signature `onJoinPool(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0xd5c096c4`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct OnJoinPoolReturn {
        pub amounts_in: ::std::vec::Vec<::ethers::core::types::U256>,
        pub due_protocol_fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `onSwap` function with signature `onSwap((uint8,address,address,uint256,bytes32,uint256,address,address,bytes),uint256[],uint256,uint256)` and selector `0x01ec954a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct OnSwapWithRequestAndBalancesAndIndexInReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `onSwap` function with signature `onSwap((uint8,address,address,uint256,bytes32,uint256,address,address,bytes),uint256,uint256)` and selector `0x9d2c110c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct OnSwapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queryExit` function with signature `queryExit(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0x6028bfd4`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct QueryExitReturn {
        pub bpt_in: ::ethers::core::types::U256,
        pub amounts_out: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `queryJoin` function with signature `queryJoin(bytes32,address,address,uint256[],uint256,uint256,bytes)` and selector `0x87ec6817`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct QueryJoinReturn {
        pub bpt_out: ::ethers::core::types::U256,
        pub amounts_in: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct SymbolReturn(pub String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TransferFromReturn(pub bool);
    ///`SwapRequest(uint8,address,address,uint256,bytes32,uint256,address,address,bytes)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct SwapRequest {
        pub a: u8,
        pub b: ::ethers::core::types::Address,
        pub c: ::ethers::core::types::Address,
        pub d: ::ethers::core::types::U256,
        pub e: [u8; 32],
        pub f: ::ethers::core::types::U256,
        pub g: ::ethers::core::types::Address,
        pub h: ::ethers::core::types::Address,
        pub i: ::ethers::core::types::Bytes,
    }
}
