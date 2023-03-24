use rand::{
	prelude::SliceRandom, RngCore
};
#[cfg(test)]
use rand::rngs::mock::StepRng;

use crate::provider::{ProviderImpl, ProviderError};

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
	fn new(row_count: usize) -> Result<Self, ProviderError> {
		Ok( Self {
			rng: Box::new( rand::thread_rng() ),
			long: false,
		} )
	}

	#[cfg(test)]
	fn new(row_count: usize) -> Result<Self, ProviderError> {
		Ok( Self {
			rng: Box::new( StepRng::new(0, 1) ),
			long: false,
		} )
	}

	fn reset(&mut self, arguments: &Vec<String>) -> Result<(), ProviderError> {
		if let Some(should_be_long) = arguments.get(0) {
			self.long = should_be_long.parse::<bool>()
				.map_err( |_| ProviderError::UnexpectedArgument(
					arguments[0].clone(),
					"Boolean".to_string(),
				) )?;
		} else {
			self.long = false;
		}

		Ok(())
	}

	fn provide(&mut self) -> Result<String, ProviderError> {
		let gender = GENDER_LIST.choose(&mut self.rng)
			.expect("GENDER_LIST is empty");

		if self.long {
			Ok( format!("'{}'", gender.long) )
		} else {
			Ok( format!("'{}'", gender.short) )
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const ROW_COUNT: usize = 1000;

	#[test]
	fn test_provide_returns_gender() -> Result<(), ProviderError> { // {{{
		let expected = format!( "'{}'", GENDER_LIST[0].short );
		let mut sut = GenderProvider::new(ROW_COUNT)?;

		sut.reset( &vec![] )?;

		assert_eq!( expected, sut.provide()? );

		Ok(())
	} // }}}

	#[test]
	fn test_provide_returns_long_gender() -> Result<(), ProviderError> { // {{{
		let expected = format!( "'{}'", GENDER_LIST[0].long );
		let mut sut = GenderProvider::new(ROW_COUNT)?;

		sut.reset( &vec![ "true".to_string() ] )?;

		assert_eq!( expected, sut.provide()? );

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_set_long_to_default_when_no_arguments_are_given() -> Result<(), ProviderError> { // {{{
		let mut sut = GenderProvider::new(ROW_COUNT)?;

		sut.reset( &vec![ "true".to_string() ] )?;

		sut.reset( &vec![] )?;

		assert_eq!(false, sut.long);

		Ok(())
	} // }}}

	#[test]
	fn test_reset_should_return_error_when_no_boolean_is_given_to_long() -> Result<(), ProviderError> { // {{{
		let arg = "123".to_string();
		let expected = Err( ProviderError::UnexpectedArgument( arg.clone(), "Boolean".to_string() ) );
		let mut sut = GenderProvider::new(ROW_COUNT)?;

		let result = sut.reset( &vec![ arg.clone() ] );

		assert_eq!(expected, result);

		Ok(())
	} // }}}
}
