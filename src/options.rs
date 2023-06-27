use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct SharedOptions {
    
    #[clap(short = 't', long = "types", default_value = "false")]
    pub skip_types: bool,
}
