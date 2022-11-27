use std::fs::File;

use crate::parser::config::ColumnType;

#[derive(Clone)]
pub struct ColumnData {
	pub name: String,
	pub r#type: ColumnType,
	pub data: Vec<String>,
}

pub type GeneratorData = Vec<ColumnData>;

#[derive(Debug)]
pub enum GeneratorError {
	/// Used when the output file {0} already exists.
	FileAlreadyExists(String),

	/// Used when something goes wrong while writing to the output file.
	Write(String),
}

pub trait GeneratorImpl<'a> {
	fn new(table_name: String, row_count: usize, output_file: &'a mut File) -> Self
		where Self: Sized;

	fn generate(&mut self, data: GeneratorData) -> Result<(), GeneratorError>;
}
