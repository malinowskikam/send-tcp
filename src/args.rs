use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of a open TCP socket
    #[arg(index = 1)]
    address: String,

    /// Source file
    #[arg(index = 2)]
    file: Option<PathBuf>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
