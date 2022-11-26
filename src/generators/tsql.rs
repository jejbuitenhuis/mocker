use std::{
	fs::File,
	io::Write,
};

use crate::generator::{
	ColumnData,
	GeneratorData,
	GeneratorError,
	GeneratorImpl,
};

pub struct TsqlGenerator<'a> {
	table_name: String,
	row_count: usize,
	output_file: &'a mut File,
	/// Contains the string `insert into <table> (<columns>) values `,
	/// including the trailing space
	columns: String,
}

impl<'a> TsqlGenerator<'a> {
	fn write_to_output(&mut self, data: &String) -> Result<(), GeneratorError> { // {{{
		self.output_file.write( data.as_bytes() )
			.map_err( |e| GeneratorError::Write( e.to_string() ) )?;

		Ok(())
	} // }}}

	fn generate_columns(&mut self, columns: Vec<&ColumnData>) -> Result<(), GeneratorError> { // {{{
		let columns_string = columns.into_iter()
			.map( |(name, _)| name.clone() )
			.collect::< Vec<String> >()
			.join(", ");

		self.columns = format!(
			"insert into {} ({}) values ",
			self.table_name,
			columns_string,
		);

		Ok(())
	} // }}}

	fn generate_row(&mut self, rows: &Vec<&String>) -> Result<(), GeneratorError> { // {{{
		println!("writing data: {:?}", rows);

		self.write_to_output( &self.columns.clone() )?;

		self.write_to_output( &"(".to_string() )?;

		for i in 0..rows.len() - 1 {
			self.write_to_output( rows.get(i).unwrap() )?;
			self.write_to_output( &", ".to_string() )?;
		}

		self.write_to_output( rows.get( rows.len() - 1 ).unwrap() )?;

		self.write_to_output( &");\n".to_string() )?;

		self.output_file.flush()
			.map_err( |e| GeneratorError::Write( e.to_string() ) )?;

		Ok(())
	} // }}}
}

impl<'a> GeneratorImpl<'a> for TsqlGenerator<'a> {
	fn new(table_name: String, row_count: usize, output_file: &'a mut File) -> Self
		where Self: Sized { // {{{
		TsqlGenerator {
			table_name,
			row_count,
			output_file,
			columns: "".to_string(),
		}
	} // }}}

	fn generate(&mut self, data: GeneratorData) -> Result<(), GeneratorError> { // {{{
		self.generate_columns( data.keys().collect() )?;

		// allocate vector with length equal to the amount of columns
		let mut row_data: Vec<&String> = Vec::with_capacity( data.len() );

		for i in 0..self.row_count {
			for (_row, data) in data.iter() {
				row_data.push( data.get(i).unwrap() );
			}

			self.generate_row(&row_data)?;

			row_data.clear();
		}

		Ok(())
	} // }}}
}
