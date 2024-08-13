use pest::iterators::Pair;
use std::fmt;

use crate::parser::errors::ParserError;
use super::Rule;

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
	Int(i64),
	Float(f64),
	String(String),
	Boolean(bool),
}

impl Argument {
	// try_from implementations {{{
	fn try_from_int_rule(rule: Pair<Rule>) -> Result<Self, ParserError> {
		let value = rule.as_span()
			.as_str();
		let value = value.parse()
			.map_err( |_| ParserError::Unexpected(
				value.to_string(),
				"int".to_string(),
			) )?;

		Ok( Self::Int(value) )
	}

	fn try_from_float_rule(rule: Pair<Rule>) -> Result<Self, ParserError> {
		let value = rule.as_span()
			.as_str();
		let value = value.parse()
			.map_err( |_| ParserError::Unexpected(
				value.to_string(),
				"float".to_string(),
			) )?;

		Ok( Self::Float(value) )
	}

	fn try_from_string_rule(rule: Pair<Rule>) -> Result<Self, ParserError> {
		let value = rule.as_span()
			.as_str()
			.to_string();

		Ok( Self::String(value) )
	}

	fn try_from_boolean_rule(rule: Pair<Rule>) -> Result<Self, ParserError> {
		let value = rule.as_span()
			.as_str();
		let value = value.parse()
			.map_err( |_| ParserError::Unexpected(
				value.to_string(),
				"boolean".to_string(),
			) )?;

		Ok( Self::Boolean(value) )
	}
	// }}}
}

impl TryFrom< Pair<'_, Rule> > for Argument { // {{{
	type Error = ParserError;

	fn try_from(rule: Pair<Rule>) -> Result<Self, Self::Error> {
		let parsed_type = match rule.as_rule() {
			Rule::INT => Self::try_from_int_rule(rule)?,
			Rule::FLOAT => Self::try_from_float_rule(rule)?,
			Rule::STRING => Self::try_from_string_rule(rule)?,
			Rule::BOOLEAN => Self::try_from_boolean_rule(rule)?,

			r => return Err( ParserError::Unexpected(
				format!("{:?}", r),
				"valid type".to_string(),
			) ),
		};

		Ok(parsed_type)
	}
} // }}}

impl fmt::Display for Argument { // {{{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let value: String = match self {
			Self::Int(i) => format!("{}", i),
			Self::Float(f) => format!("{}", f),
			Self::String(s) => format!("{}", s),
			Self::Boolean(b) => format!("{}", b),
		};

		write!(f, "{}", value)
	}
} // }}}

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

	fn try_from(kind: String) -> Result<Self, Self::Error> {
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
pub struct Constraint {
	pub name: String,
	pub arguments: Vec<Argument>,
}

impl Constraint { // {{{
	pub fn new(name: String, arguments: Vec<Argument>) -> Self {
		Self { name, arguments }
	}
} // }}}

#[derive(Debug, Clone, PartialEq)]
pub struct Provider {
	pub name: String,
	pub arguments: Vec<Argument>,
}

impl Provider { // {{{
	pub fn new(name: String, arguments: Vec<Argument>) -> Self {
		Self { name, arguments }
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
	pub fn new(
		name: String,
		kind: ColumnType,
		constraints: Vec<Constraint>,
		provider: Provider,
	) -> Column {
		Self { name, kind, constraints, provider }
	}
}
// }}}

#[derive(Debug)]
pub struct Table {
	pub name: String,
	pub columns: Vec<Column>,
}

impl Table { // {{{
	pub fn new(name: String, columns: Vec<Column>) -> Table {
		Self { name, columns }
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
			tables: Vec::with_capacity(5),
		}
	}

	pub fn add_table(&mut self, table: Table) {
		self.tables.push(table);
	}
}
// }}}
