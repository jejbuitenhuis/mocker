use crate::{
	generator::CellValue,
	provider::{
		ProviderCreationData,
		ProviderImpl,
		ProviderError,
	},
	parser::config::Argument,
};

pub struct RowProvider {
	curr_count: u64,
}

impl ProviderImpl for RowProvider {
	fn new(_data: &ProviderCreationData) -> Result<Self, ProviderError> {
		Ok( RowProvider {
			curr_count: 1,
		} )
	}

	fn reset(&mut self, _arguments: &Vec<Argument>) -> Result<(), ProviderError> {
		self.curr_count = 1;

		Ok(())
	}

	fn provide(&mut self) -> Result<CellValue, ProviderError> {
		let temp = self.curr_count;

		self.curr_count += 1;

		Ok( CellValue::UnsignedInt(temp) )
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

		assert_eq!( CellValue::UnsignedInt(1), result );

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

		assert_eq!( CellValue::UnsignedInt(5), result );

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

		assert_eq!( CellValue::UnsignedInt(1), result );

		Ok(())
	} // }}}
}
