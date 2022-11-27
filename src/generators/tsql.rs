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

pub struct TsqlGenerator {
	table_name: String,
	row_count: usize,
	output_file: Option<File>,
	/// Contains the string `insert into <table> (<columns>) values `,
	/// including the trailing space
	columns: String,
	initialized: bool,
}

impl TsqlGenerator {
	fn write_to_output(&mut self, data: &String) -> Result<(), GeneratorError> { // {{{
		self.output_file.as_ref()
			.unwrap()
			.write( data.as_bytes() )
			.map_err( |e| GeneratorError::Write( e.to_string() ) )?;

		Ok(())
	} // }}}

	fn generate_columns(&mut self, columns: &Vec<ColumnData>) -> Result<(), GeneratorError> { // {{{
		let columns_string = columns.into_iter()
			.map( |c| c.name.clone() )
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

		self.output_file.as_ref()
			.unwrap()
			.flush()
			.map_err( |e| GeneratorError::Write( e.to_string() ) )?;

		Ok(())
	} // }}}
}

impl GeneratorImpl for TsqlGenerator {
	fn new() -> Self
		where Self: Sized { // {{{
		TsqlGenerator {
			table_name: "".to_string(),
			row_count: 0,
			output_file: None,
			columns: "".to_string(),
			initialized: false,
		}
	} // }}}

	fn init(&mut self, table_name: String, row_count: usize, output_file: File) -> Result<(), GeneratorError> { // {{{
		self.table_name = table_name;
		self.row_count = row_count;
		self.output_file = Some(output_file);
		self.initialized = true;

		Ok(())
	} // }}}

	fn generate(&mut self, data: GeneratorData) -> Result<(), GeneratorError> { // {{{
		if !self.initialized {
			return Err( GeneratorError::Uninitialized );
		}

		self.generate_columns(&data)?;

		// allocate vector with length equal to the amount of columns
		let mut row_data: Vec<&String> = Vec::with_capacity( data.len() );

		for i in 0..self.row_count {
			for row in data.iter() {
				row_data.push( row.data.get(i).unwrap() );
			}

			self.generate_row(&row_data)?;

			row_data.clear();
		}

		Ok(())
	} // }}}
}

#[cfg(test)]
mod tests {
	use tempfile::tempfile;
	use std::io::{ Read, Seek, SeekFrom };

	use crate::parser::config::ColumnType;
	use super::*;

	const ROW_COUNT: usize = 10;
	const TABLE_NAME: &str = "test_table";

	struct Setup { // {{{
		column_1: ColumnData,
		column_2: ColumnData,
		column_data: Vec<ColumnData>,
	}

	impl Setup {
		fn new() -> Self {
			let column_1: ColumnData = ColumnData { // {{{
				name: "test_column_1".to_string(),
				r#type: ColumnType::String(10),
				data: vec![
					"'Data1'".to_string(),
					"'Data2'".to_string(),
					"'Data3'".to_string(),
					"'Data4'".to_string(),
					"'Data5'".to_string(),
					"'Data6'".to_string(),
					"'Data7'".to_string(),
					"'Data8'".to_string(),
					"'Data9'".to_string(),
					"'Data10'".to_string(),
				],
			}; // }}}

			let column_2: ColumnData = ColumnData { // {{{
				name: "test_column_2".to_string(),
				r#type: ColumnType::String(10),
				data: vec![
					"'Data11'".to_string(),
					"'Data12'".to_string(),
					"'Data13'".to_string(),
					"'Data14'".to_string(),
					"'Data15'".to_string(),
					"'Data16'".to_string(),
					"'Data17'".to_string(),
					"'Data18'".to_string(),
					"'Data19'".to_string(),
					"'Data20'".to_string(),
				],
			}; // }}}

			// create a copy of column_1 and column_2, because String doesn't
			// implement Copy, but we need a copy for Setup.column_data
			let column_3: ColumnData = ColumnData { // {{{
				name: "test_column_1".to_string(),
				r#type: ColumnType::String(10),
				data: vec![
					"'Data1'".to_string(),
					"'Data2'".to_string(),
					"'Data3'".to_string(),
					"'Data4'".to_string(),
					"'Data5'".to_string(),
					"'Data6'".to_string(),
					"'Data7'".to_string(),
					"'Data8'".to_string(),
					"'Data9'".to_string(),
					"'Data10'".to_string(),
				],
			}; // }}}

			let column_4: ColumnData = ColumnData { // {{{
				name: "test_column_2".to_string(),
				r#type: ColumnType::String(10),
				data: vec![
					"'Data11'".to_string(),
					"'Data12'".to_string(),
					"'Data13'".to_string(),
					"'Data14'".to_string(),
					"'Data15'".to_string(),
					"'Data16'".to_string(),
					"'Data17'".to_string(),
					"'Data18'".to_string(),
					"'Data19'".to_string(),
					"'Data20'".to_string(),
				],
			}; // }}}

			Self {
				column_1,
				column_2,
				column_data: vec![ column_3, column_4 ],
			}
		}
	} // }}}

	#[test]
	fn test_generate_columns_generates_the_correct_column_layout() -> Result<(), GeneratorError> { // {{{
		let setup = Setup::new();
		// TODO: check if file actually deletes after usage
		let file = tempfile().unwrap();

		let mut sut = TsqlGenerator::new();
		sut.init(
			TABLE_NAME.to_string(),
			ROW_COUNT,
			file,
		)?;

		sut.generate_columns(&setup.column_data)?;

		assert_eq!(
			format!(
				"insert into {} ({}, {}) values ",
				TABLE_NAME,
				setup.column_1.name,
				setup.column_2.name,
			),
			sut.columns,
		);

		Ok(())
	} // }}}

	#[test]
	fn test_generate_row_generates_the_correct_output_row() -> Result<(), GeneratorError> { // {{{
		let setup = Setup::new();
		let mut file = tempfile().unwrap();
		let columns = format!(
			"insert into {} ({}, {}) values ",
			TABLE_NAME,
			setup.column_1.name,
			setup.column_2.name,
		);

		let mut sut = TsqlGenerator::new();
		sut.init(
			TABLE_NAME.to_string(),
			ROW_COUNT,
			file.try_clone().unwrap(),
		)?;

		sut.columns = columns.clone();

		sut.generate_row(
			&vec![ &setup.column_1.data[0], &setup.column_2.data[0] ]
		)?;

		let mut output = String::with_capacity(50);

		file.seek( SeekFrom::Start(0) ).unwrap();
		file.read_to_string(&mut output).unwrap();

		assert_eq!(
			format!(
				"{}({}, {});\n",
				columns.clone(),
				setup.column_1.data[0],
				setup.column_2.data[0],
			),
			output,
		);

		Ok(())
	} // }}}

	// TODO: write a unit test for TsqlGenerator.generate(self, GeneratorData).
	// Probably need some way of mocking for it to be a unit test?
}
