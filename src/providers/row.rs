use crate::{
	provider::{
		ProviderCreationData,
		ProviderImpl,
		ProviderError,
	},
	parser::config::Argument,
};

pub struct RowProvider {
	curr_count: usize,
}

impl ProviderImpl for RowProvider {
	fn new(data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( RowProvider {
			curr_count: 1,
		} )
	}

	fn reset(&mut self, _arguments: &Vec<Argument>) -> Result<(), ProviderError> {
		self.curr_count = 1;

		Ok(())
	}

	fn provide(&mut self) -> Result<String, ProviderError> {
		let temp = self.curr_count;

		self.curr_count += 1;

		Ok( temp.to_string() )
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const CREATION_DATA: ProviderCreationData = ProviderCreationData { row_count: 1000 };

	#[test]
	fn test_provide_should_return_1() -> Result<(), ProviderError> { // {{{
		let mut sut = RowProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![] )?;

		let result = sut.provide()?;

		assert_eq!( "1".to_string(), result );

		Ok(())
	} // }}}

	#[test]
	fn test_provide_should_return_5() -> Result<(), ProviderError> { // {{{
		let mut sut = RowProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![] )?;

		// run 4 times, so the next provide should return "5"
		for _ in 0..4 {
			sut.provide()?;
		}

		let result = sut.provide()?;

		assert_eq!( "5".to_string(), result );

		Ok(())
	} // }}}

	#[test]
	fn test_reset_resets_counter() -> Result<(), ProviderError> { // {{{
		let mut sut = RowProvider::new(&CREATION_DATA)?;

		sut.reset( &vec![] )?;

		// run 4 times, so the next provide should return "5"
		for _ in 0..4 {
			sut.provide()?;
		}

		sut.reset( &vec![] )?;

		let result = sut.provide()?;

		assert_eq!( "1".to_string(), result );

		Ok(())
	} // }}}
}
