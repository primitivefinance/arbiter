use super::*;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct ArbiterConfig {
    /// The path to the directory where the bindings will be generated.
    pub bindings_path: PathBuf,
    /// Whether to generate bindings for submodules.
    pub submodules: bool,
    /// Ignore interfaces flag
    pub ignore_interfaces: bool,
}

impl ArbiterConfig {
    pub fn _new_mock_config() -> Self {
        ArbiterConfig {
            bindings_path: PathBuf::from("src").join("bindings"),
            submodules: false,
            ignore_interfaces: false,
        }
    }
}

impl ArbiterConfig {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("abiter.toml"))
            .build()?;
        s.try_deserialize()
        // Try loading the TOML content from the file
    }
}
