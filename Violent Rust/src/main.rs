extern crate getopts;
extern crate synner;

use getopts::Options;
use std::env;
use std::mem;
use std::net::{TcpStream, IpAddr, Ipv4Addr, Ipv6Addr};
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

use synner::tcp::packet::{send_tcp_packets};

const MAX: u16 =  65535;


///  for if they pass the -h argument
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} stuff", program);
    print!("{}", opts.usage(&brief));
}

fn scan_target(tx: Sender<u16>, host_ip: &str, port: u16) {
    match TcpStream::connect((host_ip, port)){
        Ok(_) => {
            println!("port {} is accessible", port);
            tx.send(port).unwrap();
        }
        Err(_) => {
            println!("there was an error trying to access port {}", port);
        }
    };
}

///  This is the actual port scanning function
//  might take a while to scan all ports
fn scan_all(tx: Sender<u16>, start_port: u16, host_ip: &str, num_threads: u16 ) {
    let mut port: u16 = start_port + 1;

    loop {
        match TcpStream::connect((host_ip, port)){
            Ok(_) => {
                println!("port {} is accessible", port);
                tx.send(port).unwrap();
            }
            Err(_) => {
                println!("there was an error trying to access port {}", port);
            }
        };

        if (MAX - port) <= num_threads{
            break;
        }

        port += num_threads;
    }
}

///  turn any strings into &'static strings
//  this particular function can incur memory leaks
//  use with caution
fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

fn main() {

	let args_vec: Vec<String> = env::args().collect();

    //  command line options parser
	let mut opts = Options::new();
    opts.reqopt("H", "host", "set target host name", "Host Name");
    opts.optopt("p", "port", "scan individual target port number", "Port Number");
    opts.optopt("t", "threads", "scan all ports with set number of threads", "thread number");
    opts.optflag("h", "help", "print this help menu");

    //  parse the command line args.  the first arg is the program itself
    let matches = match opts.parse(&args_vec[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    //  if the user needs to see how to register for help
    if matches.opt_present("h") {
        print_usage(&args_vec[0].clone(), opts);
        return;
    }

    let host = string_to_static_str(matches.opt_str("host").unwrap());
    
    //  test using synner for syn flood attack
    send_tcp_packets(host.parse::<Ipv4Addr>().unwrap(), "eth0".to_string(), 1);

    

    let (tx, rx) = channel();
	
    //  if there is a specific port to target
    if matches.opt_present("p") {
        let p = matches.opt_str("p").unwrap().parse().unwrap();

        thread::spawn(move || {
            scan_target(tx, host, p);
        });
    }
    //  if target ports were not defined
    else if matches.opt_present("t"){
        let num_threads = matches.opt_str("t").unwrap().parse().unwrap();

        for i in 0 .. num_threads{
            let tx = tx.clone();

            thread::spawn(move || {
                scan_all(tx, i, host, num_threads);
            });
        }
       
    }
    else{
        print_usage(&args_vec[0].clone(), opts);
        return;
    }

	
    // drop(tx);
    let mut out = vec![];
    
    for p in rx{
        out.push(p);
    }

    out.sort();
    for v in out{
        println!("{} is open", v);
    }
}