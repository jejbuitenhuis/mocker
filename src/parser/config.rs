use crate::parser::errors::ParserError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColumnType {
	/// String type with a max length of {0}
	String(usize),
	Int,
	Long,
	Float,
	Double,
}

impl TryFrom<String> for ColumnType { // {{{
	type Error = ParserError;

	fn try_from(kind: String) -> Result<ColumnType, Self::Error> {
		match kind.as_str() {
			"int" => Ok(ColumnType::Int),
			"long" => Ok(ColumnType::Long),
			"float" => Ok(ColumnType::Float),
			"double" => Ok(ColumnType::Double),
			// TODO: Make string length variable
			"string" => Ok( ColumnType::String(usize::MAX) ),
			_ => Err( ParserError::Unexpected( kind, "Type".to_string() ) ),
		}
	}
} // }}}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
	Primary,
	/// A null constraint with a percentage (`0` to `100`) of how many times it
	/// can be null
	Null(u8),
	/// A link to another column, possibly in another table
	Link(String),
}

impl Constraint { // {{{
	pub fn try_from(name: String, arguments: Vec<String>) -> Result<Constraint, ParserError> {
		let arg_empty = arguments.is_empty();
		let arg_count = arguments.len() as usize;

		match name.as_str() {
			"primary" if arg_empty => Ok(Constraint::Primary),
			"primary" if !arg_empty => Err( ParserError::TooManyArguments(arg_count, 0) ),

			"null" if arg_count == 1 => {
				let percentage = arguments[0].parse::<u8>()
					.map_err( |e| ParserError::Unknown( e.to_string() ) )?;

				Ok( Constraint::Null(percentage) )
			},
			"null" if arg_empty => Err( ParserError::TooFewArguments(arg_count, 1) ),
			"null" => Err( ParserError::TooManyArguments(arg_count, 1) ),

			"link" if arg_count == 1 => Ok( Constraint::Link( arguments[0].clone() ) ),
			"link" if arg_empty => Err( ParserError::TooFewArguments(arg_count, 1) ),
			"link" => Err( ParserError::TooManyArguments(arg_count, 1) ),

			_ => Err( ParserError::Unexpected( name, "Constraint".to_string() ) ),
		}
	}
} // }}}

#[derive(Debug, Clone, PartialEq)]
pub struct Provider {
	name: String,
	arguments: Vec<String>,
}

impl Provider { // {{{
	pub fn from(name: String, arguments: Vec<String>) -> Result<Provider, ParserError> {
		Ok(Provider {
			name,
			arguments,
		})
	}
} // }}}

#[derive(Debug)]
pub struct Column {
	pub name: String,
	pub kind: ColumnType,
	pub constraints: Vec<Constraint>,
	pub provider: Provider,
}

impl Column { // {{{
	pub fn new(name: String, kind: ColumnType, provider: Provider) -> Column {
		Column {
			name,
			kind,
			constraints: vec![],
			provider,
		}
	}

	pub fn add_constraint(&mut self, constraint: Constraint) {
		self.constraints.push(constraint);
	}
}
// }}}

#[derive(Debug)]
pub struct Table {
	pub name: String,
	pub columns: Vec<Column>,
}

impl Table { // {{{
	pub fn new(name: String) -> Table {
		Table {
			name,
			columns: vec![],
		}
	}

	pub fn add_column(&mut self, column: Column) {
		self.columns.push(column);
	}
}
// }}}

#[derive(Debug)]
pub struct Config {
	pub tables: Vec<Table>,
}

impl Config { // {{{
	pub fn new() -> Config {
		Config {
			tables: vec![],
		}
	}

	pub fn add_table(&mut self, table: Table) {
		self.tables.push(table);
	}
}
// }}}
