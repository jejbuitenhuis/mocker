use std::collections::HashMap;
use clap::Parser as CliParser;
use std::fs;

use crate::{
	arguments::Args,
	generator::{
		ColumnData,
		GeneratorData,
		GeneratorError,
		GeneratorRegistry,
	},
	parser::Parser,
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
	// Initialize mocker and parse config {{{
	let args = Args::parse();
	let mut parser = Parser::new(args.config).unwrap();
	let config = parser.parse().unwrap();
	let mut provider_registry = ProviderRegistry::new();
	let mut generator_registry = GeneratorRegistry::new();

	println!("{:#?}", config);

	register_providers(&mut provider_registry);
	register_generators(&mut generator_registry);

	provider_registry.init_providers(args.row_count)?;
	// }}}

	// Generate mock data {{{
	let mut generated_data: HashMap< String, GeneratorData >
		= HashMap::with_capacity( config.tables.len() );

	for table in &config.tables {
		let mut columns: GeneratorData
			= Vec::with_capacity( table.columns.len() );

		for column in &table.columns {
			let mut rows = Vec::with_capacity(args.row_count);
			let provider = provider_registry.get( column.provider.name.clone() )?;

			provider.reset(&column.provider.arguments)?;

			for _ in 0..args.row_count {
				rows.push( provider.provide()? );
			}

			columns.push( ColumnData {
				name: column.name.clone(),
				r#type: column.kind,
				data: rows,
			} );
		}

		generated_data.insert(
			table.name.clone(),
			columns,
		);
	}

	println!("{:#?}", generated_data);
	// }}}

	// generate output {{{
	// FIXME: We have multiple tables... At the moment we spit out one file
	// with a hard-coded table name. How do we switch to the HashMap containing
	// generated columns with rows per table?
	let output_file = fs::File::create(args.output)
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |e| ProviderError::Unknown( e.to_string() ) )?;
	let generator = generator_registry.get( args.r#type.clone() )
		.ok_or( GeneratorError::UnknownGenerator( args.r#type.to_string() ) )
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |_| ProviderError::Unknown( "Unknown generator".to_string() ) )?;

	generator.init(
		"SimpleTable".to_string(),
		args.row_count,
		output_file,
	)
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;

	generator.generate( generated_data.get("SimpleTable").unwrap().to_owned() )
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;
	// }}}

	Ok(())
}
