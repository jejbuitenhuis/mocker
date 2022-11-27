use clap::Parser as CliParser;
use std::fs;

use crate::{
	arguments::Args,
	generator::{
		ColumnData,
		GeneratorError,
		GeneratorRegistry,
	},
	parser::{
		config::ColumnType,
		Parser,
	},
	provider::{
		ProviderError,
		ProviderRegistry,
	},
	registrars::{
		register_providers,
		register_generators,
	},
};

mod arguments;
mod parser;
mod provider;
mod providers;
mod generator;
mod generators;
mod registrars;

// FIXME: Change `ProviderError` to a more generic error
fn main() -> Result<(), ProviderError> {
	let args = Args::parse();
	let mut parser = Parser::new(args.config).unwrap();
	let config = parser.parse().unwrap();
	let mut provider_registry = ProviderRegistry::new();
	let mut generator_registry = GeneratorRegistry::new();

	println!("{:#?}", config);

	register_providers(&mut provider_registry);
	register_generators(&mut generator_registry);

	provider_registry.init_providers(args.row_count)?;

	let mut generated_data = Vec::with_capacity(2);

	// generate row numbers {{{
	let row_number_provider = provider_registry.get("row")
		.unwrap();

	let mut generated_number_rows = Vec::with_capacity(args.row_count);

	for _ in 0..args.row_count {
		let result = row_number_provider.provide()?;

		println!("Row number: {:?}", result);

		generated_number_rows.push(result);
	}

	generated_data.push( ColumnData {
		name: "row".to_string(),
		r#type: ColumnType::Int,
		data: generated_number_rows,
	} );
	// }}}

	// generate genders {{{
	let gender_provider = provider_registry.get("gender")
		.unwrap();

	let mut generated_gender_rows = Vec::with_capacity(args.row_count);

	for _ in 0..args.row_count {
		let result = gender_provider.provide()?;

		println!("Gender: {:?}", result);

		generated_gender_rows.push(result);
	}

	generated_data.push( ColumnData {
		name: "gender".to_string(),
		r#type: ColumnType::String(1),
		data: generated_gender_rows,
	} );
	// }}}

	let output_file = fs::File::create(args.output)
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |e| ProviderError::Unknown( e.to_string() ) )?;
	let generator = generator_registry.get( args.r#type.clone() )
		.ok_or( GeneratorError::UnknownGenerator( args.r#type.to_string() ) )
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |_| ProviderError::Unknown( "Unknown generator".to_string() ) )?;

	generator.init(
		"some_table".to_string(),
		args.row_count,
		output_file,
	)
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;

	generator.generate(generated_data)
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;

	Ok(())
}
