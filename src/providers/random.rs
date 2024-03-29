#[cfg(test)] use lazy_static::lazy_static;
use rand::{
	prelude::Rng,
	RngCore,
};
#[cfg(test)]
use rand::rngs::mock::StepRng;

use crate::{
	provider::{
		ProviderCreationData,
		ProviderImpl,
	},
	ProviderError,
};

pub struct RandomProvider {
	rng: Box<dyn RngCore>,
	items: Vec<String>,
}

impl ProviderImpl for RandomProvider {
	fn new(data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( Self {
			#[cfg( not(test) )] rng: Box::new( rand::thread_rng() ),
			#[cfg(test)] rng: Box::new( StepRng::new(0, 1) ),
			items: vec![],
		} )
	}

	fn reset(&mut self, arguments: &Vec<String>) -> Result<(), ProviderError> {
		if arguments.len() < 2 {
			return Err( ProviderError::TooFewArguments( arguments.len(), 2 ) );
		}

		self.items = arguments.clone();

		Ok(())
	}

	fn provide(&mut self) -> Result<String, ProviderError> {
		let selected = self.rng.gen_range( 0..self.items.len() );

		Ok( self.items[selected].clone() )
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const CREATION_DATA: ProviderCreationData = ProviderCreationData { row_count: 1000 };
	lazy_static! {
		static ref ITEMS: Vec<String> = vec![
			"Item 1".to_string(),
			"Item 2".to_string(),
			"Item 3".to_string(),
			"Item 4".to_string(),
			"Item 5".to_string(),
		];
	}

	#[test]
	fn test_provide_should_return_the_first_item() -> Result<(), ProviderError> { // {{{
		let mut sut = RandomProvider::new(&CREATION_DATA)?;

		sut.reset(&ITEMS)?;

		let result = sut.provide()?;

		assert_eq!( "Item 1".to_string(), result );

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_give_an_error_when_too_few_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = RandomProvider::new(&CREATION_DATA)?;

		let result = sut.reset(&vec![ "Item 1".to_string() ]);

		assert_eq!(result, Err( ProviderError::TooFewArguments(1, 2) ));

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_not_give_an_error_when_too_few_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = RandomProvider::new(&CREATION_DATA)?;

		let result = sut.reset(&vec![
			"Item 1".to_string(),
			"Item 2".to_string(),
		]);

		assert_eq!( result, Ok(()) );

		Ok(())
	} // }}}
}
