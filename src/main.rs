use crate::parser::Parser;

mod parser;

fn main() {
	let mut parser = Parser::new( "test.mock".to_string() ).unwrap();

	let result = parser.parse().unwrap();

	println!("{:#?}", result);
}
