use rand::{
	prelude::Rng,
	RngCore
};
#[cfg(test)]
use rand::rngs::mock::StepRng;

use crate::{
	provider::{
		ProviderCreationData,
		ProviderImpl,
		ProviderError,
	},
	parser::config::Argument,
};

pub struct NumberProvider {
	rng: Box<dyn RngCore>,
	min: i64,
	max: i64,
}

impl ProviderImpl for NumberProvider {
	#[cfg( not(test) )]
	fn new(data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( NumberProvider {
			rng: Box::new( rand::thread_rng() ),
			min: 0,
			max: i64::MAX,
		} )
	}

	#[cfg(test)]
	fn new(data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( Self {
			rng: Box::new( StepRng::new(0, 1) ),
			min: 0,
			max: i64::MAX,
		} )
	}

	fn reset(&mut self, arguments: &Vec<Argument>) -> Result<(), ProviderError> {
		todo!()

		// if let Some(min) = arguments.get(0) {
			// self.min = min.parse::<i64>()
				// .map_err( |_| ProviderError::UnexpectedArgument(
					// arguments[0].clone(),
					// "Number".to_string()
				// ) )?;
		// } else {
			// self.min = 0;
		// }

		// if let Some(max) = arguments.get(1) {
			// self.max = max.parse::<i64>()
				// .map_err( |_| ProviderError::UnexpectedArgument(
					// arguments[1].clone(),
					// "Number".to_string()
				// ) )?;
		// } else {
			// self.max = i64::MAX;
		// }

		// Ok(())
	}

	fn provide(&mut self) -> Result<String, ProviderError> {
		Ok( self.rng.gen_range(self.min..=self.max).to_string() )
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const CREATION_DATA: ProviderCreationData = ProviderCreationData { row_count: 1000 };

	#[test]
	fn test_provide_should_return_a_number() -> Result<(), ProviderError> { // {{{
		let mut sut = NumberProvider::new(&CREATION_DATA)?;
		sut.reset( &vec![] )?;

		let result = sut.provide()?;

		assert_eq!( "0".to_string(), result );

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_minimum_to_10() -> Result<(), ProviderError> { // {{{
		let expected = 10;
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ expected.to_string() ] )?;

		assert_eq!(expected, sut.min);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_maximum_to_10() -> Result<(), ProviderError> { // {{{
		let expected = 10;
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ 5.to_string(), expected.to_string() ] )?;

		assert_eq!(expected, sut.max);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_minimum_to_15_when_maximum_is_also_set() -> Result<(), ProviderError> { // {{{
		let expected = 15;
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ expected.to_string(), 20.to_string() ] )?;

		assert_eq!(expected, sut.min);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_return_error_when_no_number_is_given_to_minimum() -> Result<(), ProviderError> { // {{{
		let arg = "abc".to_string();
		let expected = Err( ProviderError::UnexpectedArgument( arg.clone(), "Number".to_string() ) );
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		let result = sut.reset( &vec![ arg.clone() ] );

		assert_eq!(expected, result);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_return_error_when_no_number_is_given_to_maximum() -> Result<(), ProviderError> { // {{{
		let arg = "abc".to_string();
		let expected = Err( ProviderError::UnexpectedArgument( arg.clone(), "Number".to_string() ) );
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		let result = sut.reset( &vec![ 5.to_string(), arg.clone() ] );

		assert_eq!(expected, result);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_min_to_default_when_no_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ 5.to_string(), 10.to_string() ] )?;

		sut.reset( &vec![] )?;

		assert_eq!(0, sut.min);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_max_to_default_when_no_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ 5.to_string(), 10.to_string() ] )?;

		sut.reset( &vec![] )?;

		assert_eq!(i64::MAX, sut.max);

		Ok(())
	} // }}}
}
