#[cfg(test)] use lazy_static::lazy_static;
use rand::{
	prelude::Rng,
	RngCore,
};
#[cfg(test)]
use rand::rngs::mock::StepRng;
use std::{
	path::PathBuf,
	fs,
};
#[cfg(test)] use tempfile::{
	TempDir,
	tempdir,
};

use crate::{
	generator::CellValue,
	provider::{
		ProviderCreationData,
		ProviderImpl,
		ProviderError,
	},
};

#[cfg(test)]
lazy_static! {
	static ref FILE: TempDir = tempdir().unwrap();
}

#[cfg( not(test) )]
fn get_file() -> PathBuf {
	PathBuf::from("./sources/first_names.txt")
}

#[cfg(test)]
fn get_file() -> PathBuf {
	FILE.path()
		.join("first_names.txt")
}

pub struct FirstNameProvider {
	rng: Box<dyn RngCore>,
	items: Vec<String>,
}

impl ProviderImpl for FirstNameProvider {
	fn new(_data: &ProviderCreationData) -> Result<Self, ProviderError> {
		let items = fs::read_to_string( get_file() )
			.map_err( |e| ProviderError::Unknown( e.to_string() ) )?
			.lines()
			.map(String::from)
			.collect();

		Ok( Self {
			#[cfg( not(test) )] rng: Box::new( rand::thread_rng() ),
			#[cfg(test)] rng: Box::new( StepRng::new(0, 1) ),

			items,
		} )
	}

	fn provide(&mut self) -> Result<CellValue, ProviderError> {
		let selected = self.rng.gen_range( 0..self.items.len() );

		Ok(
			CellValue::String( self.items[selected].clone() )
		)
	}
}

#[cfg(test)]
mod tests {
	use lazy_static::lazy_static;
	use std::fs;

	use super::*;
	use crate::parser::config::Argument;

	const CREATION_DATA: ProviderCreationData = ProviderCreationData { row_count: 1000 };
	lazy_static! {
		static ref NAMES: Vec<String> = vec![
			"Name 1".to_string(),
			"Name 2".to_string(),
			"Name 3".to_string(),
			"Name 4".to_string(),
			"Name 5".to_string(),
		];
		static ref ARGUMENTS: Vec<Argument> = NAMES.iter()
			.map( |arg| Argument::String( arg.clone() ) )
			.collect();
	}

	fn setup() {
		let content = NAMES.join("\n");
		let file = get_file();

		fs::write(file, content)
			.unwrap();
	}

	#[test]
	fn test_provide_should_return_the_first_item_correctly_formatted() -> Result<(), ProviderError> { // {{{
		setup();

		let mut sut = FirstNameProvider::new(&CREATION_DATA)?;

		sut.reset(&ARGUMENTS)?;

		let result = sut.provide()?;

		assert_eq!(
			CellValue::String( "Name 1".to_string() ),
			result,
		);

		Ok(())
	} // }}}
}
