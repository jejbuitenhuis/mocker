use clap::Parser as CliParser;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::{
	arguments::Args,
	generator::{
		ColumnData,
		GeneratorData,
	},
	parser::{
		errors::ParserError,
		Parser,
	},
	provider::ProviderError,
	registry::registrars::{
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
mod registry;

lazy_static! { // {{{
	static ref FILE_EXTENSION_MAPPINGS: HashMap<&'static str, &'static str> = {
		let mut m = HashMap::new();

		m.insert("tsql", "sql");

		m
	};
} // }}}

// FIXME: Change `ProviderError` to a more generic error
fn main() -> Result<(), ProviderError> {
	// Initialize mocker and parse config {{{
	let args = Args::parse();
	let mut parser = Parser::new(&args.config).unwrap();
	let config = parser.parse();

	if let Err( ParserError::SyntaxError(parsing_error) ) = config {
		println!("Syntax error: {}", parsing_error);

		std::process::exit(1);
	}

	let config = config.unwrap();

	println!("Parsed config: {:#?}", config);

	let mut provider_registry = register_providers(&args);
	let mut generator_registry = register_generators(&args);
	// }}}

	// Generate mock data {{{
	let mut generated_data: HashMap< String, GeneratorData >
		= HashMap::with_capacity( config.tables.len() );

	for table in &config.tables {
		let mut columns: GeneratorData
			= Vec::with_capacity( table.columns.len() );

		for column in &table.columns {
			let mut rows = Vec::with_capacity(args.row_count);
			let provider = provider_registry.get( column.provider.name.clone() )
				// FIXME: Change `ProviderError` to a more generic error
				.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;

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

	println!("Generated data: {:#?}", generated_data);
	// }}}

	// generate output {{{
	let output_dir = Path::new(&args.output);
	let file_extension = FILE_EXTENSION_MAPPINGS.get( args.r#type.as_str() )
		// If the type is not in the mappings map, the type is the same as the
		// file extension
		.unwrap_or( &args.r#type.as_str() )
		.to_string();
	let generator = generator_registry.get( args.r#type.clone() )
		// FIXME: Change `ProviderError` to a more generic error
		.map_err( |_| ProviderError::Unknown( "Unknown generator".to_string() ) )?;

	for (table, data) in generated_data.into_iter() {
		let output_file_name = output_dir.join( format!(
			"{}.{}",
			table,
			file_extension.clone(),
		) );

		println!("Using file '{}' for table '{}'", output_file_name.display(), table);

		let output_file = fs::File::create( output_file_name.clone() )
			// FIXME: Change `ProviderError` to a more generic error
			.map_err( |e| ProviderError::Unknown( e.to_string() ) )?;

		generator.init(
			table,
			args.row_count,
			output_file,
		)
			// FIXME: Change `ProviderError` to a more generic error
			.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;

		generator.generate(data)
			// FIXME: Change `ProviderError` to a more generic error
			.map_err( |e| ProviderError::Unknown( format!("{:?}", e) ) )?;
	}
	// }}}

	println!("Done generating data files!");

	Ok(())
}
