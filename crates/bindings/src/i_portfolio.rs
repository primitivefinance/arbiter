pub use i_portfolio::*;
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
pub mod i_portfolio {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaLiquidity\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Allocate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ChangeParameters\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ClaimFees\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"decimalsAsset\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"decimalsQuote\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CreatePair\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"controller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"maxPrice\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"duration\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"volatility\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CreatePool\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deltaLiquidity\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deallocate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DecreaseReserveBalance\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deposit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IncreaseReserveBalance\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"input\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"output\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feeAmountDec\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"invariantWad\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Swap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prevFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdateProtocolFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REGISTRY\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"VERSION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"useMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaLiquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"maxDeltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"maxDeltaQuote\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"allocate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeParameters\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"createPair\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"controller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"volatility\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"duration\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"maxPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"price\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"createPool\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"useMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaLiquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"minDeltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"minDeltaQuote\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deallocate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"sellAsset\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"swapper\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOut\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"deltaLiquidity\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLiquidityDeltas\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"deltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaQuote\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxLiquidity\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"deltaLiquidity\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNetBalance\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPairId\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPairNonce\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"pairNonce\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"deltaAsset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deltaQuote\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVirtualPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVirtualReservesDec\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"deltaAsset\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"deltaQuote\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"pairId\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pairs\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"tokenAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsAsset\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenQuote\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsQuote\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pools\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"virtualX\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"virtualY\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"lastTimestamp\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"controller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct PortfolioCurve\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"maxPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"jit\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"fee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"duration\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"volatility\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"priorityFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"createdAt\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"perpetual\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"struct PortfolioPair\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsAsset\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenQuote\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimalsQuote\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"positions\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"freeLiquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"lastTimestamp\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setProtocolFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct Order\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bool\",\"name\":\"useMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"input\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"output\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"sellAsset\",\"type\":\"bool\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"poolId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"input\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"output\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IPORTFOLIO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IPortfolio<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPortfolio<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPortfolio<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPortfolio<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPortfolio<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IPortfolio)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPortfolio<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPORTFOLIO_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `REGISTRY` (0x06433b1b) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([6, 67, 59, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocate` (0x94c32b4a) function
        pub fn allocate(
            &self,
            use_max: bool,
            pool_id: u64,
            delta_liquidity: u128,
            max_delta_asset: u128,
            max_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [148, 195, 43, 74],
                    (use_max, pool_id, delta_liquidity, max_delta_asset, max_delta_quote),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeParameters` (0xaf777855) function
        pub fn change_parameters(
            &self,
            pool_id: u64,
            priority_fee: u16,
            fee: u16,
            jit: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 119, 120, 85], (pool_id, priority_fee, fee, jit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimFee` (0xdda40797) function
        pub fn claim_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 164, 7, 151], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPair` (0xc9c65396) function
        pub fn create_pair(
            &self,
            asset: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([201, 198, 83, 150], (asset, quote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPool` (0x40b54e8e) function
        pub fn create_pool(
            &self,
            pair_id: u32,
            controller: ::ethers::core::types::Address,
            priority_fee: u16,
            fee: u16,
            volatility: u16,
            duration: u16,
            jit: u16,
            max_price: u128,
            price: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash(
                    [64, 181, 78, 142],
                    (
                        pair_id,
                        controller,
                        priority_fee,
                        fee,
                        volatility,
                        duration,
                        jit,
                        max_price,
                        price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0x5bc55464) function
        pub fn deallocate(
            &self,
            use_max: bool,
            pool_id: u64,
            delta_liquidity: u128,
            min_delta_asset: u128,
            min_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [91, 197, 84, 100],
                    (use_max, pool_id, delta_liquidity, min_delta_asset, min_delta_quote),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x19057807) function
        pub fn get_amount_out(
            &self,
            pool_id: u64,
            sell_asset: bool,
            amount_in: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 5, 120, 7], (pool_id, sell_asset, amount_in, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityDeltas` (0x8992f20a) function
        pub fn get_liquidity_deltas(
            &self,
            pool_id: u64,
            delta_liquidity: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([137, 146, 242, 10], (pool_id, delta_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxLiquidity` (0xd6b7dec5) function
        pub fn get_max_liquidity(
            &self,
            pool_id: u64,
            delta_asset: ::ethers::core::types::U256,
            delta_quote: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([214, 183, 222, 197], (pool_id, delta_asset, delta_quote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNetBalance` (0x4dc68a90) function
        pub fn get_net_balance(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([77, 198, 138, 144], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairId` (0x3f92a339) function
        pub fn get_pair_id(
            &self,
            asset: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([63, 146, 163, 57], (asset, quote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairNonce` (0x078888d6) function
        pub fn get_pair_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([7, 136, 136, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolNonce` (0xa5cd8a49) function
        pub fn get_pool_nonce(
            &self,
            pair_nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([165, 205, 138, 73], pair_nonce)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolReserves` (0x2afb9df8) function
        pub fn get_pool_reserves(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([42, 251, 157, 248], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserve` (0xc9a396e9) function
        pub fn get_reserve(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 163, 150, 233], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVirtualPrice` (0x61b7ea6a) function
        pub fn get_virtual_price(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 183, 234, 106], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVirtualReservesDec` (0xa21b9ba0) function
        pub fn get_virtual_reserves_dec(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([162, 27, 155, 160], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairs` (0x5e47663c) function
        pub fn pairs(
            &self,
            pair_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u8, ::ethers::core::types::Address, u8),
        > {
            self.0
                .method_hash([94, 71, 102, 60], pair_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0x89a5f084) function
        pub fn pools(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                u128,
                u128,
                u32,
                ::ethers::core::types::Address,
                PortfolioCurve,
                PortfolioPair,
            ),
        > {
            self.0
                .method_hash([137, 165, 240, 132], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positions` (0xb68513ea) function
        pub fn positions(
            &self,
            owner: ::ethers::core::types::Address,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u32)> {
            self.0
                .method_hash([182, 133, 19, 234], (owner, pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFee` (0x787dce3d) function
        pub fn set_protocol_fee(
            &self,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 125, 206, 61], fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x64f14ef2) function
        pub fn swap(
            &self,
            args: Order,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u64, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([100, 241, 78, 242], (args,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeParameters` event
        pub fn change_parameters_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeParametersFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ClaimFees` event
        pub fn claim_fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClaimFeesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CreatePair` event
        pub fn create_pair_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CreatePairFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CreatePool` event
        pub fn create_pool_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CreatePoolFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeallocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DecreaseReserveBalance` event
        pub fn decrease_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DecreaseReserveBalanceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncreaseReserveBalance` event
        pub fn increase_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IncreaseReserveBalanceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `UpdateProtocolFee` event
        pub fn update_protocol_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateProtocolFeeFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IPortfolioEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPortfolio<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Allocate",
        abi = "Allocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ChangeParameters",
        abi = "ChangeParameters(uint64,uint16,uint16,uint16)"
    )]
    pub struct ChangeParametersFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub priority_fee: u16,
        #[ethevent(indexed)]
        pub fee: u16,
        pub jit: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ClaimFees", abi = "ClaimFees(address,uint256)")]
    pub struct ClaimFeesFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CreatePair",
        abi = "CreatePair(uint24,address,address,uint8,uint8)"
    )]
    pub struct CreatePairFilter {
        #[ethevent(indexed)]
        pub pair_id: u32,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub decimals_quote: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CreatePool",
        abi = "CreatePool(uint64,address,address,address,uint128,uint16,uint16,uint16,uint16,uint16)"
    )]
    pub struct CreatePoolFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub controller: ::ethers::core::types::Address,
        pub max_price: u128,
        pub jit: u16,
        pub fee: u16,
        pub duration: u16,
        pub volatility: u16,
        pub priority_fee: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Deallocate",
        abi = "Deallocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DecreaseReserveBalance",
        abi = "DecreaseReserveBalance(address,uint256)"
    )]
    pub struct DecreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "IncreaseReserveBalance",
        abi = "IncreaseReserveBalance(address,uint256)"
    )]
    pub struct IncreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(uint64,uint256,address,uint256,address,uint256,uint256,int256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        pub price: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        pub input: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub output: ::ethers::core::types::U256,
        pub fee_amount_dec: ::ethers::core::types::U256,
        pub invariant_wad: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "UpdateProtocolFee", abi = "UpdateProtocolFee(uint256,uint256)")]
    pub struct UpdateProtocolFeeFilter {
        pub prev_fee: ::ethers::core::types::U256,
        pub next_fee: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPortfolioEvents {
        AllocateFilter(AllocateFilter),
        ChangeParametersFilter(ChangeParametersFilter),
        ClaimFeesFilter(ClaimFeesFilter),
        CreatePairFilter(CreatePairFilter),
        CreatePoolFilter(CreatePoolFilter),
        DeallocateFilter(DeallocateFilter),
        DecreaseReserveBalanceFilter(DecreaseReserveBalanceFilter),
        DepositFilter(DepositFilter),
        IncreaseReserveBalanceFilter(IncreaseReserveBalanceFilter),
        SwapFilter(SwapFilter),
        UpdateProtocolFeeFilter(UpdateProtocolFeeFilter),
    }
    impl ::ethers::contract::EthLogDecode for IPortfolioEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(IPortfolioEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = ChangeParametersFilter::decode_log(log) {
                return Ok(IPortfolioEvents::ChangeParametersFilter(decoded));
            }
            if let Ok(decoded) = ClaimFeesFilter::decode_log(log) {
                return Ok(IPortfolioEvents::ClaimFeesFilter(decoded));
            }
            if let Ok(decoded) = CreatePairFilter::decode_log(log) {
                return Ok(IPortfolioEvents::CreatePairFilter(decoded));
            }
            if let Ok(decoded) = CreatePoolFilter::decode_log(log) {
                return Ok(IPortfolioEvents::CreatePoolFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(IPortfolioEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = DecreaseReserveBalanceFilter::decode_log(log) {
                return Ok(IPortfolioEvents::DecreaseReserveBalanceFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(IPortfolioEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = IncreaseReserveBalanceFilter::decode_log(log) {
                return Ok(IPortfolioEvents::IncreaseReserveBalanceFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IPortfolioEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = UpdateProtocolFeeFilter::decode_log(log) {
                return Ok(IPortfolioEvents::UpdateProtocolFeeFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IPortfolioEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeParametersFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePairFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePoolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProtocolFeeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for IPortfolioEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<ChangeParametersFilter> for IPortfolioEvents {
        fn from(value: ChangeParametersFilter) -> Self {
            Self::ChangeParametersFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFeesFilter> for IPortfolioEvents {
        fn from(value: ClaimFeesFilter) -> Self {
            Self::ClaimFeesFilter(value)
        }
    }
    impl ::core::convert::From<CreatePairFilter> for IPortfolioEvents {
        fn from(value: CreatePairFilter) -> Self {
            Self::CreatePairFilter(value)
        }
    }
    impl ::core::convert::From<CreatePoolFilter> for IPortfolioEvents {
        fn from(value: CreatePoolFilter) -> Self {
            Self::CreatePoolFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for IPortfolioEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseReserveBalanceFilter> for IPortfolioEvents {
        fn from(value: DecreaseReserveBalanceFilter) -> Self {
            Self::DecreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for IPortfolioEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseReserveBalanceFilter> for IPortfolioEvents {
        fn from(value: IncreaseReserveBalanceFilter) -> Self {
            Self::IncreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for IPortfolioEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<UpdateProtocolFeeFilter> for IPortfolioEvents {
        fn from(value: UpdateProtocolFeeFilter) -> Self {
            Self::UpdateProtocolFeeFilter(value)
        }
    }
    ///Container type for all input parameters for the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
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
    #[ethcall(name = "REGISTRY", abi = "REGISTRY()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `allocate` function with signature `allocate(bool,uint64,uint128,uint128,uint128)` and selector `0x94c32b4a`
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
    #[ethcall(name = "allocate", abi = "allocate(bool,uint64,uint128,uint128,uint128)")]
    pub struct AllocateCall {
        pub use_max: bool,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub max_delta_asset: u128,
        pub max_delta_quote: u128,
    }
    ///Container type for all input parameters for the `changeParameters` function with signature `changeParameters(uint64,uint16,uint16,uint16)` and selector `0xaf777855`
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
        name = "changeParameters",
        abi = "changeParameters(uint64,uint16,uint16,uint16)"
    )]
    pub struct ChangeParametersCall {
        pub pool_id: u64,
        pub priority_fee: u16,
        pub fee: u16,
        pub jit: u16,
    }
    ///Container type for all input parameters for the `claimFee` function with signature `claimFee(address,uint256)` and selector `0xdda40797`
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
    #[ethcall(name = "claimFee", abi = "claimFee(address,uint256)")]
    pub struct ClaimFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
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
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub asset: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(uint24,address,uint16,uint16,uint16,uint16,uint16,uint128,uint128)` and selector `0x40b54e8e`
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
        name = "createPool",
        abi = "createPool(uint24,address,uint16,uint16,uint16,uint16,uint16,uint128,uint128)"
    )]
    pub struct CreatePoolCall {
        pub pair_id: u32,
        pub controller: ::ethers::core::types::Address,
        pub priority_fee: u16,
        pub fee: u16,
        pub volatility: u16,
        pub duration: u16,
        pub jit: u16,
        pub max_price: u128,
        pub price: u128,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
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
        name = "deallocate",
        abi = "deallocate(bool,uint64,uint128,uint128,uint128)"
    )]
    pub struct DeallocateCall {
        pub use_max: bool,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub min_delta_asset: u128,
        pub min_delta_quote: u128,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint64,bool,uint256,address)")]
    pub struct GetAmountOutCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
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
    #[ethcall(name = "getLiquidityDeltas", abi = "getLiquidityDeltas(uint64,int128)")]
    pub struct GetLiquidityDeltasCall {
        pub pool_id: u64,
        pub delta_liquidity: i128,
    }
    ///Container type for all input parameters for the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
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
    #[ethcall(name = "getMaxLiquidity", abi = "getMaxLiquidity(uint64,uint256,uint256)")]
    pub struct GetMaxLiquidityCall {
        pub pool_id: u64,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
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
    #[ethcall(name = "getNetBalance", abi = "getNetBalance(address)")]
    pub struct GetNetBalanceCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
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
    #[ethcall(name = "getPairId", abi = "getPairId(address,address)")]
    pub struct GetPairIdCall {
        pub asset: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
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
    #[ethcall(name = "getPairNonce", abi = "getPairNonce()")]
    pub struct GetPairNonceCall;
    ///Container type for all input parameters for the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
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
    #[ethcall(name = "getPoolNonce", abi = "getPoolNonce(uint24)")]
    pub struct GetPoolNonceCall {
        pub pair_nonce: u32,
    }
    ///Container type for all input parameters for the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
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
    #[ethcall(name = "getPoolReserves", abi = "getPoolReserves(uint64)")]
    pub struct GetPoolReservesCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
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
    #[ethcall(name = "getReserve", abi = "getReserve(address)")]
    pub struct GetReserveCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getVirtualPrice` function with signature `getVirtualPrice(uint64)` and selector `0x61b7ea6a`
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
    #[ethcall(name = "getVirtualPrice", abi = "getVirtualPrice(uint64)")]
    pub struct GetVirtualPriceCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getVirtualReservesDec` function with signature `getVirtualReservesDec(uint64)` and selector `0xa21b9ba0`
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
    #[ethcall(name = "getVirtualReservesDec", abi = "getVirtualReservesDec(uint64)")]
    pub struct GetVirtualReservesDecCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
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
    #[ethcall(name = "pairs", abi = "pairs(uint24)")]
    pub struct PairsCall {
        pub pair_id: u32,
    }
    ///Container type for all input parameters for the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
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
    #[ethcall(name = "pools", abi = "pools(uint64)")]
    pub struct PoolsCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `positions` function with signature `positions(address,uint64)` and selector `0xb68513ea`
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
    #[ethcall(name = "positions", abi = "positions(address,uint64)")]
    pub struct PositionsCall {
        pub owner: ::ethers::core::types::Address,
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `setProtocolFee` function with signature `setProtocolFee(uint256)` and selector `0x787dce3d`
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
    #[ethcall(name = "setProtocolFee", abi = "setProtocolFee(uint256)")]
    pub struct SetProtocolFeeCall {
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap((bool,uint64,uint128,uint128,bool))` and selector `0x64f14ef2`
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
    #[ethcall(name = "swap", abi = "swap((bool,uint64,uint128,uint128,bool))")]
    pub struct SwapCall {
        pub args: Order,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPortfolioCalls {
        Registry(RegistryCall),
        Version(VersionCall),
        Weth(WethCall),
        Allocate(AllocateCall),
        ChangeParameters(ChangeParametersCall),
        ClaimFee(ClaimFeeCall),
        CreatePair(CreatePairCall),
        CreatePool(CreatePoolCall),
        Deallocate(DeallocateCall),
        GetAmountOut(GetAmountOutCall),
        GetLiquidityDeltas(GetLiquidityDeltasCall),
        GetMaxLiquidity(GetMaxLiquidityCall),
        GetNetBalance(GetNetBalanceCall),
        GetPairId(GetPairIdCall),
        GetPairNonce(GetPairNonceCall),
        GetPoolNonce(GetPoolNonceCall),
        GetPoolReserves(GetPoolReservesCall),
        GetReserve(GetReserveCall),
        GetVirtualPrice(GetVirtualPriceCall),
        GetVirtualReservesDec(GetVirtualReservesDecCall),
        Multicall(MulticallCall),
        Pairs(PairsCall),
        Pools(PoolsCall),
        Positions(PositionsCall),
        SetProtocolFee(SetProtocolFeeCall),
        Swap(SwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPortfolioCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Registry(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded)
                = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded)
                = <ChangeParametersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ChangeParameters(decoded));
            }
            if let Ok(decoded)
                = <ClaimFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimFee(decoded));
            }
            if let Ok(decoded)
                = <CreatePairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePair(decoded));
            }
            if let Ok(decoded)
                = <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePool(decoded));
            }
            if let Ok(decoded)
                = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded)
                = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded)
                = <GetLiquidityDeltasCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLiquidityDeltas(decoded));
            }
            if let Ok(decoded)
                = <GetMaxLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxLiquidity(decoded));
            }
            if let Ok(decoded)
                = <GetNetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNetBalance(decoded));
            }
            if let Ok(decoded)
                = <GetPairIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPairId(decoded));
            }
            if let Ok(decoded)
                = <GetPairNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPairNonce(decoded));
            }
            if let Ok(decoded)
                = <GetPoolNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPoolNonce(decoded));
            }
            if let Ok(decoded)
                = <GetPoolReservesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPoolReserves(decoded));
            }
            if let Ok(decoded)
                = <GetReserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserve(decoded));
            }
            if let Ok(decoded)
                = <GetVirtualPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVirtualPrice(decoded));
            }
            if let Ok(decoded)
                = <GetVirtualReservesDecCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVirtualReservesDec(decoded));
            }
            if let Ok(decoded)
                = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded)
                = <PairsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pairs(decoded));
            }
            if let Ok(decoded)
                = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded)
                = <PositionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Positions(decoded));
            }
            if let Ok(decoded)
                = <SetProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPortfolioCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Registry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeParameters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePair(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidityDeltas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNetBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPairId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPairNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVirtualPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVirtualReservesDec(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pairs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IPortfolioCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Registry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeParameters(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePair(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidityDeltas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMaxLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVirtualPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVirtualReservesDec(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RegistryCall> for IPortfolioCalls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<VersionCall> for IPortfolioCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WethCall> for IPortfolioCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<AllocateCall> for IPortfolioCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<ChangeParametersCall> for IPortfolioCalls {
        fn from(value: ChangeParametersCall) -> Self {
            Self::ChangeParameters(value)
        }
    }
    impl ::core::convert::From<ClaimFeeCall> for IPortfolioCalls {
        fn from(value: ClaimFeeCall) -> Self {
            Self::ClaimFee(value)
        }
    }
    impl ::core::convert::From<CreatePairCall> for IPortfolioCalls {
        fn from(value: CreatePairCall) -> Self {
            Self::CreatePair(value)
        }
    }
    impl ::core::convert::From<CreatePoolCall> for IPortfolioCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for IPortfolioCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for IPortfolioCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetLiquidityDeltasCall> for IPortfolioCalls {
        fn from(value: GetLiquidityDeltasCall) -> Self {
            Self::GetLiquidityDeltas(value)
        }
    }
    impl ::core::convert::From<GetMaxLiquidityCall> for IPortfolioCalls {
        fn from(value: GetMaxLiquidityCall) -> Self {
            Self::GetMaxLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNetBalanceCall> for IPortfolioCalls {
        fn from(value: GetNetBalanceCall) -> Self {
            Self::GetNetBalance(value)
        }
    }
    impl ::core::convert::From<GetPairIdCall> for IPortfolioCalls {
        fn from(value: GetPairIdCall) -> Self {
            Self::GetPairId(value)
        }
    }
    impl ::core::convert::From<GetPairNonceCall> for IPortfolioCalls {
        fn from(value: GetPairNonceCall) -> Self {
            Self::GetPairNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolNonceCall> for IPortfolioCalls {
        fn from(value: GetPoolNonceCall) -> Self {
            Self::GetPoolNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolReservesCall> for IPortfolioCalls {
        fn from(value: GetPoolReservesCall) -> Self {
            Self::GetPoolReserves(value)
        }
    }
    impl ::core::convert::From<GetReserveCall> for IPortfolioCalls {
        fn from(value: GetReserveCall) -> Self {
            Self::GetReserve(value)
        }
    }
    impl ::core::convert::From<GetVirtualPriceCall> for IPortfolioCalls {
        fn from(value: GetVirtualPriceCall) -> Self {
            Self::GetVirtualPrice(value)
        }
    }
    impl ::core::convert::From<GetVirtualReservesDecCall> for IPortfolioCalls {
        fn from(value: GetVirtualReservesDecCall) -> Self {
            Self::GetVirtualReservesDec(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for IPortfolioCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<PairsCall> for IPortfolioCalls {
        fn from(value: PairsCall) -> Self {
            Self::Pairs(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for IPortfolioCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for IPortfolioCalls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeCall> for IPortfolioCalls {
        fn from(value: SetProtocolFeeCall) -> Self {
            Self::SetProtocolFee(value)
        }
    }
    impl ::core::convert::From<SwapCall> for IPortfolioCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    ///Container type for all return fields from the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
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
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `allocate` function with signature `allocate(bool,uint64,uint128,uint128,uint128)` and selector `0x94c32b4a`
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
    pub struct AllocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
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
    pub struct CreatePairReturn {
        pub pair_id: u32,
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(uint24,address,uint16,uint16,uint16,uint16,uint16,uint128,uint128)` and selector `0x40b54e8e`
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
    pub struct CreatePoolReturn {
        pub pool_id: u64,
    }
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
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
    pub struct DeallocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
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
    pub struct GetAmountOutReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
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
    pub struct GetLiquidityDeltasReturn {
        pub delta_asset: u128,
        pub delta_quote: u128,
    }
    ///Container type for all return fields from the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
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
    pub struct GetMaxLiquidityReturn {
        pub delta_liquidity: u128,
    }
    ///Container type for all return fields from the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
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
    pub struct GetNetBalanceReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
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
    pub struct GetPairIdReturn {
        pub pair_id: u32,
    }
    ///Container type for all return fields from the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
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
    pub struct GetPairNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
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
    pub struct GetPoolNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
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
    pub struct GetPoolReservesReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
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
    pub struct GetReserveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVirtualPrice` function with signature `getVirtualPrice(uint64)` and selector `0x61b7ea6a`
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
    pub struct GetVirtualPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getVirtualReservesDec` function with signature `getVirtualReservesDec(uint64)` and selector `0xa21b9ba0`
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
    pub struct GetVirtualReservesDecReturn {
        pub delta_asset: u128,
        pub delta_quote: u128,
    }
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
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
    pub struct PairsReturn {
        pub token_asset: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub token_quote: ::ethers::core::types::Address,
        pub decimals_quote: u8,
    }
    ///Container type for all return fields from the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
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
    pub struct PoolsReturn {
        pub virtual_x: u128,
        pub virtual_y: u128,
        pub liquidity: u128,
        pub last_timestamp: u32,
        pub controller: ::ethers::core::types::Address,
        pub p5: PortfolioCurve,
        pub p6: PortfolioPair,
    }
    ///Container type for all return fields from the `positions` function with signature `positions(address,uint64)` and selector `0xb68513ea`
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
    pub struct PositionsReturn {
        pub free_liquidity: u128,
        pub last_timestamp: u32,
    }
    ///Container type for all return fields from the `swap` function with signature `swap((bool,uint64,uint128,uint128,bool))` and selector `0x64f14ef2`
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
    pub struct SwapReturn {
        pub pool_id: u64,
        pub input: ::ethers::core::types::U256,
        pub output: ::ethers::core::types::U256,
    }
}
