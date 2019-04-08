#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

#[test]
fn calculator2() {
    let result = calculator1::TermParser::new().parse("33").unwrap();
    assert_eq!(result, "33");

    let result = calculator1::TermParser::new().parse("(22)").unwrap();
    assert_eq!(result, "Twenty-two!");

    println!("Hello World!");

    let result = calculator1::TermParser::new().parse("(222)").unwrap();
    assert_eq!(result, "222");
}

fn main() {
	println!("Hello World!");
}