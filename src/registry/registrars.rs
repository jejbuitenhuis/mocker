use crate::{
	arguments::Args,
	provider::{
		ProviderCreationData,
		ProviderError,
		ProviderImpl,
	},
	providers::{
		first_name::FirstNameProvider,
		gender::GenderProvider,
		random::RandomProvider,
		number::NumberProvider,
		row::RowProvider,
	},
	generator::{
		GeneratorCreationData,
		GeneratorError,
		GeneratorImpl,
	},
	generators::tsql::TsqlGenerator,
	registry::Registry,
};

pub fn register_providers(args: &Args) -> Registry< Box<dyn ProviderImpl>, ProviderCreationData, ProviderError> {
	let creation_data = ProviderCreationData {
		row_count: args.row_count,
	};

	let mut registry: Registry< Box<dyn ProviderImpl>, _, ProviderError>
		= Registry::new(creation_data);

	registry.register(
		"row",
		|args| Ok( Box::new( RowProvider::new(args)? ) ),
	).unwrap();
	registry.register(
		"number",
		|args| Ok( Box::new( NumberProvider::new(args)? ) ),
	).unwrap();
	registry.register(
		"gender",
		|args| Ok( Box::new( GenderProvider::new(args)? ) ),
	).unwrap();
	registry.register(
		"random",
		|args| Ok( Box::new( RandomProvider::new(args)? ) ),
	).unwrap();
	registry.register(
		"first_name",
		|args| Ok( Box::new( FirstNameProvider::new(args)? ) ),
	).unwrap();

	registry
}

pub fn register_generators<'a>(args: &Args) -> Registry< Box<dyn GeneratorImpl>, GeneratorCreationData, GeneratorError> {
	let creation_data = GeneratorCreationData {};

	let mut registry: Registry< Box<dyn GeneratorImpl>, _, GeneratorError>
		= Registry::new(creation_data);

	registry.register(
		"tsql",
		|args| Ok( Box::new( TsqlGenerator::new()? ) ),
	).unwrap();

	registry
}
