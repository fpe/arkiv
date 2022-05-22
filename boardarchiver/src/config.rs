use std::{collections::HashMap, path::Path};

use anyhow::Context;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub boards: HashMap<String, BoardConfig>,
}

impl Config {
    pub async fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let raw_config = tokio::fs::read(path.as_ref())
            .await
            .context("failed to read config file")?;
        let config: Self =
            serde_yaml::from_slice(&raw_config).context("failed to deserialize config")?;

        Ok(config)
    }
}

fn full_media_default() -> bool {
    true
}

#[derive(Debug, Deserialize, Clone)]
pub struct BoardConfig {
    /// Save full size media files. Only saves thumbnails if set to `false`.
    ///
    /// Default: `true`
    #[serde(default = "full_media_default")]
    pub full_media: bool,
}
