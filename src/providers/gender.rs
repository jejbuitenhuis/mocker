use rand::{
	prelude::SliceRandom, RngCore
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

struct Gender {
	short: char,
	long: &'static str,
}

const GENDER_LIST: [Gender; 3] = [
	Gender {
		short: 'F',
		long: "FEMALE",
	},
	Gender {
		short: 'M',
		long: "MALE",
	},
	Gender {
		short: 'O',
		long: "OTHER",
	},
];

pub struct GenderProvider {
	rng: Box<dyn RngCore>,
	long: bool,
}

impl ProviderImpl for GenderProvider {
	#[cfg( not(test) )]
	fn new(_data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( Self {
			rng: Box::new( rand::thread_rng() ),
			long: false,
		} )
	}

	#[cfg(test)]
	fn new(_data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( Self {
			rng: Box::new( StepRng::new(0, 1) ),
			long: false,
		} )
	}

	fn reset(&mut self, arguments: &Vec<Argument>) -> Result<(), ProviderError> {
		if let Some(should_be_long_arg) = arguments.get(0) {
			match should_be_long_arg {
				Argument::Boolean(should_be_long) => {
					self.long = *should_be_long;
				},
				arg => return Err( ProviderError::UnexpectedArgument(
					arg.to_string(),
					"Boolean".to_string(),
				) ),
			}
		} else {
			self.long = false;
		}

		Ok(())
	}

	fn provide(&mut self) -> Result<CellValue, ProviderError> {
		let gender = GENDER_LIST.choose(&mut self.rng)
			.expect("GENDER_LIST should not be empty");

		let value = if self.long {
			gender.long.to_string()
		} else {
			gender.short.to_string()
		};

		Ok( CellValue::String( value.to_string() ) )
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const CREATION_DATA: ProviderCreationData = ProviderCreationData { row_count: 1000 };

	#[test]
	fn test_provide_returns_gender() -> Result<(), ProviderError> { // {{{
		let expected = CellValue::String( GENDER_LIST[0].short.to_string() );
		let mut sut = GenderProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![] )?;

		assert_eq!( expected, sut.provide()? );

		Ok(())
	} // }}}

	#[test]
	fn test_provide_returns_long_gender() -> Result<(), ProviderError> { // {{{
		let expected = CellValue::String( GENDER_LIST[0].long.to_string() );
		let mut sut = GenderProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ Argument::Boolean(true) ] )?;

		assert_eq!( expected, sut.provide()? );

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_long_to_default_when_no_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = GenderProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![ Argument::Boolean(true) ] )?;

		sut.reset( &vec![] )?;

		assert_eq!(false, sut.long);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_return_error_when_no_boolean_is_given_to_long() -> Result<(), ProviderError> { // {{{
		let arg = "123".to_string();
		let expected = Err( ProviderError::UnexpectedArgument( arg.clone(), "Boolean".to_string() ) );
		let mut sut = GenderProvider::new(&CREATION_DATA)?;

		let result = sut.reset( &vec![ Argument::String( arg.clone() ) ] );

		assert_eq!(expected, result);

		Ok(())
	} // }}}
}
