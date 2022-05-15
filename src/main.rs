use clap::Parser as CliParser;

use crate::{
	arguments::Args,
	parser::Parser,
	provider::ProviderRegistry,
	register_providers::register_providers,
};

mod arguments;
mod parser;
mod provider;
mod providers;
mod register_providers;

fn main() {
	let args = Args::parse();
	let mut parser = Parser::new(args.config).unwrap();
	let config = parser.parse().unwrap();
	let mut provider_registry = ProviderRegistry::new();

	println!("{:#?}", config);

	register_providers(&mut provider_registry);

	provider_registry.init_providers(args.row_count);

	let number_provider = provider_registry
		.get("number")
		.unwrap();

	number_provider.reset( &vec![ "0".to_string(), "1000".to_string() ] ).unwrap();

	for _ in 0..100 {
		println!("Result: {:?}", number_provider.provide());
	}
}
