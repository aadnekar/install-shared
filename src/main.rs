use clap::Parser;

mod options;
use anyhow::Result;
use options::{GithubOptions, LocalOptions, SharedOptions};

use crate::options::Source;
mod github;
mod local;

fn main() -> Result<()> {
    let opts: SharedOptions = SharedOptions::parse();

    println!("{:?}", opts);

    match opts.source {
        Source::Local {
            option: LocalOptions::Install,
            generate_types,
        } => {
            return local::install(generate_types);
        }
        Source::Github {
            option: GithubOptions::Install { version },
        } => {
            println!("Installing pc-nrfconnect-shared from Github.");
            println!("Version: {}", version);
            if version == "latest" {
                // Need to fetch the latest version
                // TODO: extract fit github mod
                let versions = github::get_versions(None)?;
                let latest = versions.last().unwrap();
                github::install(latest)?;
                return Ok(());
            }

            github::install(version.as_str())?;
        }
        Source::Github {
            option: GithubOptions::ListVersions { number },
        } => {
            let versions = github::get_versions(number)?;
            let versions = versions.iter().map(|v| v.as_str()).collect();
            github::list_versions(versions)?;
        }
        Source::Npm { .. } => {
            println!("Installing pc-nrfconnect-shared from NPM.");
            todo!("Implement NPM installation.");
        }
    }
    Ok(())
}
