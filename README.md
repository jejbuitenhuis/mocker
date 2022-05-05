# Mocker

A program to provide mocked data for your database.

# Config

The config is written in a custom language.

## Types

- `int`.
- `long`.
- `float`.
- `double`.
- `string(length = max)`.

## Constraints

- `$null(percentage = 100)`.
- `$primary()`: marks a column as a primary key. This makes it possible to `$link()` to it from another column.
- `$link(column)`: links the current column to `column`. Can be `"TableName.column_name"` or `"column_name"`. If only a column name is present, the table is defaulted to the current table.

## Providers

A provider determines what should be placed in the column. A provider always starts with a `#`.

- `#row()`: the current row number.
- `#number(min = 0, max = infinite)`: a random number between `min` and `max`.
- `#decimal(min = 0.0, max = infinite)`: a random decimal between `min` and `max`.
- `#name()`.
- `#first_name()`.
- `#last_name()`.
- `#gender(long = false)`: `M`, `F` or `O`. `MALE`, `FEMALE` or `OTHER` when `long` is `true`.
- `#date()`.
- `#time()`.
- `#date_time()`.
