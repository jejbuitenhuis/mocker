use std::path::{PathBuf, Path};

use clap::Parser as CliParser;

const OUTPUT_TYPES_ALLOWED: &'static [&'static str] = &[
	"tsql",
	"csv",
	"json",
	"xml",
];

fn validate_output_type(r#type: &str) -> Result<(), String> { // {{{
	let allowed = OUTPUT_TYPES_ALLOWED.into_iter()
		.fold(false, |acc, curr| acc || curr == &r#type);

	if !allowed {
		let allowed_types = OUTPUT_TYPES_ALLOWED.into_iter()
			.map( |t| t.to_string() )
			.collect::< Vec<String> >()
			.join(", ");

		return Err( format!(
			"valid output types are {}.",
			allowed_types,
		) );
	}

	Ok(())
} // }}}

fn validate_path_is_folder(path: &str) -> Result<(), String> { // {{{
	validate_path_exists(path)?;

	let path = Path::new(path);

	if !path.is_dir() {
		return Err( "path is not a directory.".to_string() );
	}

	Ok(())
} // }}}

fn validate_path_exists(path: &str) -> Result<(), String> { // {{{
	let path = Path::new(path);

	if !path.exists() {
		return Err( "file/directory does not exist.".to_string() );
	}

	Ok(())
} // }}}

#[derive(CliParser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	/// The amount of rows to create for each table
	#[clap(short = 'c', long, default_value_t = 1000)]
	pub row_count: usize,

	/// The output type to use when generating the mock data
	#[clap(short, long, default_value = "tsql", validator = validate_output_type)]
	pub r#type: String,

	/// The path to the output folder
	#[clap(short, long, validator = validate_path_is_folder)]
	pub output: String,

	/// The path to the config file
	#[clap(validator = validate_path_exists)]
	pub config: PathBuf,
}
