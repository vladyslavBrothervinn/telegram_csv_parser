# Telegram CSV Parser

This Rust program parses a CSV file containing data collected from Telegram. The primary goal is to extract messages sent by users.

## Technical Description

The parser uses the `telegram_csv_parser` crate, which is based on the Pest parser generator. It follows a set of grammar rules defined in the `csv.pest` file. The CSV file is expected to have a specific structure where messages are identified based on the presence of "PeerUser(user_id=" in the row.

### Parsing Process

1. The program reads the CSV file (`example_collected_data_from_telegram.csv`) into memory.
2. The `CSVParser` parses the content based on the specified grammar rules.
3. For each row, it checks if the row contains "PeerUser(user_id=" indicating a user's message.
4. If a message is found, the program counts the number of messages and words in the message.

## Usage

Ensure you have Rust and Cargo installed. Then run the following commands:

```bash
cargo build
cargo run
