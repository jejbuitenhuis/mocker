use core::fmt;

use crate::parser::errors::ParserError;

pub struct Pointer {
	content: Vec<char>,
}

impl Pointer {
	pub fn new(content: String) -> Pointer { // {{{
		Pointer {
			content: content.chars().rev().collect::< Vec<char> >(),
		}
	} // }}}

	/// Look at what the next character is, without removing it from the content
	pub fn peek(&self) -> Result<&char, ParserError> { // {{{
		self.content.last()
			.ok_or(ParserError::EOF)
	} // }}}

	/// Get the next character and remove it from the content
	pub fn next(&mut self) -> Result<char, ParserError> { // {{{
		self.content.pop()
			.ok_or(ParserError::EOF)
	} // }}}

	/// Get the next `count` characters and remove them from the content
	pub fn next_multiple(&mut self, count: usize) -> Result<String, ParserError> { // {{{
		let mut iteration = count;
		let mut result = String::new();

		while iteration > 0 {
			result.push( self.next()? );

			iteration -= 1;
		}

		Ok(result)
	} // }}}

	/// Gets the next characters until `to_find` returns true and removes them
	/// from the content
	pub fn next_until<F>(&mut self, to_find: F) -> Result<String, ParserError> // {{{
		where F: Fn(&char) -> bool
	{
		let mut result = String::new();

		while !to_find( self.peek()? ) {
			result.push( self.next()? );
		}

		Ok(result)
	} // }}}
}

impl fmt::Display for Pointer { // {{{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let content = self.content.iter().rev().collect::<String>();

		write!(f, "Pointer {{ {:?} }}", content)
	}
} // }}}
