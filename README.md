# Mocker

A program to provide mocked data for your database.

> **Warning** This program is a work in progress, so a lot of things won't work yet.

# Usage

Mocker is a cli, so it can be used from the command line. To view the help, run `mocker` or `mocker --help`. This will show you all available commands.

To start mocking data, you need to create a mock file like the one below. For more examples, see the `examples` folder.

```mock
table TestTable {
	some_column int $primary() #row(),
	can_be_empty int $null(25) #number(0, 1000)
}

table SecondTable {
	id int #link("TestTable.some_column"),
	another_column string #name()
}
```

When run with `mocker --row-count 5 --output /tmp/mocker --type tsql file.mock`, the output should look something like this:

```sql
-- file /tmp/mocker/TestTable.sql
insert into TestTable (some_column, can_be_empty) values (1, 291);
insert into TestTable (some_column, can_be_empty) values (2, 624);
insert into TestTable (some_column, can_be_empty) values (3, 28);
insert into TestTable (some_column, can_be_empty) values (4, null);
insert into TestTable (some_column, can_be_empty) values (5, 300);

-- file /tmp/file/SecondTable.sql
insert into SecondTable (some_column, another_column) values (1, 'Winni Crinage');
insert into SecondTable (some_column, another_column) values (2, 'Maggie Sennett');
insert into SecondTable (some_column, another_column) values (3, 'Glad Barti');
insert into SecondTable (some_column, another_column) values (4, 'Orran O'' Markey');
insert into SecondTable (some_column, another_column) values (5, 'Dur Chittleburgh');
```

## Output types

The output of the following config when ran with `mocker --row-count 5 --type <language> file.mock` can be found below per `<language>`.

```mock
table Account {
	id int $primary() #row(),
	name string(128) #name(),
	gender string(1) $null(25) #gender(),
	created date_time #date_time()
}
```

### tsql

```sql
insert into Account (id, name, gender, created) values (1, 'Clementine Baglow', 'F', '2021-11-18 01:49:49');
insert into Account (id, name, gender, created) values (2, 'Delinda Perulli', null, '2022-01-18 11:50:58');
insert into Account (id, name, gender, created) values (3, 'Dillie Yarrall', 'O', '2021-09-10 22:37:09');
insert into Account (id, name, gender, created) values (4, 'Quintilla Talby', 'M', '2022-01-17 12:14:55');
insert into Account (id, name, gender, created) values (5, 'Corilla Impey', 'F', '2022-01-06 04:19:37');
```

### csv

```csv
id,name,gender,created
1,Clementine Baglow,F,2021-11-18 01:49:49
2,Delinda Perulli,,2022-01-18 11:50:58
3,Dillie Yarrall,O,2021-09-10 22:37:09
4,Quintilla Talby,M,2022-01-17 12:14:55
5,Corilla Impey,F,2022-01-06 04:19:37
```

### json

```json
[
	{
		"id": 1,
		"name": "Clementine Baglow",
		"gender": "F",
		"created": "2021-11-18 01:49:49"
	}, {
		"id": 2,
		"name": "Delinda Perulli",
		"gender": null,
		"created": "2022-01-18 11:50:58"
	}, {
		"id": 3,
		"name": "Dillie Yarrall",
		"gender": "O",
		"created": "2021-09-10 22:37:09"
	}, {
		"id": 4,
		"name": "Quintilla Talby",
		"gender": "M",
		"created": "2022-01-17 12:14:55"
	}, {
		"id": 5,
		"name": "Corilla Impey",
		"gender": "F",
		"created": "2022-01-06 04:19:37"
	}
]
```

### xml

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<data>
	<entry>
		<id>1</id>
		<name>Clementine Baglow</name>
		<gender>F</gender>
		<created>2021-11-18 01:49:49</created>
	</entry>
	<entry>
		<id>2</id>
		<name>Delinda Perulli</name>
		<gender />
		<created>2022-01-18 11:50:58</created>
	</entry>
	<entry>
		<id>3</id>
		<name>Dillie Yarrall</name>
		<gender>O</gender>
		<created>2021-09-10 22:37:09</created>
	</entry>
	<entry>
		<id>4</id>
		<name>Quintilla Talby</name>
		<gender>M</gender>
		<created>2022-01-17 12:14:55</created>
	</entry>
	<entry>
		<id>5</id>
		<name>Corilla Impey</name>
		<gender>F</gender>
		<created>2022-01-06 04:19:37</created>
	</entry>
</data>
```

# Config

The config is written in a custom language. The basic syntax is defined below:

```mock
table TableName {
	column_name type $optional_constraint() $another_optional_constraint() #required_provider()
}
```

## Types

- `int`
- `long`
- `float`
- `double`
- `string`

### Planned types

- `string(length = max)`
- `date`
- `time`
- `date_time`

## Constraints

A constraint restricts certain actions on a column. A constraint always starts with a `$`.

- `$null(percentage = 100)`

### Planned constraints

- `$primary()`: marks a column as a primary key. This makes it possible to `#link()` to it from another column

## Providers

A provider determines what should be placed in the column. A provider always starts with a `#`.

- `#row()`: the current row number. Starts at `1` and increments by 1 for every row
- `#number(min = 0, max = infinite)`: a random number between `min` and `max`
- `#gender(long = false)`: `M`, `F` or `O`. `MALE`, `FEMALE` or `OTHER` when `long` is `true`
- `#random(opt1, ..., optn)`: randomly chooses one of the provided options
- `#first_name()`: return a random first name

### Planned providers

- `#decimal(min = 0.0, max = infinite)`: a random decimal between `min` and `max`
- `#name()`
- `#last_name()`
- `#date()`
- `#time()`
- `#date_time()`
- `#link(column)`: links the current column to `column`. Can be `"TableName.column_name"` or `"column_name"`. If only a column name is present, the table is defaulted to the current table
