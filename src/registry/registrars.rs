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
	registry::{ Registry, RegistryError },
};

pub fn register_providers(args: &Args) -> Result<
	Registry< Box<dyn ProviderImpl>, ProviderCreationData, ProviderError>,
	RegistryError<ProviderError>,
> {
	let creation_data = ProviderCreationData {
		row_count: args.row_count,
	};

	let mut registry: Registry< Box<dyn ProviderImpl>, _, _ >
		= Registry::new(creation_data);

	registry.register(
		"row",
		|args| Ok( Box::new( RowProvider::new(args)? ) ),
	)?;
	registry.register(
		"number",
		|args| Ok( Box::new( NumberProvider::new(args)? ) ),
	)?;
	registry.register(
		"gender",
		|args| Ok( Box::new( GenderProvider::new(args)? ) ),
	)?;
	registry.register(
		"random",
		|args| Ok( Box::new( RandomProvider::new(args)? ) ),
	)?;
	registry.register(
		"first_name",
		|args| Ok( Box::new( FirstNameProvider::new(args)? ) ),
	)?;

	Ok(registry)
}

pub fn register_generators<'a>(_args: &Args) -> Result<
	Registry< Box<dyn GeneratorImpl>, GeneratorCreationData, GeneratorError>,
	RegistryError<GeneratorError>,
> {
	let creation_data = GeneratorCreationData {};

	let mut registry: Registry< Box<dyn GeneratorImpl>, _, _ >
		= Registry::new(creation_data);

	registry.register(
		"tsql",
		|_args| Ok( Box::new( TsqlGenerator::new()? ) ),
	)?;

	Ok(registry)
}
