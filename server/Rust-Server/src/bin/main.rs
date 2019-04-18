// extern crate ftp;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::str;
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
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // let pool = ThreadPool::new(4);
    // let signals = Signals::new(&[SIGINT])?;

    for stream in listener.incoming().take(5){
    	let stream = stream.unwrap();

        print!("incoming IP {:?} ", stream.local_addr().unwrap());

        

    	// pool.execute(|| {
            handle_connection(stream);
        // });
    }

    println!("shutting down");
}

fn handle_connection(mut stream: TcpStream){
	let mut buffer = [0; 512];

	stream.read(&mut buffer).unwrap();

	let get_home = b"GET / HTTP/1.1\r\n";
    let simple_home = b"home\n";
	let get_file = b"GET /file HTTP/1.1\r\n";
    let simple_file = b"file";

	//  get localhost:port/
	if buffer.starts_with(get_home) || buffer.starts_with(simple_home) {

        println!("request: {:?}", str::from_utf8(get_home).unwrap());

		// response code and content
        let response = "HTTP/1.1 200 OK\r\n\r\npenis";

        //  sending response
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } 

    else if buffer.starts_with(get_file) || buffer.starts_with(simple_file) {  //  tries to transfer a file

        println!("request: {:?}", str::from_utf8(get_file).unwrap());

        // let ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();

    	let contents = fs::read_to_string("test-file.exe").unwrap();

		// response code and content
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        //  sending response
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\nfail";

        println!("request: {:?}", status_line);
        let response = format!("{}", status_line);

        stream.write(response.as_bytes()).unwrap();
    	stream.flush().unwrap();
    }
}