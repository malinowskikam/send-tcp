use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    /// Address of a open TCP socket as <IP>:<PORT>
    #[arg(index = 1)]
    pub address: String,

    /// Input file path
    #[arg(index = 2)]
    pub file_path: Option<PathBuf>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
