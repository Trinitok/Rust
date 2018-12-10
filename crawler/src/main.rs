extern crate html5ever;
extern crate url;
extern crate fetch;

mod parse;
mod fetch;
mod crawler;

use std::env;
use std::io::stdout;
use std::io::Write;
use url::Url;

use fetch::UrlState;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
    	let start_url_string = &args[1];

    	let start_url = Url::parse(start_url_string).unwrap();
    	let domain = start_url.domain().expect("I can't find a domain in your URL");

    	let mut success_cnt = 0;
    	let mut fail_cnt = 0;

    	for url_state in crawler::crawl(&domain, &start_url){
    		match url_state{
    			UrlState::Accessible(_) => {
    				success_cnt += 1;

    			}
    			status => {
    				fail_cnt += 1;
    				println!("{}", status);
    			}
    		}

    		print!("Succeeded: {} Failed {} \n", success_cnt, fail_cnt);
    		stdout().flush().unwrap();
    	}
    }

    else{
    	println!("Please provide a URL");
    }
}
