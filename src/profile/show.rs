use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::config::Config;
use crate::rule::ProfileRef;

pub fn show(profile_ref: ProfileRef) -> Result<()> {
    let config = Config::load()?;
    let profile = config
        .profiles
        .get(&profile_ref.name)
        .ok_or_else(|| anyhow!("Unknown profile: {}", &profile_ref.name))?;

    profile
        .configs
        .iter()
        .sorted_by_key(|(k, _)| k.to_string())
        .for_each(|(k, v)| {
            println!(r#"{} = "{}""#, k, v);
        });

    Ok(())
}
