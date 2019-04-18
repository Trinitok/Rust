// extern crate ftp;


use std::io::Write;
use std::io::{Read};
use std::io;
use std::net::TcpStream;
use std::fs::File;


// use std::{error::Error};
// use signal_hook::{iterator::Signals, SIGINT};

// use ftp::FtpStream; 

// use Rust_Server::ThreadPool;

// struct ThreadPool;
// impl ThreadPool {
//     fn new(size: u32) -> ThreadPool { 
//         ThreadPool }
//     fn execute<F>(&self, f: F)
//         where F: FnOnce() + Send + 'static {}
// }

fn main() {
	let mut buffer = String::with_capacity(512);
	let mut request = String::new();
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
    	println!("Connected to the server!");
    	// stream.take(5);
    	io::stdin().read_line(&mut request);

    	println!("read from user {:?}", request);

    	stream.write(request.as_bytes());
    	stream.read_to_string(&mut buffer);
    	println!("received from server: {:?}", buffer);

    	if request.starts_with("file") {
    		let name = request.split_off(5);
    		let mut file = File::create(name).unwrap();
    		file.write_all(buffer.as_bytes()).unwrap();

    	}
    	
	} else {
	    println!("Couldn't connect to server...");
	}

    println!("shutting down");
}