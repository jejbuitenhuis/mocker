use std::fs;

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

pub struct Parser {
	config: Config,
	pointer: Pointer,
}

impl Parser {
	pub fn new(file_path: String) -> Result<Parser, ParserError> { // {{{
		let file_content = fs::read_to_string(file_path)
			.map_err( |e| ParserError::FileError( e.to_string() ) )?;

		Ok(Parser {
			config: Config::new(),
			pointer: Pointer::new(file_content),
		})
	} // }}}

	fn parse_whitespace(&mut self) -> Result<(), ParserError> { // {{{
		loop {
			let next = self.pointer.peek()?;

			// a ',' acts as a line break in our language
			if !( next.is_whitespace() || next == &',' ) {
				break;
			}

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

		match self.parse_whitespace() {
			Err(e) => match e {
				ParserError::EOF => return Ok(result),
				_ => return Err(e),
			},
			_ => (),
		};

		let next = self.pointer.next()?;

		if next != '{' {
			return Err( ParserError::Unexpected(
				next.to_string(),
				"{".to_string(),
			) );
		}

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

			loop {
				match char {
					'}' | ',' => break,
					c if c.is_whitespace() => {}, // skip to next character
					'$' => {
						column_constraints.push( self.parse_constraint()? )
					},
					'#' if column_provider.is_some() => return Err(ParserError::MultipleProviders),
					'#' if column_provider.is_none() => {
						column_provider = Some( self.parse_provider()? )
					},
					c => {
						println!("Found a weird char:{}", c);
						return Err( ParserError::Unexpected( c.to_string(), "¯\\_(ツ)_/¯".to_string() ) );
					}
				}

				char = self.pointer.next()?;
			}

			let column_provider = column_provider.ok_or( ParserError::NoProvider )?;
			
			let mut column = Column::new(column_name, column_type, column_provider);

			for constraint in column_constraints {
				column.add_constraint(constraint);
			}

			result.push(column);

			// a `,` or `}` was found in the loop above, but we don't know
			// which one if it is the `}`, we break out of the loop because we
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
				ParserError::EOF => return Ok(None),
				_ => return Err(e),
			},
			_ => (),
		};

		// "table" keyword
		let keyword = match self.pointer.next_multiple(5) {
			Ok(k) => k,
			Err(e) => match e {
				ParserError::EOF => return Ok(None), // no more tables present
				_ => return Err(e),
			},
		};

		if keyword != "table" {
			return Err( ParserError::Unexpected( keyword, "table".to_string() ) )
		}

		self.parse_whitespace()?;

		let table_name = self.pointer.next_until( |c| c.is_whitespace() )?;

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
