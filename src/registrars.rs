use crate::{
	provider::{
		ProviderRegistry,
		ProviderImpl
	},
	providers::{
		row::RowProvider,
		number::NumberProvider,
		gender::GenderProvider,
	},
	generator::{
		GeneratorImpl,
		GeneratorRegistry,
	},
	generators::tsql::TsqlGenerator,
};

pub fn register_providers(registry: &mut ProviderRegistry) {
	registry.register( "row".to_string(), RowProvider::new() ).unwrap();
	registry.register( "number".to_string(), NumberProvider::new() ).unwrap();
	registry.register( "gender".to_string(), GenderProvider::new() ).unwrap();
}

pub fn register_generators<'a>(registry: &mut GeneratorRegistry) {
	registry.register( "tsql", TsqlGenerator::new() ).unwrap();
}
