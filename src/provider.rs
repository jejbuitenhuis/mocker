use thiserror::Error;

use crate::{
	generator::CellValue,
	parser::config::Argument,
};

#[derive(PartialEq, Debug, Error)]
pub enum ProviderError {
	/// Used when a provider is already registered under the name {0}
	#[error("provider named '{0}' is already registered")]
	AlreadyRegistered(String),

	/// Used when an argument given to a [`ProviderImpl`] is not correct.
	/// `Unexpected {0}, expected {1}`
	#[error("unexpected argument '{0}', expected {1}")]
	UnexpectedArgument(String, String),

	/// Used when the provider {0} is not registered in the [`ProviderRegistry`]
	#[error("unknown provider '{0}'")]
	UnknownProvider(String),

	/// Used when too few arguments are given ({0}), but {1} were expected
	#[error("{0} arguments were given, but at least {1} were expected")]
	TooFewArguments(usize, usize),

	/// Used when too many arguments are given ({0}), but {1} were expected
	#[error("{0} arguments were given, but at most {1} were expected")]
	TooManyArguments(usize, usize),

	/// Unknown error {0}
	#[error("an unknown error occurred: {0}")]
	Unknown(String),
}

pub struct ProviderCreationData {
	pub row_count: usize,
}

pub trait ProviderImpl { // {{{
	/// Used to create a new provider. Can also be used to, for example,
	/// initialize a list of items that `provide()` can return.
	///
	/// # Arguments
	///
	/// - `row_count` The amount of rows to generate for every column.
	fn new(data: &ProviderCreationData) -> Result<Self, ProviderError>
		where Self: Sized;

	/// Gets called before a new table is filled. This method can be used to,
	/// for example, reset a counter used by `provide()`.
	///
	/// # Arguments
	///
	/// - `arguments` A list with arguments for the provider to use for the
	///   current column
	fn reset(&mut self, arguments: &Vec<Argument>) -> Result<(), ProviderError> {
		Ok(())
	}

	/// Gets called every time a row is created. Should return the item for the
	/// cell in the row as a [`String`].
	fn provide(&mut self) -> Result<CellValue, ProviderError>;
} // }}}
