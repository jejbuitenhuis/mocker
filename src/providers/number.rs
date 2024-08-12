use rand::{
	prelude::Rng,
	RngCore
};
#[cfg(test)]
use rand::rngs::mock::StepRng;

use crate::{
	generator::CellValue,
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

impl NumberProvider {
	fn parse_value_from_arg(&self, arg: &Argument) -> Result<i64, ProviderError> {
		if let Argument::Int(value) = arg {
			return Ok( value.clone() );
		}

		Err( ProviderError::UnexpectedArgument(
			arg.to_string(),
			"int".to_string(),
		) )
	}
}

impl ProviderImpl for NumberProvider {
	#[cfg( not(test) )]
	fn new(_data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( NumberProvider {
			rng: Box::new( rand::thread_rng() ),
			min: 0,
			max: i64::MAX,
		} )
	}

	#[cfg(test)]
	fn new(_data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( Self {
			rng: Box::new( StepRng::new(0, 1) ),
			min: 0,
			max: i64::MAX,
		} )
	}

	fn reset(&mut self, arguments: &Vec<Argument>) -> Result<(), ProviderError> {
		if arguments.is_empty() {
			self.min = 0;
			self.max = i64::MAX;

			return Ok(());
		}

		if let Some(min) = arguments.get(0) {
			self.min = self.parse_value_from_arg(min)?;
		}

		if let Some(max) = arguments.get(1) {
			self.max = self.parse_value_from_arg(max)?;
		}

		Ok(())
	}

	fn provide(&mut self) -> Result<CellValue, ProviderError> {
		let value = self.rng.gen_range(self.min..=self.max);

		Ok( CellValue::Int(value) )
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

		assert_eq!( CellValue::Int(0), result );

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_minimum_to_10() -> Result<(), ProviderError> { // {{{
		let expected = 10;
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ Argument::Int(expected) ] )?;

		assert_eq!(expected, sut.min);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_maximum_to_10() -> Result<(), ProviderError> { // {{{
		let expected = 10;
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ Argument::Int(5), Argument::Int(expected) ] )?;

		assert_eq!(expected, sut.max);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_minimum_to_15_when_maximum_is_also_set() -> Result<(), ProviderError> { // {{{
		let expected = 15;
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ Argument::Int(expected), Argument::Int(20) ] )?;

		assert_eq!(expected, sut.min);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_return_error_when_no_number_is_given_to_minimum() -> Result<(), ProviderError> { // {{{
		let arg = "abc".to_string();
		let expected = Err( ProviderError::UnexpectedArgument( arg.clone(), "int".to_string() ) );
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		let result = sut.reset( &vec![ Argument::String(arg) ] );

		assert_eq!(expected, result);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_return_error_when_no_number_is_given_to_maximum() -> Result<(), ProviderError> { // {{{
		let arg = "abc".to_string();
		let expected = Err( ProviderError::UnexpectedArgument( arg.clone(), "int".to_string() ) );
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		let result = sut.reset( &vec![
			Argument::Int(5),
			Argument::String(arg),
		] );

		assert_eq!(expected, result);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_min_to_default_when_no_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ Argument::Int(5), Argument::Int(10) ] )?;

		sut.reset( &vec![] )?;

		assert_eq!(0, sut.min);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_max_to_default_when_no_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = NumberProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ Argument::Int(5), Argument::Int(10) ] )?;

		sut.reset( &vec![] )?;

		assert_eq!(i64::MAX, sut.max);

		Ok(())
	} // }}}
}
