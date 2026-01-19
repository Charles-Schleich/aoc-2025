use std::num::ParseIntError;
use thiserror::Error;

// 
use super::day1::Day1Error;

//
// pub enum ReadError <T>
// where T:std::fmt::Debug {

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Failed to read file.\nError: {0}")]
    Fmt(#[from] std::io::Error),

    #[error("Failed to parse as int\nError: {0}")]
    Parse(#[from] ParseIntError),

    #[error("DataSetParsing\nError: {0}")]
    DataSetParsingError(#[from] Day1Error),
    // #[error("data store disconnected")]
    // Disconnect(#[from] std::io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("unknown data store error")]
    Unknown,
}
