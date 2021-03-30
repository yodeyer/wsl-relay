use std::env::args;
use std::fs::File;
use std::io::{copy, stdin, stdout};
use std::net::TcpStream;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::thread::spawn;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage : relay.exe <destination_host>:<destination_port>");
        std::process::exit(0)
    }

    let mut stream_in = TcpStream::connect(&args[1]).unwrap();
    let mut stream_out = stream_in.try_clone().unwrap();

    unsafe {
        let mut stdin_file = File::from_raw_handle(stdin().as_raw_handle());
        let mut stdout_file = File::from_raw_handle(stdout().as_raw_handle());

        let thread = spawn(move || {
            copy(&mut stdin_file, &mut stream_out).unwrap();
        });
        copy(&mut stream_in, &mut stdout_file).unwrap();

        thread.join().unwrap();
    }
}
