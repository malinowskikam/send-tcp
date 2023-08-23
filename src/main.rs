mod args;

use std::{io::{Read, Write}, fs::File, process::exit, net::{TcpStream, TcpListener}, time};
use args::{parse_args, Args};
use anyhow::{Result, Error};

const EXIT_INPUT_FAILED: i32 = 1;
const EXIT_CONN_FAILED: i32 = 2;
const EXIT_WRITE_FAILED: i32 = 3;

fn main() {
    let handle = std::thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:22000").unwrap();
        println!("listening");
        let mut stream = listener.accept().unwrap().0;
        
        println!("reading");
        let mut buff = String::new();
        stream.read_to_string(&mut buff).unwrap();
        println!("{}", buff);
    });

    std::thread::sleep(time::Duration::from_secs(1));


    let args = parse_args();

    let input = match get_input(&args) {
        Ok(input) => input,
        Err(e) => exit_with_error(e, EXIT_INPUT_FAILED)
    };
    println!("{}", input.len());
    
    let mut tcp_stream = match TcpStream::connect(&args.address) {
        Ok(stream) => stream,
        Err(e) => exit_with_error(e.into(), EXIT_CONN_FAILED)
    };
    println!("connected");


    std::thread::sleep(time::Duration::from_secs(1));

    match tcp_stream.write_all(&input) {
        Ok(_) => {
            println!("written");
            tcp_stream.shutdown(std::net::Shutdown::Both).unwrap();
            handle.join().unwrap()
        },
        Err(e) => exit_with_error(e.into(), EXIT_WRITE_FAILED)
    };
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
