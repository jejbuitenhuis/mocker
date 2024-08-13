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
}
