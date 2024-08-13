use thiserror::Error;
use std::collections::HashMap;

pub mod registrars;

#[derive(Debug, Error)]
pub enum RegistryError<E> {
	#[error("Duplicate item '{0}'")]
	DuplicateItem(String),
	#[error("Unknown creator '{0}'")]
	UnknownCreator(String),
	#[error("Error while creating: {0}")]
	CreationError(E),
	#[error("An unknown error occurred: {0}")]
	Unknown(String),
}

type CreateFn<R, D, E> = fn(data: &D) -> Result<R, E>;

pub struct Registry<R, D, E> {
	creation_data: D,
	creators: HashMap< String, CreateFn<R, D, E> >,
	items: HashMap<String, R>,
}

impl<R, D, E> Registry<R, D, E> {
	pub fn new(creation_data: D) -> Self {
		Self {
			creation_data,
			creators: HashMap::new(),
			items: HashMap::new(),
		}
	}

	pub fn get(&mut self, name: impl ToString) -> Result< &mut R, RegistryError<E> > {
		let name = name.to_string();

		if self.items.contains_key(&name) {
			let item = self.items.get_mut(&name).unwrap();

			return Ok(item);
		}

		// the item doesn't exist yet, so create it
		let creator = self.creators.get(&name);

		if creator.is_none() {
			return Err( RegistryError::UnknownCreator(name) );
		}

		let creator = creator.unwrap();

		let item = creator(&self.creation_data)
			.map_err( |e| RegistryError::CreationError(e) )?;

		self.items.insert( name.clone(), item );

		let item = self.items.get_mut(&name)
			.ok_or_else( || RegistryError::Unknown( "Item wasn't found after creating it".to_string() ) )?;

		Ok(item)
	}

	pub fn register(&mut self, name: impl ToString, creator: CreateFn<R, D, E>) -> Result< (), RegistryError<E> > {
		let name = name.to_string();

		if self.creators.contains_key(&name) {
			return Err( RegistryError::DuplicateItem(name) );
		}

		self.creators.insert(name, creator);

		Ok(())
	}
}
