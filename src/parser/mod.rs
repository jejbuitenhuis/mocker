use std::{fs, path::{Path, PathBuf}};

use crate::{
	parser::{
		errors::ParserError,
		config::{
			Config, Column, Provider, ColumnType, Constraint, Table,
		},
		pointer::Pointer,
	},
};

pub mod config;
pub mod errors;
mod pointer;

const CHAR_CONSTRAINT: char = '$';
const CHAR_PROVIDER: char = '#';
const KEYWORD_TABLE: &str = "table";

pub struct Parser {
	config: Config,
	pointer: Pointer,
}

impl Parser {
	pub fn new(file: PathBuf) -> Result<Parser, ParserError> { // {{{
		let file_content = fs::read_to_string(file)
			.map_err( |e| ParserError::FileError( e.to_string() ) )?;

		Ok(Parser {
			config: Config::new(),
			pointer: Pointer::new(file_content),
		})
	} // }}}

	fn parse_whitespace(&mut self) -> Result<(), ParserError> { // {{{
		while self.pointer.peek()?.is_whitespace() {
			self.pointer.next()?;
		}

		Ok(())
	} // }}}

	fn parse_constraint(&mut self) -> Result<Constraint, ParserError> { // {{{
		let constraint_name = self.pointer.next_until(|c| c == &'(')?;

		// skip `(`
		self.pointer.next()?;

		let constraint_arguments = self.pointer.next_until(|c| c == &')')?
			.split_terminator(',')
			.map(str::trim)
			.map(str::to_string)
			.collect::< Vec<_> >();

		// skip `)`
		self.pointer.next()?;

		Ok( Constraint::try_from(constraint_name, constraint_arguments)? )
	} // }}}

	fn parse_provider(&mut self) -> Result<Provider, ParserError> { // {{{
		let constraint_name = self.pointer.next_until(|c| c == &'(')?;

		// skip `(`
		self.pointer.next()?;

		let constraint_arguments = self.pointer.next_until(|c| c == &')')?
			.split_terminator(',')
			.map(str::trim)
			.map(str::to_string)
			.collect::< Vec<_> >();

		// skip `)`
		self.pointer.next()?;

		Ok( Provider::from(constraint_name, constraint_arguments)? )
	} // }}}

	fn parse_columns(&mut self) -> Result<Vec<Column>, ParserError> { // {{{
		let mut result = vec![];

		self.parse_whitespace()?;

		let next = self.pointer.next()?;

		if next != '{' {
			return Err( ParserError::Unexpected(
				next.to_string(),
				"{".to_string(),
			) );
		}

		// loop for parsing a whole column, so from the start (column name) to
		// the end (a `,` or `}`)
		loop {
			self.parse_whitespace()?;

			let column_name = self.pointer.next_until( |c| c.is_whitespace() )?;

			self.parse_whitespace()?;

			let column_type = ColumnType::try_from(
				self.pointer.next_until( |c| c.is_whitespace() )?
			)?;

			self.parse_whitespace()?;

			let mut column_constraints: Vec<Constraint> = vec![];
			let mut column_provider: Option<Provider> = None;
			let mut char = self.pointer.next()?;

			// loop for parsing the constraints and provider. It loops until it
			// gets to the end of the column or the end of the table
			// definition (a `}`)
			loop {
				match char {
					'}' | ',' => break,
					c if c.is_whitespace() => {}, // skip to next character

					CHAR_CONSTRAINT => column_constraints.push( self.parse_constraint()? ),

					// only one provider is allowed
					CHAR_PROVIDER if column_provider.is_some() => return Err(ParserError::MultipleProviders),
					CHAR_PROVIDER if column_provider.is_none() =>
						column_provider = Some( self.parse_provider()? ),

					c => {
						return Err( ParserError::Unexpected(
							c.to_string(),
							"¯\\_(ツ)_/¯".to_string()
						) );
					}
				}

				char = self.pointer.next()?;
			}

			let column_provider = column_provider.ok_or(ParserError::NoProvider)?;

			let mut column = Column::new(column_name, column_type, column_provider);

			for constraint in column_constraints {
				column.add_constraint(constraint);
			}

			result.push(column);

			// a `,` or `}` was found in the loop above, but we don't know
			// which one. If it is the `}`, we break out of the loop because we
			// found all of the columns. If it wasn't a `{`, we can assume it
			// was a `,` (we filter all other characters above) and we can look
			// for the next column
			if char == '}' {
				break;
			}
		}

		Ok(result)
	} // }}}

	fn parse_table(&mut self) -> Result<Option<Table>, ParserError> { // {{{
		match self.parse_whitespace() {
			Err(e) => match e {
				ParserError::EOF => return Ok(None), // no more tables follow
				_ => return Err(e),
			},
			_ => (),
		};

		// "table" keyword
		let keyword = match self.pointer.next_multiple( KEYWORD_TABLE.len() ) {
			Ok(k) => k,
			Err(e) => match e {
				ParserError::EOF => return Ok(None), // no more tables follow
				_ => return Err(e),
			},
		};

		if keyword != KEYWORD_TABLE {
			return Err( ParserError::Unexpected( keyword, KEYWORD_TABLE.to_string() ) )
		}

		self.parse_whitespace()?;

		let table_name = self.pointer.next_until( |c| c.is_whitespace() || c == &'{' )?;
		let mut table = Table::new(table_name);
		let columns = self.parse_columns()?;

		for column in columns {
			table.add_column(column);
		}

		Ok( Some(table) )
	} // }}}

	pub fn parse(&mut self) -> Result<&Config, ParserError> { // {{{
		loop {
			let table = self.parse_table()?;

			if table.is_none() {
				break;
			}

			// never panics, because we checked above
			self.config.add_table( table.unwrap() );
		}

		Ok(&self.config)
	} // }}}
}
