use std::{
	collections::HashMap,
	fs::File,
};

use crate::parser::config::ColumnType;

/// The column named {0} is of type {1}.
pub type ColumnData = (String, ColumnType);

/// A [`HashMap`] of [`ColumnData`] mapped to [`Vec`]tors of [`String`]s. The
/// [`Vec`]tor contains the data that should go in the column the
/// [`ColumnData`] represents.
pub type GeneratorData = HashMap< ColumnData, Vec<String> >;

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
