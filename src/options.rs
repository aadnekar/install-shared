use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SharedOptions {
    #[command(subcommand)]
    pub source: Source,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Source {
    #[clap(about="Installs pc-nrfconnect-shared from a local workspace defined by an environment variable called \"SharedWorkspace\"")]
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
    #[clap(about="Installs pc-nrfconnect-shared from Github: https://github.com/NordicSemiconductor/pc-nrfconnect-shared")]
    Github {
        #[command(subcommand)]
        option: GithubOptions,
    },
    #[clap(about="Not Implemented")]
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
    #[clap(about="Installs pc-nrfconnect-shared from Github: https://github.com/NordicSemiconductor/pc-nrfconnect-shared")]
    Install {
        #[clap(
            short = 'v',
            long = "version",
            default_value = "latest",
            help = "Version of pc-nrfconnect-shared to install"
        )]
        version: String,
    },
    #[clap(about="Lists versions of pc-nrfconnect-shared available on Github")]
    ListVersions {
        #[clap(short = 'n', long = "number", help = "Number of versions to list")]
        number: Option<usize>,
    },
}
