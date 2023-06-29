use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct SharedOptions {
    
    #[clap(short = 't', long = "types", default_value = "false", help = "Generates types using 'npm run generate-types'")]
    pub generate_types: bool,
}
