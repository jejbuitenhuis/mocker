use std::collections::HashMap;
use std::fs::File;

use crate::parser::config::ColumnType;

#[derive(Clone, Debug)]
pub struct ColumnData {
	pub name: String,
	pub r#type: ColumnType,
	pub data: Vec<String>,
}

pub type GeneratorData = Vec<ColumnData>;

#[derive(Debug)]
pub enum GeneratorError {
	/// Used when a generator is already registered under the name {0}.
	AlreadyRegistered(String),

	/// Used when the output file {0} already exists.
	FileAlreadyExists(String),

	/// Used when for some reason the check in the args let an unknown
	/// generator named {0} through.
	UnknownGenerator(String),

	/// Used when a generator is asked to generate data, but it's not
	/// initialized.
	Uninitialized,

	/// Used when something goes wrong while writing to the output file.
	Write(String),
}

pub trait GeneratorImpl { // {{{
	fn new() -> Self
		where Self: Sized;

	fn init(&mut self, table_name: String, row_count: usize, output_file: File) -> Result<(), GeneratorError>;

	// FIXME: Switch GeneratorData to HashMap<String, GeneratorData> so the
	// generator can generate multiple tables
	fn generate(&mut self, data: GeneratorData) -> Result<(), GeneratorError>;
} // }}}

pub struct GeneratorRegistry {
	generators: HashMap< String, Box<dyn GeneratorImpl> >,
}

impl GeneratorRegistry { // {{{
	pub fn new() -> Self {
		Self {
			generators: HashMap::new(),
		}
	}

	pub fn get(&mut self, name: impl ToString) -> Option< &mut Box<dyn GeneratorImpl> > {
		self.generators.get_mut( &name.to_string() )
	}

	pub fn register( &mut self, name: impl ToString, generator: impl GeneratorImpl + 'static ) -> Result<(), GeneratorError> {
		// TODO: Switch to https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.try_insert
		// when it is in stable
		if self.generators.get( &name.to_string() ).is_some() {
			return Err( GeneratorError::AlreadyRegistered( name.to_string() ) );
		}

		self.generators.insert( name.to_string(), Box::new(generator) );

		Ok(())
	}
} // }}}
