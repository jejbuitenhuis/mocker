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
	registry.register(
		"row",
		|row_count| Ok( Box::new( RowProvider::new(row_count)? ) )
	).unwrap();
	registry.register(
		"number",
		|row_count| Ok( Box::new( NumberProvider::new(row_count)? ) )
	).unwrap();
	registry.register(
		"gender",
		|row_count| Ok( Box::new( GenderProvider::new(row_count)? ) )
	).unwrap();
	registry.register(
		"random",
		|row_count| Ok( Box::new( RandomProvider::new(row_count)? ) )
	).unwrap();
}

pub fn register_generators<'a>(registry: &mut GeneratorRegistry) {
	registry.register(
		"tsql",
		|| Ok( Box::new( TsqlGenerator::new()? ) )
	).unwrap();
}
