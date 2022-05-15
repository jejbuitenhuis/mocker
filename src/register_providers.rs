use crate::{
	provider::{
		ProviderRegistry,
		ProviderImpl
	},
	providers::row::RowProvider,
};

pub fn register_providers(registry: &mut ProviderRegistry) {
	registry.register( "row".to_string(), RowProvider::new() ).unwrap();
}
