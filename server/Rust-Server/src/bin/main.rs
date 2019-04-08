extern crate ftp;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

use ftp::FtpStream; 

use Rust_Server::ThreadPool;

// struct ThreadPool;
// impl ThreadPool {
//     fn new(size: u32) -> ThreadPool { 
//         ThreadPool }
//     fn execute<F>(&self, f: F)
//         where F: FnOnce() + Send + 'static {}
// }

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2){
    	let stream = stream.unwrap();

    	pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("shutting down");
}

fn handle_connection(mut stream: TcpStream){
	let mut buffer = [0; 512];

	stream.read(&mut buffer).unwrap();

	let get_home = b"GET / HTTP/1.1\r\n";
	let get_file = b"GET /file HTTP/1.1\r\n";

	//  get localhost:port/
	if buffer.starts_with(get_home) {

		// response code and content
        let response = "HTTP/1.1 200 OK\r\n\r\npenis";

        //  sending response
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } 

    else if buffer.starts_with(get_file) {  //  tries to transfer a file

        let ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();

    	let contents = fs::read_to_string("test-file.exe").unwrap();

		// response code and content
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        //  sending response
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\nfail";
        let response = format!("{}", status_line);

        stream.write(response.as_bytes()).unwrap();
    	stream.flush().unwrap();
    }
}