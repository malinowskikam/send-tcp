mod args;
mod input;

use std::{io::{Read, Write}, fs::File, process::exit, net::{TcpStream, TcpListener}, time};
use args::{parse_args, Args};
use anyhow::{Result, Error};

const EXIT_INPUT_FAILED: i32 = 1;
const EXIT_CONN_FAILED: i32 = 2;
const EXIT_WRITE_FAILED: i32 = 3;

fn main() {
    let handle = std::thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:22000").unwrap();
        let mut stream = listener.accept().unwrap().0;
        let mut buff = String::new();
        stream.read_to_string(&mut buff).unwrap();
        stream.write_all(&[0]).unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    });

    std::thread::sleep(time::Duration::from_secs(1));

    let args = parse_args();

    let input = match read_input(&args) {
        Ok(input) => input,
        Err(e) => exit_with_error(e, EXIT_INPUT_FAILED)
    };
    println!("req: {:?}", input);
   
    let mut tcp_stream = match TcpStream::connect(&args.address) {
        Ok(stream) => stream,
        Err(e) => exit_with_error(e.into(), EXIT_CONN_FAILED)
    };

    std::thread::sleep(time::Duration::from_millis(100));

    match tcp_stream.write_all(&input) {
        Ok(_) => {
            tcp_stream.shutdown(std::net::Shutdown::Write).unwrap();
            
            let mut buf: Vec<u8> = Vec::new();
            tcp_stream.read_to_end(&mut buf).unwrap();
            println!("res: {:?}", buf);

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

fn read_input(args: &Args) -> Result<Vec<u8>> {
    let mut input: Vec<u8> = Vec::new();

    if let Some(file_path) = &args.file_path {
        let mut file = File::open(file_path)?;
        file.read_to_end(&mut input)?;
    } else {        
        std::io::stdin().lock().read_to_end(&mut input)?;
    };

    Ok(input)
}
