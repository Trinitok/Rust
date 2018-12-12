extern crate select;
extern crate reqwest;
extern crate url;

use std::io::Read;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use url::Url;


//  select example from https://github.com/utkarshkukreti/select.rs
fn main() {

	let target = "https://docs.rs/reqwest/0.2.0/reqwest/struct.Response.html";
	let mut body = String::new();

	let mut response = reqwest::get(target)
    	.expect("Failed to send request");

    response.read_to_string(&mut body);

    println!("{}", response.status());
    println!("{}", body);

    let parsed = Url::parse(target);

    for i in Document::from(include_str!(&body)).find(Name("a")) {
    	println!("{:?}",i.text() );       //prints text content of all articles
	};

	//  this link is loaded in
	// let document = Document::from(include_str!("stackoverflow.html"));
}

// #[macro_use] extern crate serde_derive;
// #[macro_use] extern crate serde_json;
// #[macro_use] extern crate structopt_derive;
// extern crate failure;
// extern crate kuchiki;
// extern crate reqwest;
// extern crate serde;
// extern crate structopt;

// use failure::Error;
// use kuchiki::*;
// use kuchiki::traits::*;
// use std::fs::File;
// use std::fs::create_dir;
// use std::thread;
// use std::time;

// #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
// struct Votes {
//     upvotes: usize,
//     piggies: usize,
//     downvotes: usize,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// enum Vote {
//     Rulez,
//     Isok,
//     Sucks,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Comment {
//     content: String,
//     user: String,
//     vote: Vote,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Prod {
//     title: String,
//     votes: Votes,
//     comments: Vec<Comment>,
// }

// fn get_document(url: &str) -> Result<NodeRef, Error> {
//     let mut res = reqwest::get(url)?;
//     Ok(kuchiki::parse_html().one(res.text()?))
// }

// fn prod_from_url(pouet_prod_url: &str) -> Option<Prod> {
//     let document = get_document(pouet_prod_url).ok()?;

//     let votes_selector = document.select("#pouetbox_prodmain .r2").unwrap();
//     let votes_wrapper: NodeDataRef<ElementData> = votes_selector.into_iter().next()?;
//     let text_content = votes_wrapper
//         .text_contents();
//     let numbers: Vec<usize> = text_content
//         .lines()
//         .filter(|s| s.chars().any(|c| !c.is_whitespace()))
//         .filter_map(|s| {
//             s.trim().parse::<usize>().ok()
//         })
//         .collect();

//     if numbers.len() != 3 {
//         println!("Received more or less than 3 numbers :(");
//         return None;
//     }

//     let votes = Votes {
//         upvotes: numbers[0],
//         piggies: numbers[1],
//         downvotes: numbers[2],
//     };

//     let title = document.select("#title big").unwrap().next()?.text_contents().trim().to_owned();

//     Some(Prod {
//         title,
//         votes,
//         comments: vec!(),
//     })
// }


// use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// #[structopt(name = "prodbot", about = "Scraper for pouet.net")]
// struct Opt {
//     #[structopt(long = "clear_cache", help = "Clear cache directory")]
//     clear_cache: bool,

//     #[structopt(long = "slack_webhook_url", help = "Target slack webhook url. Omitting will only print to console instead")]
//     slack_webhook_url: Option<String>,

//     #[structopt(long = "pouet_prod_ids", help = "Which pouet prod ids to listen to")]
//     pouet_prod_ids: Vec<usize>,
// }

// fn check_prods(prods: &[(&usize, String)], options: &Opt) -> Result<(), Error> {
//     let client = reqwest::Client::new();

//     for &(prod_id, ref prod_url) in prods {
//         if let Some(prod) = prod_from_url(&prod_url) {
//             let cache_key = &format!("cache/{}.json", prod_id);

//             let mut cached_prod: Option<Prod> = None;
//             if let Ok(file) = File::open(cache_key) {
//                 let concrete_cached_prod: Prod = serde_json::from_reader(file)?;
//                 if concrete_cached_prod.votes == prod.votes {
//                     println!("Prod {} has no difference between pouet and cache. Skipping webhook delivery", prod.title);
//                     continue;
//                 }

//                 cached_prod = Some(concrete_cached_prod);
//             }

//             let postfix = if let Some(cached_prod) = cached_prod {
//                 format!(", up from {:#?}", cached_prod)
//             } else {
//                 String::new()
//             };
//             let slack_text = format!("Prod <{}|{}> now has {:#?}{}",
//                                      prod_url, prod.title, prod.votes, postfix
//             );

//             println!("{}", slack_text);
//             if let Some(ref slack_webhook_url) = options.slack_webhook_url {
//                 client.post(slack_webhook_url)
//                     .json(&json!({
//                             "text": slack_text,
//                         }))
//                     .send()?;
//                 println!("Delivered slack webhook");
//             }

//             serde_json::to_writer(File::create(cache_key)?, &prod)?;
//         }
//     }

//     Ok(())
// }


// fn run() -> Result<(), Error> {
//     let options: Opt = Opt::from_args();
//     let prods: Vec<(&usize, String)> = options.pouet_prod_ids
//         .iter()
//         .map(|id| (id, format!("https://www.pouet.net/prod.php?which={}", id)))
//         .collect();

//     if options.clear_cache {
//         std::fs::remove_dir_all("cache")
//             .unwrap_or_else(
//                 |error| println!("Error deleting cache foolder: {:?}", error)
//             );
//     }

//     create_dir("cache")
//         .unwrap_or_else(
//             |error| println!("Error creating cache directory: {:?}", error));

//     let sleep_duration = time::Duration::from_secs(60);
//     loop {
//         println!("Checking prods {:?}", options.pouet_prod_ids);
//         if let Err(error) = check_prods(&prods, &options) {
//             println!("Encountered error checking prods: {:?}", error);
//         }
//         println!("Sleeping for {:?}", sleep_duration);
//         thread::sleep(sleep_duration);
//     }
// }

// fn main() {
//     if let Err(error) = run() {
//         for cause in error.causes() {
//             println!("{:?}", cause);
//         }
//     }
// }


// extern crate select;

// use hyper::Client;
// use std::io::Read;
// use select::document::Document;
// use select::predicate::{Class};

// fn main() {
// 	println!("working");
// 	let client = Client::new();
// 	let response =
//     	client.get("https://brson.github.io/demo/wishlist.html")
//     		.send()
//     		.expect("Request failed");
//     let mut body = String::new();
// 	response.read_to_string(&mut body).expect("Read failed");
// 	println!("{:?}", body);

// 	let document = Document::from(body.as_str());
// 	let rows = document.find(Class("a-row"));

// 	for row in rows.iter() {
//     	println!(" * Row {}", row.text());
//     	let maybe_name_node = row.find(Name("h5")).first();
// 		let maybe_price_node = row.find(Class("a-color-price")).first();
// 	}

	
// }


// extern crate futures;
// extern crate hyper;
// extern crate tokio_core;

// use std::io::{self, Write};
// use futures::{Future, Stream};
// use hyper::Client;
// use tokio_core::reactor::Core;


// fn main() {
//     let mut core = Core::new().unwrap();
//     let client = Client::new(&core.handle());

//     let uri = "https://www.google.com".parse().unwrap();
//     let work =
//         client.get(uri).and_then(|res| {
//             println!("Response: {}", res.status());

//             res.body().for_each(|chunk| {
//                 io::stdout()
//                     .write_all(&chunk)
//                     .map_err(From::from)
//             })
//         });
//     core.run(work).unwrap();
// }



// extern crate hyper;
// extern crate html5ever;

// use html5ever::rcdom::RcDom;
// use html5ever::driver::parse_document;
// use hyper::Uri;
// use std::env;

// fn main() {
//     if let Some(arg1) = env::args().nth(1) {
//     	let hostname = arg1.parse::<Uri>().unwrap();

//     	let client = hyper::Client::new();
// 		let mut res = client.get(hostname).send().unwrap();
// 		let dom = parse_document(RcDom::default(),
//     		Default::default()).from_utf8().read_from(&mut res).unwrap();
//     }
// }