pub use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;
