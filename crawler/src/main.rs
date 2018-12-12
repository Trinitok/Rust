extern crate html5ever;
extern crate hyper;
extern crate select;

use html5ever::{ParseOpts, parse_document};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::rcdom::RcDom;
use html5ever::rcdom::NodeEnum::Element;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use hyper::Uri;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() {

    //  simulated web data
    let data = "<!DOCTYPE html><html><body><a href=\"foo\"></a></body></html>".to_string();

    //  printing out a web page in chrome
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut data.as_bytes())
        .unwrap();

    let document = dom.document.borrow();
    let html = document.children[0].borrow();
    let body = html.children[1].borrow(); // Implicit head element at children[0].

    {
        let mut a = body.children[0].borrow_mut();
        if let Element(_, _, ref mut attributes) = a.node {
            attributes[0].value.push_tendril(&From::from("#anchor"));
        }
    }

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);

    ///  SELECT LIBRARY Testing
    //  if I have the webpage I can just sift through it like in
    let additional_links = VecDequeue::new();
    let select_doc = Document::from(include_str!("index.html"));   // use on html doc

    //  get me all them sweet sweet links.  yummy
    loop{
        let curr_link = additional_links.pop_front();

        println!("here is where I currently am: {}", curr_link.attr("href").unwrap());

        for node in select_doc.find(Name("a")){
            additional_links.push_back(node);
            println!("Found: {}", node.attr("href").unwrap());

        }

        //  check to see if there are no links left in the queue
        if additional_links.is_empty() {
            break;
        }
    }
    
}