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

// FIXME: doesn't fail when using `/tmp/test.sql`. Why are we even checking this?
fn validate_config_path(path: &str) -> Result<(), String> { // {{{
	let path = Path::new(path);

	if !path.exists() {
		return Err( "file does not exist.".to_string() );
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

	/// The path to the output file
	#[clap(short, long, default_value = "mock_data.sql")]
	pub output: String,

	/// The path to the config file
	#[clap(validator = validate_config_path)]
	pub config: PathBuf,
}
