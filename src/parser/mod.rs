use std::{ fs, path::PathBuf };
use pest::{
	iterators::Pair,
	Parser as PestParser,
};
use pest_derive::Parser as PestParser;

use crate::parser::{
	errors::ParserError,
	config::{
		Argument,
		Config,
		Column,
		Provider,
		ColumnType,
		Constraint,
		Table,
	},
};

pub mod config;
pub mod errors;

#[derive(PestParser)]
#[grammar = "parser/mock_table_definition.pest"]
struct MockerParser;

pub struct Parser {
	file_content: String,
}

impl Parser {
	pub fn new(file: &PathBuf) -> Result<Parser, ParserError> { // {{{
		let file_content = fs::read_to_string(file)
			.map_err( |e| ParserError::FileError( e.to_string() ) )?;

		Ok(Parser { file_content })
	} // }}}

	fn parse_function_call_args(
		&self,
		function_call_args: Pair<Rule>,
		args: &mut Vec<Argument>,
	)
		-> Result<(), ParserError>
	{ // {{{
			// Argument::try_from(function_call_item)
		for argument in function_call_args.into_inner() {
			match argument.as_rule() {
				Rule::call_arg => {
					let arg_rule = argument.into_inner()
						.next()
						.expect("function call argument should contain an argument value");

					args.push( Argument::try_from(arg_rule)? );
				},

				r => unreachable!("Unexpected rule encountered while parsing function_call_args: {:?}", r),
			}
		}

		Ok(())
	} // }}}

	fn parse_function_call(
		&self,
		function_call: Pair<Rule>,
		args: &mut Vec<Argument>,
	) -> Result<(), ParserError> { // {{{
		for function_call_item in function_call.into_inner() {
			match function_call_item.as_rule() {
				Rule::function_call_empty => {}, // no arguments to parse
				Rule::function_call_args =>
					self.parse_function_call_args(function_call_item, args)?,

				r => unreachable!("Unexpected rule encountered while parsing function_call_item: {:?}", r),
			}
		}

		Ok(())
	} // }}}

	fn parse_constraint(&self, constraint: Pair<Rule>)
		-> Result<Constraint, ParserError>
	{ // {{{
		let mut constraint_name: Option<String> = None;
		let mut constraint_args = Vec::with_capacity(2);

		for constraint_item in constraint.into_inner() {
			match constraint_item.as_rule() {
				Rule::constraint_name => {
					let name = constraint_item.as_span()
						.as_str()
						.to_string();

					constraint_name = Some(name);
				},
				Rule::function_call =>
					self.parse_function_call(constraint_item, &mut constraint_args)?,

				r => unreachable!("Unexpected rule encountered while parsing constraint: {:?}", r),
			}
		}

		Ok( Constraint::new(
			constraint_name.expect("no constraint name should be caught by pest.rs"),
			constraint_args,
		) )
	} // }}}

	fn parse_provider(&self, provider: Pair<Rule>)
		-> Result<Provider, ParserError>
	{ // {{{
		let mut provider_name: Option<String> = None;
		let mut provider_args = Vec::with_capacity(2);

		for provider_item in provider.into_inner() {
			match provider_item.as_rule() {
				Rule::provider_name => {
					let name = provider_item.into_inner()
						.next()
						.expect("provider should contain a provider name")
						.as_str()
						.to_string();

					provider_name = Some(name);
				},
				Rule::function_call =>
					self.parse_function_call(provider_item, &mut provider_args)?,

				r => unreachable!("Unexpected rule encountered while parsing provider: {:?}", r),
			}
		}

		Ok( Provider::new(
			provider_name.expect("no provider name should be caught by pest.rs"),
			provider_args,
		) )
	} // }}}

	fn parse_column_definition(&self, column_definition: Pair<Rule>)
		-> Result<Column, ParserError>
	{ // {{{
		let mut column_name: Option<String> = None;
		let mut column_type: Option<ColumnType> = None;
		let mut constraints = Vec::with_capacity(2);
		let mut provider: Option<Provider> = None;

		for column_definition_item in column_definition.into_inner() {
			match column_definition_item.as_rule() {
				Rule::column_name => {
					let name = column_definition_item.into_inner()
						.next()
						.expect("column_definition should contain a column name")
						.as_str()
						.to_string();

					column_name = Some(name);
				},
				Rule::r#type => {
					let r#type = column_definition_item.as_span()
						.as_str()
						.to_string();
					let r#type = ColumnType::try_from(r#type)?;

					column_type = Some(r#type);
				},
				Rule::constraint => {
					constraints.push(
						self.parse_constraint(column_definition_item)?
					);
				},
				Rule::provider => {
					provider = Some(
						self.parse_provider(column_definition_item)?
					);
				},

				r => unreachable!("Unexpected rule encountered while parsing column_definition: {:?}", r),
			}
		}

		Ok( Column::new(
			column_name.expect("no column name should be caught by pest.rs"),
			column_type.expect("no column type should be caught by pest.rs"),
			constraints,
			provider.expect("no provider should be caught by pest.rs"),
		) )
	} // }}}

	fn parse_table_content(
		&self,
		table_content: Pair<Rule>,
		column_output: &mut Vec<Column>,
	) -> Result<(), ParserError> { // {{{
		for column_definition in table_content.into_inner() {
			match column_definition.as_rule() {
				Rule::column_definition => {
					let column = self.parse_column_definition(column_definition)?;

					column_output.push(column);
				},

				r => unreachable!("Unexpected rule encountered while parsing table_content: {:?}", r),
			}
		}

		Ok(())
	} // }}}

	fn parse_table_definition(&self, definition: Pair<Rule>)
		-> Result<Table, ParserError>
	{ // {{{
		let mut table_name = String::new();
		let mut table_columns = Vec::with_capacity(5);

		for pair in definition.into_inner() {
			match pair.as_rule() {
				Rule::table_definition => {
					table_name = pair.into_inner()
						.next()
						.expect("table_definition should contain a table name")
						.as_str()
						.trim()
						.to_string();
				},
				Rule::table_content =>
					self.parse_table_content(pair, &mut table_columns)?,

				r => unreachable!("Unexpected rule encountered while parsing mock_definition: {:?}", r),
			}
		}

		Ok( Table::new(table_name, table_columns) )
	} // }}}

	pub fn parse(&self) -> Result<Config, ParserError> { // {{{
		let mut config = Config::new();

		let result = MockerParser::parse( Rule::output, self.file_content.as_str() )
			.map_err( |e| ParserError::SyntaxError( e.to_string() ) )?
			.next()
			.ok_or( ParserError::EOF )?;

		for mock_definition in result.into_inner() {
			let table = self.parse_table_definition(mock_definition)?;

			config.add_table(table);
		}

		Ok(config)
	} // }}}
}
