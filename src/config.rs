use std::fs::read_to_string;
use std::path::Path;

use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::profile::Profiles;
use crate::rule::Rules;
use crate::url::Patterns;

#[derive(Debug, Default, Deserialize)]
pub struct Defaults {
    pub owner: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub patterns: Patterns,
    #[serde(default)]
    pub profiles: Profiles,
    #[serde(default)]
    pub rules: Rules,
}

impl Config {
    pub(crate) fn load() -> Result<Self> {
        let path = dirs::home_dir()
            .ok_or_else(|| anyhow!("Cannot find home directory"))?
            .join(".config/profile/profile.toml");
        Ok(Self::load_from_path(path)?.unwrap_or_default())
    }
    fn load_from_path<P>(path: P) -> Result<Option<Self>>
    where
        P: AsRef<Path>,
    {
        Ok(match path.as_ref().exists() {
            true => Some(Self::load_from_str(read_to_string(path)?.as_str())?),
            _ => None,
        })
    }

    fn load_from_str(s: &str) -> Result<Self> {
        Ok(toml::from_str::<Self>(s)?.with_defaults())
    }

    fn with_defaults(mut self) -> Self {
        self.patterns = self.patterns.with_defaults();
        self
    }
}
