# Telegram CSV Parser

This Rust program parses a CSV file containing data collected from Telegram. The primary goal is to extract messages sent by users.

## Technical Description

The parser uses the `telegram_csv_parser` crate, which is based on the Pest parser generator. It follows a set of grammar rules defined in the `csv.pest` file. The CSV file is expected to have a specific structure where messages are identified based on the presence of "PeerUser(user_id=" in the row.

### Parsing Process

1. The program reads the CSV file (`example_collected_data_from_telegram.csv`) into memory.
2. The `CSVParser` parses the content based on the specified grammar rules.
3. For each row, it checks if the row contains "PeerUser(user_id=" indicating a user's message.
4. If a message is found, the program counts the number of messages and words in the message.

### Parsing Logic

#### Grammar rules to parse fields of CSV file
 
#### `quoted_string = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }`

This rule defines a quoted string within double quotes.
' \" ' matches the opening double quote.
' (!"\"" ~ ANY)* ' matches any sequence of characters that is not a double quote, capturing everything between the double quotes.
' \" ' matches the closing double quote.

#### `value = { quoted_string | (!"," ~ (!"\n" ~ ANY))* }`

This rule defines a value, which can be either a quoted string or any sequence of characters that is not a comma.
Quoted_string is an alternative option.
' (!"," ~ (!"\n" ~ ANY))* ' matches any sequence of characters that is not a comma, capturing everything until a comma or the end of the line.

#### `row = { value ~ ("," ~ value)* }`

This rule defines a row, which consists of one or more values separated by commas.
Value matches the first value.
' ( "," ~ value)* ' matches zero or more occurrences of a comma followed by another value.

#### `file = { SOI ~ (row ~ ("\r\n" | "\n"))* ~ EOI }`

This rule defines a file, which starts with the start of input (SOI).
(row ~ ("\r\n" | "\n"))* matches zero or more occurrences of a row followed by either a Windows-style line ending (\r\n) or a Unix-style line ending (\n).
It ends with the end of input (EOI).

## Usage

Ensure you have Rust and Cargo installed. Then run the following commands:

`cargo run -- -f name_of_the_file_to_parse.csv`

Or

`cargo run -- --file name_of_the_file_to_parse.csv`

When need help, use

`cargo run -- - help`

## Documentation

In case if you need a documentation, follow the [link](doc\telegram_csv_parser\index.html).

## Example

![alt text](examples/example_1.jpg)

Running the following command:

![alt text](examples/example_2.jpg)

Or

![alt text](examples/example_3.jpg)

Result:

![alt text](examples/example_4.jpg)


