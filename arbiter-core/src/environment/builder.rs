//! This module provides all the necessary structures for creating an
//! [`Environment`]. This includes the [`EnvironmentBuilder`] and
//! [`EnvironmentParameters`] structures as well as the [`BlockSettings`] and
//! [`GasSettings`] enums.

#![warn(missing_docs)]

use super::*;

/// Parameters necessary for creating or modifying an `Environment`.
///
/// This structure holds configuration details or other parameters that might
/// be required when instantiating or updating an `Environment`.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct EnvironmentParameters {
    /// A label for the [`Environment`].
    /// Used to allow the [`Manager`] to locate the [`Environment`] in order to
    /// control it. Also used to be able to organize, track progress, and
    /// post-process results.
    pub label: Option<String>,

    /// The type of block that will be used to step forward the [`EVM`].
    /// This can either be a [`BlockType::UserControlled`] or a
    /// [`BlockType::RandomlySampled`].
    /// The former will allow the end user to control the block number from
    /// their own external API and the latter will allow the end user to set
    /// a rate parameter and seed for a Poisson distribution that will be
    /// used to sample the amount of transactions per block.
    pub block_settings: BlockSettings,

    /// The gas settings for the [`Environment`].
    /// This can either be [`GasSettings::UserControlled`],
    /// [`GasSettings::RandomlySampled`], or [`GasSettings::Constant`].
    /// The first will allow the end user to control the gas price from
    /// their own external API (not yet implemented) and the second will allow
    /// the end user to set a multiplier for the gas price that will be used
    /// to sample the amount of transactions per block. The last will allow
    /// the end user to set a constant gas price for all transactions.
    /// By default, [`GasSettings::UserControlled`] begins with a gas price of
    /// 0.
    pub gas_settings: GasSettings,
}

/// A builder for creating an `Environment`.
///
/// This builder allows for the configuration of an `Environment` before it is
/// instantiated. It provides methods for setting the label, block settings, and
/// gas settings of the `Environment`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnvironmentBuilder {
    /// An optional label for the `Environment`.
    /// This label is used to allow the [`Manager`] to locate the
    /// [`Environment`] in order to control it. It is also used for
    /// organizing, tracking progress, and post-processing results.
    pub label: Option<String>,

    /// The type of block that will be used to step forward the [`EVM`].
    /// This can either be a [`BlockType::UserControlled`] or a
    /// [`BlockType::RandomlySampled`].
    /// The former allows the end user to control the block number from
    /// their own external API and the latter allows the end user to set
    /// a rate parameter and seed for a Poisson distribution that will be
    /// used to sample the amount of transactions per block.
    pub block_settings: BlockSettings,

    /// The gas settings for the `Environment`.
    /// This can either be [`GasSettings::UserControlled`],
    /// [`GasSettings::RandomlySampled`], or [`GasSettings::Constant`].
    /// The first allows the end user to control the gas price from
    /// their own external API (not yet implemented) and the second allows
    /// the end user to set a multiplier for the gas price that will be used
    /// to sample the amount of transactions per block. The last allows
    /// the end user to set a constant gas price for all transactions.
    /// By default, [`GasSettings::UserControlled`] begins with a gas price of
    /// 0.
    pub gas_settings: GasSettings,
}

/// The `EnvironmentBuilder` is a builder pattern for creating an
/// [`Environment`]. It allows for the configuration of the [`Environment`]
/// before it is created.
impl EnvironmentBuilder {
    /// Creates a new `EnvironmentBuilder` with default settings.
    /// By default, the `block_settings` and `gas_settings` are set to
    /// `UserControlled`.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            label: None,
            block_settings: BlockSettings::UserControlled,
            gas_settings: GasSettings::UserControlled,
        }
    }

    /// Sets the `block_settings` for the `EnvironmentBuilder`.
    /// This determines how the block number and timestamp are controlled in the
    /// [`Environment`].
    pub fn block_settings(mut self, block_settings: BlockSettings) -> Self {
        self.block_settings = block_settings;
        self
    }

    /// Sets the `gas_settings` for the `EnvironmentBuilder`.
    /// This determines how the gas price is controlled in the [`Environment`].
    pub fn gas_settings(mut self, gas_settings: GasSettings) -> Self {
        self.gas_settings = gas_settings;
        self
    }

    /// Sets the `label` for the `EnvironmentBuilder`.
    /// This is an optional string that can be used to identify the
    /// [`Environment`].
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Converts the `EnvironmentBuilder` into `EnvironmentParameters`.
    /// This is a private function used in the `build` function.
    fn into_environment_parameters(self) -> EnvironmentParameters {
        EnvironmentParameters {
            label: self.label.clone(),
            block_settings: self.block_settings.clone(),
            gas_settings: self.gas_settings.clone(),
        }
    }

    /// Builds the `Environment` from the `EnvironmentBuilder`.
    /// This consumes the `EnvironmentBuilder` and returns an [`Environment`].
    pub fn build(self) -> Environment {
        let parameters = self.into_environment_parameters();
        let mut env = Environment::new(parameters);
        env.run();
        env
    }
}

/// Provides a means of deciding how the block number of the [`EVM`] will be
/// chosen.
/// This can either be a [`BlockType::UserControlled`] or a
/// [`BlockType::RandomlySampled`].
/// The former will allow the end user to control the block number from
/// their own external API and the latter will allow the end user to set
/// a rate parameter and seed for a Poisson distribution that will be
/// used to sample the amount of transactions per block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BlockSettings {
    /// The block number will be controlled by the end user.
    #[default]
    UserControlled,

    /// The block number will be sampled from a Poisson distribution.
    /// A seeded Poisson distribution that is sampled from in order to determine
    /// the average block size. [`SeededPoisson`] is created with a seed in
    /// order to have repeatable simulations.
    RandomlySampled {
        /// The mean of the rate at which the environment will
        /// process blocks (e.g., the rate parameter in the Poisson distribution
        /// used in the [`SeededPoisson`] field of an [`Environment`]).
        block_rate: f64,

        /// The amount of time the block timestamp will increase for each new
        /// block.
        block_time: u32,

        /// A value chosen to generate randomly chosen block sizes
        /// for the environment.
        seed: u64,
    },
}

/// Provides a means of deciding how the gas price of the
/// [`EVM`] will be chosen.
/// This can either be a [`GasSettings::UserControlled`],
/// [`GasSettings::RandomlySampled`], or [`GasSettings::None`].
/// The former will allow the end user to control the gas price from
/// their own external API and the latter will allow the end user to set
/// a constant gas price.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GasSettings {
    /// The gas limit will be controlled by the end user.
    /// In the future, Foundry cheatcodes will be used to control gas
    /// on-the-fly.
    #[default]
    UserControlled,

    /// The gas price will depend on the number of transactions in the block.
    /// The user *must* set the [`BlockType`] to [`BlockType::RandomlySampled`].
    /// We determine the gas price by multiplying the number of transactions in
    /// the block by the multiplier which represents paying higher fees for a
    /// more congested network.
    RandomlySampled {
        /// Multiplies the number of transactions in the block to determine the
        /// gas price.
        multiplier: f64,
    },

    /// The gas price will be a constant value from the inner value.
    Constant(u128),
}
