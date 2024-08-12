use std::fs::File;

use crate::parser::config::{ Argument, ColumnType };

#[derive(Clone, Debug)]
#[cfg_attr( test, derive(PartialEq) )]
pub enum CellValue {
	Int(i64),
	UnsignedInt(u64),
	Float(f64),
	String(String),
	Boolean(bool),
}

impl From<&Argument> for CellValue {
	fn from(arg: &Argument) -> Self {
		match arg {
			Argument::Int(value) => Self::Int( value.clone() ),
			Argument::Float(value) => Self::Float( value.clone() ),
			Argument::String(value) => Self::String( value.clone() ),
			Argument::Boolean(value) => Self::Boolean( value.clone() ),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ColumnData {
	pub name: String,
	pub r#type: ColumnType,
	pub data: Vec<CellValue>,
}

pub type GeneratorData = Vec<ColumnData>;

#[derive(Debug)]
pub enum GeneratorError { // {{{
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
} // }}}

pub struct GeneratorCreationData {}

pub trait GeneratorImpl { // {{{
	fn new() -> Result<Self, GeneratorError>
		where Self: Sized;

	fn init(&mut self, table_name: String, row_count: usize, output_file: File) -> Result<(), GeneratorError>;

	fn format_cell_value(&mut self, value: &CellValue) -> Result<String, GeneratorError>;

	fn generate(&mut self, data: GeneratorData) -> Result<(), GeneratorError>;
} // }}}
