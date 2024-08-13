use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
	/// Syntax error
	#[error("syntax error:\n{0}")]
	SyntaxError(String),
	/// ???
	#[error("")]
	FileError(String),
	/// Unexpected End Of File when parsing config
	#[error("unexpected Eof Of File")]
	EOF,
	/// Unexpected `{0}`, expected `{1}`
	#[error("unexpected '{0}', expected {1}")]
	Unexpected(String, String),
	// TODO: Add column name?
	/// No provider is assigned to a column
	#[error("no provider assigned to column")]
	NoProvider,
	// TODO: Add "(found X)" to message?
	/// Only one provider per table column allowed
	#[error("only one provider per column is allowed")]
	MultipleProviders,
	/// Too few arguments. Got `{0}`, expected `{1}`
	#[error("{0} arguments were given, but at least {1} were expected")]
	TooFewArguments(usize, usize),
	/// Too many arguments. Got `{0}`, expected `{1}`
	#[error("{0} arguments were given, but at most {1} were expected")]
	TooManyArguments(usize, usize),
	/// An unknown error
	#[error("an unknown error occurred: {0}")]
	Unknown(String),
}
