use std::io::prelude::*;
use std::net::TcpStream;

use std::thread;

static NTHREADS: i32 = 100;

fn main() {
	let mut children = vec![];

	for i in 0..NTHREADS{
		// Spin up another thread
        children.push(thread::spawn(move || {
        	let mut buffer = String::with_capacity(512);
			let request = String::from("GET /api/equation/1 HTTP/1.1\r\n");
        	
			
		    if let Ok(mut stream) = TcpStream::connect("localhost:80") {
		    	println!("Connected to the server!");
		    	let sent = stream.write(request.as_bytes());
		    	match sent{
		    		
		    		Ok(_o)=>{
		    			println!("it is ok ");
		    		},
		    		Err(e)=>{
		    			println!("there is an error {}", e);
		    		}
		    	};
		    	stream.shutdown(std::net::Shutdown::Write).expect("could not shutdown");
		    	println!("waiting on response");
		    	let rec = stream.read_to_string(&mut buffer);
		    	match rec {
		    		Ok(o)=>{
		    			println!("received {}", o);
		    		},
		    		Err(e)=>{
		    			if e.kind() == std::io::ErrorKind::WouldBlock {
                                    println!("Would Block");
                                }
		    			println!("error when receiving {}", e);
		    		}
		    	}
		    	println!("received from server: {:?}", buffer);
		    	
			} // end of stream

			else {
			    println!("Couldn't connect to server...");
			}

		    println!("shutting down");
            println!("this is thread number {}", i);
        }));
	}

 	for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}