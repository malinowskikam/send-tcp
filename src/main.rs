mod args;

use std::{io::Read, fs::File, process::exit};
use args::{parse_args, Args};
use anyhow::{Result, Error};

const EXIT_INPUT_FAILED: i32 = 1;

fn main() {
    let args = parse_args();
    println!("{:?}", &args);

    let input = match get_input(&args) {
        Ok(input) => input,
        Err(e) => exit_with_error(e, EXIT_INPUT_FAILED)
    };
    
    println!("{:?}", std::str::from_utf8(&input).unwrap())    
}

fn exit_with_error(error: Error, code: i32) -> ! {
    println!("error: {}", error);
    exit(code)
}

fn get_input(args: &Args) -> Result<Vec<u8>> {
    let mut input: Vec<u8> = Vec::new();

    if let Some(file_path) = &args.file_path {
        let mut file = File::open(file_path)?;
        file.read_to_end(&mut input)?;
    } else {        
        std::io::stdin().lock().read_to_end(&mut input)?;
    };

    Ok(input)
}
