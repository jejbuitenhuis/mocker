use crate::{
	provider::{
		ProviderRegistry,
		ProviderImpl
	},
	providers::{
		gender::GenderProvider,
		random::RandomProvider,
		number::NumberProvider,
		row::RowProvider,
	},
	generator::{
		GeneratorImpl,
		GeneratorRegistry,
	},
	generators::tsql::TsqlGenerator,
};

pub fn register_providers(registry: &mut ProviderRegistry) {
	registry.register( "row", RowProvider::new() ).unwrap();
	registry.register( "number", NumberProvider::new() ).unwrap();
	registry.register( "gender", GenderProvider::new() ).unwrap();
	registry.register( "random", RandomProvider::new() ).unwrap();
}

pub fn register_generators<'a>(registry: &mut GeneratorRegistry) {
	registry.register( "tsql", TsqlGenerator::new() ).unwrap();
}
