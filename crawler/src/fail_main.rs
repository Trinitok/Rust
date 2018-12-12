extern crate html5ever;

use std::io::{self, Write};
use std::default::Default;

use html5ever::{parse_document, serialize};
use html5ever::driver::ParseOpts;
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;

fn main() {

	//  simulated URL input
	// let issue_list_url = Url::parse("https://docs.rs/url/1.7.2/url/");

	println!("issue_list_url: {}", issue_list_url);

    //  simulated web data
    let data = "<!DOCTYPE html><html><body><a href=\"foo\"></a></body></html>".to_string();

    //  printing out a web page in chrome
    // let opts = ParseOpts {
    //     tree_builder: TreeBuilderOpts {
    //         drop_doctype: true,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // };

    // let dom = parse_document(RcDom::default(), opts)
    //     .from_utf8()
    //     .read_from(&mut data.as_bytes())
    //     .unwrap();

    // // let document = dom.document.borrow();
    // let html = document.children[0].borrow();
    // let body = html.children[1].borrow(); // Implicit head element at children[0].

    // {
    //     let mut a = body.children[0].borrow_mut();
    //     if let Element(_, _, ref mut attributes) = a.node {
    //         attributes[0].value.push_tendril(&From::from("#anchor"));
    //     }
    // }

    // let mut bytes = vec![];
    // serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    // let result = String::from_utf8(bytes).unwrap();
    // println!("{}", result);

    //  SELECT LIBRARY Testing
    //  if I have the webpage I can just sift through it like in
    // let mut additional_links: VecDeque<&'static str> = VecDeque::new();
    // let mut visited_links: LinkedList<Document> = LinkedList::new();
    // let select_doc = Document::from(include_str!("index.html"));   // use on html doc
    let _start: &'static str = "index.html";
    // let mut select_doc = Document::new();

    additional_links.push_front(_start);

    //  get me all them sweet sweet links.  yummy
    // loop{
    //     let curr_link: &'static str = additional_links.pop_front().unwrap();
    //     println!("here is where I currently am: {}", curr_link);
    //     if visited_link(curr_link, &visited_links){
    //     	continue;
    //     }

    //     visited_links.push_front(Document::from(curr_link));

    //     let select_doc = visited_links.front().unwrap();
        
    //     // let nodes = Document::from(curr_link).find(Name("a"));

    //     for node in select_doc.find(Name("a")){
            
    //         println!("Found: {}", node.attr("href").unwrap());
    //         additional_links.push_back(node.attr("href").unwrap());
    //     }

    //     //  check to see if there are no links left in the queue
    //     if additional_links.is_empty() {
    //         break;
    //     }

        
    // }
    
}