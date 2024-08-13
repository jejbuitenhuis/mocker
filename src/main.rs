use clap::Parser as CliParser;

use crate::{
	arguments::Args,
	mocker::Mocker,
};

mod arguments;
mod generator;
mod generators;
mod mocker;
mod parser;
mod provider;
mod providers;
mod registry;

fn main() -> anyhow::Result<()> {
	env_logger::init();

	let args = Args::parse();
	let mocker = Mocker::new(&args);

	let config = mocker.parse_config()?;

	let generated_data = mocker.generate_mock_data(config)?;

	mocker.write_mock_data(generated_data)?;

	Ok(())
}
