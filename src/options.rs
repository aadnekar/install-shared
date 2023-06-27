use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct SharedOptions {
    
    #[clap(short = 'S', long = 'skip-types')]
    pub skip_types: Option<PathBuf>,
}
