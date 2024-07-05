use anyhow::{anyhow, Result};
use console::style;
use git2::Repository;
use log::info;
use url::Url;

use crate::config::Config;
use crate::rule::ProfileRef;
use crate::url::Url as RepoUrl;

const INHERIT: &str = "(inherit)";

pub fn apply(profile_name: Option<String>) -> Result<()> {
    match profile_name {
        Some(name) => {
            let profile_ref = ProfileRef { name };
            let config = Config::load()?;
            let profile = config
                .profiles
                .get(&profile_ref.name)
                .ok_or_else(|| anyhow!("Unknown profile: {}", &profile_ref.name))?;

            profile.apply()?;

            info!(
                "Attached profile [{}] successfully.",
                style(profile_ref.name).bold()
            );
        }
        None => {
            let repo = Repository::open_from_env();
            let repo = match repo {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("   {} - {}", style("ERROR").red(), style("Not a git repository!").bold());
                    return Ok(());
                }
            };

            let remote = repo.find_remote("origin")?;
            let url = RepoUrl::from_url(&Url::parse(remote.url().expect("No remote url"))?)?;

            let config = Config::load()?;
            let rule = config.rules.resolve(&url);
            match rule {
                None => {
                    info!("No profile found for [{}].", style(&url).bold());
                    return Ok(());
                }
                Some(rule) => {
                    let profile = config
                        .profiles
                        .resolve(&rule.profile)
                        .expect("No profile found");
                    profile.1.apply()?;
                    info!(
                        "Attached profile [{}] successfully.",
                        style(profile.0).bold()
                    );
                    println!(
                        "   {} - {}: {} {}",
                        style("OK").cyan(),
                        style(profile.0).bold(),
                        profile
                            .1
                            .configs
                            .get("user.name")
                            .map(|name| name.as_str())
                            .unwrap_or(INHERIT),
                        style(&format!(
                            "<{}>",
                            profile
                                .1
                                .configs
                                .get("user.email")
                                .map(|email| email.as_str())
                                .unwrap_or(INHERIT),
                        ))
                        .dim(),
                    );
                }
            }
        }
    }

    Ok(())
}
