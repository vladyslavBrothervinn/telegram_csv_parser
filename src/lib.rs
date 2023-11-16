pub use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CsvError {
    #[error("Value is not correct")]
    ValueError,

    #[error("Row is not correct")]
    RowError,

    #[error("Incorrect string")]
    ParseError(String)

}