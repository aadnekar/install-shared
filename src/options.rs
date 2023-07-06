use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "Install Shared")]
#[command(author = "Ådne Karstad <aadne.karstad@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Installs pc-nrfconnect-shared", long_about = None)]
#[clap()]
pub struct SharedOptions {
    #[command(subcommand)]
    pub source: Source,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Source {
    Local {
        #[clap(
            short = 't',
            long = "types",
            default_value = "false",
            help = "Generates types using 'npm run generate-types'"
        )]
        generate_types: bool,
        option: LocalOptions,
    },
    Github {
        #[command(subcommand)]
        option: GithubOptions,
    },
    Npm {
        option: (),
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum LocalOptions {
    Install,
}

#[derive(Clone, Debug, Subcommand)]
pub enum GithubOptions {
    Install {
        #[clap(
            short = 'v',
            long = "version",
            default_value = "latest",
            help = "Version of pc-nrfconnect-shared to install"
        )]
        version: String,
    },
    ListVersions {
        #[clap(short = 'n', long = "number", help = "Number of versions to list")]
        number: Option<usize>,
    },
}
