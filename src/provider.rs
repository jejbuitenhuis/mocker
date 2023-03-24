use std::collections::HashMap;

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

pub trait ProviderImpl { // {{{
	/// Used to create a new provider. Can also be used to, for example,
	/// initialize a list of items that `provide()` can return.
	///
	/// # Arguments
	///
	/// - `row_count` The amount of rows to generate for every column.
	fn new(row_count: usize) -> Result<Self, ProviderError>
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

pub type ProviderProvider = fn(row_count: usize)
	-> Result< Box<dyn ProviderImpl>, ProviderError>;

pub struct ProviderRegistry {
	row_count: usize,
	providers: HashMap<String, ProviderProvider>,
	created_providers: HashMap< String, Box<dyn ProviderImpl> >,
}

impl ProviderRegistry { // {{{
	pub fn new(row_count: usize) -> Self {
		Self {
			row_count,
			providers: HashMap::new(),
			created_providers: HashMap::new(),
		}
	}

	pub fn get(&mut self, name: impl ToString) -> Result< &mut Box<dyn ProviderImpl>, ProviderError > {
		let name = name.to_string();
		let provider_provider = self.providers.get_mut(&name);

		if provider_provider.is_none() {
			return Err( ProviderError::UnknownProvider(name) );
		}

		if self.created_providers.get(&name).is_none() {
			let provider = provider_provider.unwrap()(self.row_count)?;

			self.created_providers.insert( name.clone(), provider );
		}

		let provider = self.created_providers.get_mut(&name);

		if provider.is_none() {
			return Err( ProviderError::UnknownProvider(name) );
		}

		Ok( provider.unwrap() )
	}

	pub fn register(&mut self, name: impl ToString, provider: ProviderProvider) -> Result<(), ProviderError> {
		let name = name.to_string();

		// TODO: Switch to https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.try_insert
		// when it is in stable
		if self.providers.get(&name).is_some() {
			return Err( ProviderError::AlreadyRegistered(name) );
		}

		self.providers.insert(name, provider);

		Ok(())
	}
} // }}}
