#[derive(Debug)]
pub enum ParserError {
	/// An unknown error
	Unknown(String),
	FileError(String),
	/// Unexpected End Of File when parsing config
	EOF,
	/// Unexpected `{0}`, expected `{1}`
	Unexpected(String, String),
	/// No provider is assigned to a column
	NoProvider,
	/// Only one provider per table column allowed
	MultipleProviders,
	/// Too few arguments. Got `{0}`, expected `{1}`
	TooFewArguments(usize, usize),
	/// Too many arguments. Got `{0}`, expected `{1}`
	TooManyArguments(usize, usize),
}
