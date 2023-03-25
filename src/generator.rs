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

pub struct GeneratorCreationData {}

pub trait GeneratorImpl { // {{{
	fn new() -> Result<Self, GeneratorError>
		where Self: Sized;

	fn init(&mut self, table_name: String, row_count: usize, output_file: File) -> Result<(), GeneratorError>;

	fn generate(&mut self, data: GeneratorData) -> Result<(), GeneratorError>;
} // }}}
