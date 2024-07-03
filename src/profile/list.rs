use anyhow::Result;
use console::style;

use crate::config::Config;

const INHERIT: &str = "(inherit)";

pub fn list(short: bool) -> Result<()> {
    let config = Config::load()?;

    config.profiles.iter().for_each(|(name, profile)| {
        if short {
            println!("{}", name)
        } else {
            println!(
                "   {} - {}: {} {}",
                style("OK").cyan(),
                style(name).bold(),
                profile
                    .configs
                    .get("user.name")
                    .map(|name| name.as_str())
                    .unwrap_or(INHERIT),
                style(&format!(
                    "<{}>",
                    profile
                        .configs
                        .get("user.email")
                        .map(|email| email.as_str())
                        .unwrap_or(INHERIT),
                ))
                .dim(),
            );
        }
    });

    Ok(())
}
