extern crate hyper;
extern crate html5ever;

use html5ever::rcdom::RcDom;
use html5ever::driver::parse_document;
use hyper::Uri;
use std::env;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
    	let hostname = arg1.parse::<Uri>().unwrap();

    	let client = hyper::Client::new();
		let mut res = client.get(hostname).send().unwrap();
		let dom = parse_document(RcDom::default(),
    		Default::default()).from_utf8().read_from(&mut res).unwrap();
    }
}