use log::{ debug, info };
use std::{
	collections::HashMap,
	fs,
	path::Path,
};

use crate::{
	arguments::Args,
	parser::{
		errors::ParserError,
		config::Config,
		Parser,
	},
	generator::{ ColumnData, GeneratorData },
	registry::registrars::{
		register_providers,
		register_generators,
	},
};

lazy_static::lazy_static! { // {{{
	static ref FILE_EXTENSION_MAPPINGS: HashMap<&'static str, &'static str> = {
		let mut m = HashMap::new();

		m.insert("tsql", "sql");

		m
	};
} // }}}

type MockData = HashMap<String, GeneratorData>;

pub struct Mocker<'a> {
	args: &'a Args,
}

impl<'a> Mocker<'a> {
	pub fn new(args: &'a Args) -> Self {
		Self { args }
	}

	pub fn parse_config(&self) -> Result<Config, ParserError> { // {{{
		let parser = Parser::new(&self.args.config)?;
		let config = parser.parse()?;

		debug!("Parsed config: {:#?}", config);

		Ok(config)
	} // }}}

	pub fn generate_mock_data(&self, config: Config) -> Result< MockData, anyhow::Error > { // {{{
		let mut provider_registry = register_providers(&self.args)?;

		let mut generated_data: HashMap< String, GeneratorData >
			= HashMap::with_capacity( config.tables.len() );

		for table in &config.tables {
			let mut columns: GeneratorData
				= Vec::with_capacity( table.columns.len() );

			for column in &table.columns {
				let mut rows = Vec::with_capacity(self.args.row_count);
				let provider = provider_registry.get(
					column.provider.name.clone()
				)?;

				provider.reset(&column.provider.arguments)?;

				for _ in 0..self.args.row_count {
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

		debug!("Generated data: {:#?}", generated_data);

		Ok(generated_data)
	} // }}}

	pub fn write_mock_data(&self, generated_data: MockData) -> Result<(), anyhow::Error> { // {{{
		let mut generator_registry = register_generators(&self.args)?;

		let output_dir = Path::new(&self.args.output);
		let file_extension = FILE_EXTENSION_MAPPINGS.get(
			self.args.r#type.as_str(),
		)
			// If the type is not in the mappings map, the type is the same as the
			// file extension
			.unwrap_or( &self.args.r#type.as_str() )
			.to_string();
		let generator = generator_registry.get( self.args.r#type.clone() )?;

		for (table, data) in generated_data.into_iter() {
			let output_file_name = output_dir.join( format!(
				"{}.{}",
				table,
				file_extension.clone(),
			) );

			info!("Using file '{}' for table '{}'", output_file_name.display(), table);

			let output_file = fs::File::create( output_file_name.clone() )?;

			generator.init(
				table,
				self.args.row_count,
				output_file,
			)?;

			generator.generate(data)?;
		}

		info!("Done generating data files!");

		Ok(())
	} // }}}
}
