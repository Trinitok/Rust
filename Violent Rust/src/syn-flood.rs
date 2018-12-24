extern crate lib_blaster;

use lib_blaster::builder::Pkt;
use lib_blaster::collector::Collector;
use lib_blaster::sender::Sender;
use lib_blaster::tools::{fast_random, rand_ipv4}
use std::net::Ipv4Addr;


///  This will take in a vector of open ports
fn send_pkt(vec: Vector<i32>) {
	for port_val in vec{
		let mut packet = Pkt::new(&Collector{
			src_ip = 
		}).unwrap();
	}
}