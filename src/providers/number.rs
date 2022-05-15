use rand::{prelude::ThreadRng, Rng};

use crate::provider::{ProviderImpl, ProviderError};

pub struct NumberProvider {
	rng: ThreadRng,
	min: i64,
	max: i64,
}

impl ProviderImpl for NumberProvider {
	fn new() -> Self {
		NumberProvider {
			rng: rand::thread_rng(),
			min: 0,
			max: i64::MAX,
		}
	}

	fn reset(&mut self, arguments: &Vec<String>) -> Result<(), ProviderError> {
		if let Some(min) = arguments.get(0) {
			self.min = min.parse()
				.map_err( |_| ProviderError::UnexpectedArgument(
					arguments[0].clone(),
					"Number".to_string()
				) )?;
		} else {
			self.min = 0;
		}

		if let Some(max) = arguments.get(1) {
			self.max = max.parse()
				.map_err( |_| ProviderError::UnexpectedArgument(
					arguments[1].clone(),
					"Number".to_string()
				) )?;
		} else {
			self.max = i64::MAX;
		}

		Ok(())
	}

	fn provide(&mut self) -> Result<String, ProviderError> {
		Ok( self.rng.gen_range(self.min..=self.max).to_string() )
	}
}
