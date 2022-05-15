use std::path::{PathBuf, Path};

use clap::Parser as CliParser;

fn validate_config_path(path: &str) -> Result<(), String> {
	let path = Path::new(path);

	if !path.exists() {
		return Err( "file does not exist".to_string() );
	}

	Ok(())
}

#[derive(CliParser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	/// The amount of rows to create for each table
	#[clap(short, long, default_value_t = 1000)]
	pub row_count: u64,

	/// The path to the config file
	#[clap(validator = validate_config_path)]
	pub config: PathBuf,
}
