use anyhow::Result;
use clap::Parser;

mod options;
use options::{GithubOptions, LocalOptions, SharedOptions, Source};

mod github;
mod local;

fn main() -> Result<()> {
    let opts: SharedOptions = SharedOptions::parse();

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
            if version == "latest" {
                // Need to fetch the latest version
                // TODO: extract fit github mod
                let versions = github::get_versions(1)?;
                let latest = versions.last().unwrap();
                github::install(latest)?;
                return Ok(());
            }

            github::install(version.as_str())?;
        }
        Source::Github {
            option: GithubOptions::List { number },
        } => {
            github::list_versions(number)?;
        }
        Source::Npm { .. } => {
            println!("Installing pc-nrfconnect-shared from NPM.");
            todo!("Implement NPM installation.");
        }
    }
    Ok(())
}
