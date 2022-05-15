use crate::{
	provider::{
		ProviderRegistry,
		ProviderImpl
	},
	providers::{row::RowProvider, number::NumberProvider},
};

pub fn register_providers(registry: &mut ProviderRegistry) {
	registry.register( "row".to_string(), RowProvider::new() ).unwrap();
	registry.register( "number".to_string(), NumberProvider::new() ).unwrap();
}
