extern crate html5ever;

use html5ever::{ParseOpts, parse_document};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::rcdom::RcDom;
use html5ever::rcdom::NodeEnum::Element;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;

fn main() {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let data = "<!DOCTYPE html><html><body><a href=\"foo\"></a></body></html>".to_string();
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
}