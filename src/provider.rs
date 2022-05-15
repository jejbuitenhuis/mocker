use std::{error::Error, collections::HashMap};

#[derive(Debug)]
pub enum ProviderError {
	/// Used when a provider is already registered under the name {0}
	AlreadyRegistered(String),
	Unknown( Box<dyn Error> ),
}

pub trait ProviderImpl {
	/// Used to create a new provider.
	fn new() -> Self where Self: Sized;
	
	/// Gets called one time, when initializing the providers. This method can
	/// be used to, for example, initialize a list of items that `provide()`
	/// can return.
	///
	/// # Params
	///
	/// - `row_count` The amount of rows to generate for every column.
	fn init(&mut self, row_count: u64) {}

	/// Gets called before a new table is filled. This method can be used to,
	/// for example, reset a counter used by `provide()`.
	fn reset(&mut self) {}

	/// Gets called every time a row is created. Should return the item for the
	/// cell in the row as a [`String`].
	fn provide(&mut self) -> Result<String, ProviderError>;
}

pub struct ProviderRegistry {
	providers: HashMap< String, Box<dyn ProviderImpl> >,
}

impl ProviderRegistry {
	pub fn new() -> ProviderRegistry {
		ProviderRegistry {
			providers: HashMap::new(),
		}
	}

	pub fn get(&mut self, name: impl ToString) -> Option< &mut Box<dyn ProviderImpl> > {
		self.providers.get_mut( &name.to_string() )
	}

	pub fn register(&mut self, name: String, provider: impl ProviderImpl + 'static) -> Result<(), ProviderError> {
		// TODO: Switch to https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.try_insert
		// when it is in stable
		if self.providers.get(&name).is_some() {
			return Err( ProviderError::AlreadyRegistered(name) );
		}

		self.providers.insert( name, Box::new(provider) );

		Ok(())
	}

	pub fn init_providers(&mut self, row_count: u64) {
		for (_, provider) in &mut self.providers {
			provider.init(row_count);
		}
	}
}
