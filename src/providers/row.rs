use crate::provider::{ProviderImpl, ProviderError};

pub struct RowProvider {
	curr_count: u64,
}

impl ProviderImpl for RowProvider {
	fn new() -> Self {
		RowProvider {
			curr_count: 1,
		}
	}

	fn reset(&mut self, _arguments: &Vec<String>) -> Result<(), ProviderError> {
		self.curr_count = 1;

		Ok(())
	}

	fn provide(&mut self) -> Result<String, ProviderError> {
		let temp = self.curr_count;

		self.curr_count += 1;

		Ok( temp.to_string() )
	}
}
