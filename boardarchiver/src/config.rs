use std::{collections::HashMap, path::Path};

use anyhow::Context;
use regex::{Regex, RegexBuilder};

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

fn reverse_filter_default() -> bool {
    false
}

#[derive(Debug, Clone)]
pub struct CustomRegex(pub Regex);
impl<'de> serde::Deserialize<'de> for CustomRegex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let regex = RegexBuilder::new(&s)
            .case_insensitive(true)
            .build()
            .map_err(serde::de::Error::custom)?;

        Ok(CustomRegex(regex))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BoardConfig {
    /// Save full size media files. Only saves thumbnails if set to `false`.
    ///
    /// Default: `true`
    #[serde(default = "full_media_default")]
    pub full_media: bool,

    /// Regex Filters
    #[serde(default)]
    pub filters: Vec<CustomRegex>,

    /// Include filters and exclude everything else. Excludes filters if set to `false`.
    ///
    /// Default: `false`
    #[serde(default = "reverse_filter_default")]
    pub reverse_filter: bool,
}
