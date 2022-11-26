use clap::Parser as CliParser;
use std::{
	collections::HashMap,
	fs,
};

use crate::{
	arguments::Args,
	generator::GeneratorImpl,
	generators::tsql::TsqlGenerator,
	parser::{
		config::ColumnType,
		Parser,
	},
	provider::{
		ProviderError,
		ProviderRegistry,
	},
	register_providers::register_providers,
};

mod arguments;
mod parser;
mod provider;
mod providers;
mod generator;
mod generators;
mod register_providers;

// TODO: Change `ProviderError` to a more generic error
fn main() -> Result<(), ProviderError> {
	let args = Args::parse();
	let mut parser = Parser::new(args.config).unwrap();
	let config = parser.parse().unwrap();
	let mut provider_registry = ProviderRegistry::new();

	println!("{:#?}", config);

	register_providers(&mut provider_registry);

	provider_registry.init_providers(args.row_count)?;

	// TODO: Switch to a map that preserves order
	let mut generated_data = HashMap::with_capacity(2);

	// generate row numbers {{{
	let row_number_provider = provider_registry.get("row")
		.unwrap();

	let mut generated_number_rows = Vec::with_capacity(args.row_count);

	for _ in 0..generated_number_rows.capacity() {
		let result = row_number_provider.provide()?;

		println!("Row number: {:?}", result);

		generated_number_rows.push(result);
	}

	generated_data.insert(
		( "row".to_string(), ColumnType::Int ),
		generated_number_rows,
	);
	// }}}

	// generate genders {{{
	let gender_provider = provider_registry.get("gender")
		.unwrap();

	let mut generated_gender_rows = Vec::with_capacity(args.row_count);

	for _ in 0..generated_gender_rows.capacity() {
		let result = gender_provider.provide()?;

		println!("Gender: {:?}", result);

		generated_gender_rows.push(result);
	}

	generated_data.insert(
		( "gender".to_string(), ColumnType::String(1) ),
		generated_gender_rows,
	);
	// }}}

	let mut output_file = fs::File::create(args.output)
		.map_err( |e| ProviderError::Unknown( e.to_string() ) )?;
	let mut generator = TsqlGenerator::new(
		"some_table".to_string(),
		args.row_count,
		&mut output_file,
	);

	generator.generate(generated_data)
		.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;

	Ok(())
}
