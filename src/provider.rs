#[derive(PartialEq, Debug)]
pub enum ProviderError {
	/// Used when a provider is already registered under the name {0}
	AlreadyRegistered(String),

	/// Used when an argument given to a [`ProviderImpl`] is not correct.
	/// `Unexpected {0}, expected {1}`
	UnexpectedArgument(String, String),

	/// Unknown error {0}
	Unknown(String),

	/// Used when the provider {0} is not registered in the [`ProviderRegistry`]
	UnknownProvider(String),

	/// Used when too few arguments are given ({0}), but {1} were expected
	TooFewArguments(usize, usize),

	/// Used when too many arguments are given ({0}), but {1} were expected
	TooManyArguments(usize, usize),
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
	fn reset(&mut self, arguments: &Vec<String>) -> Result<(), ProviderError> {
		Ok(())
	}

	/// Gets called every time a row is created. Should return the item for the
	/// cell in the row as a [`String`].
	fn provide(&mut self) -> Result<String, ProviderError>;
} // }}}
