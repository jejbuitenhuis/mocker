use std::fs::File;
use thiserror::Error;

use crate::parser::config::{
	self as Types,
	Argument,
	ColumnType,
};

#[derive(Clone, Debug)]
#[cfg_attr( test, derive(PartialEq) )]
pub enum CellValue {
	Int(i64),
	UnsignedInt(u64),
	Float(f64),
	String(String),
	Boolean(bool),
}

impl From<&Argument> for CellValue { // {{{
	fn from(arg: &Argument) -> Self {
		match arg {
			Argument::Int(value) => Self::Int( value.clone() ),
			Argument::Float(value) => Self::Float( value.clone() ),
			Argument::String(value) => Self::String( value.clone() ),
			Argument::Boolean(value) => Self::Boolean( value.clone() ),
		}
	}
} // }}}

impl std::fmt::Display for CellValue { // {{{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		macro_rules! write_type {
			($key: expr, $value: expr) => {
				write!(f, "{}[{}]", $key, $value)
			}
		}

		match self {
			Self::Int(value) => write_type!(Types::KEY_COLUMN_TYPE_INT, value),
			Self::UnsignedInt(value) => write_type!(Types::KEY_COLUMN_TYPE_UNSIGNED_INT, value),
			Self::Float(value) => write_type!(Types::KEY_COLUMN_TYPE_FLOAT, value),
			Self::Boolean(value) => write_type!(Types::KEY_COLUMN_TYPE_BOOLEAN, value),
			Self::String(value) => write_type!(Types::KEY_COLUMN_TYPE_STRING, value),
		}
	}
} // }}}

#[derive(Clone, Debug)]
pub struct ColumnData {
	pub name: String,
	pub r#type: ColumnType,
	pub data: Vec<CellValue>,
}

pub type GeneratorData = Vec<ColumnData>;

#[derive(Debug, Error)]
pub enum GeneratorError { // {{{
	// TODO: Add generator name?
	/// Used when a generator is asked to generate data, but it's not
	/// initialized.
	#[error("uninitialized generator")]
	Uninitialized,

	/// Used when something goes wrong while writing to the output file.
	#[error("something went wrong while writing data to the output file: {0}")]
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
