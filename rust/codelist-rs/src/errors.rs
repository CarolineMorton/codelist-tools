use std::fmt;
use std::io;
use serde_json;
use csv;

/// Enum to represent the different types of errors that can occur in the codelist library
/// 
/// * `InvalidCodeListType` - An error that occurs when an invalid code list type is provided
/// * `JSONError` - An error that occurs when there is an error serializing or deserializing JSON
/// * `IOError` - An error that occurs when there is an error reading or writing to a file
/// * `EntryNotFound` - An error that occurs when an entry is not found in the codelist
/// * `CSVError` - An error that occurs when there is an error serializing or deserializing CSV

#[derive(Debug)]
pub enum CodeListError {
    InvalidCodeListType(String),
    JSONError(serde_json::Error),
    IOError(io::Error),
    EntryNotFound(String),
    CSVError(csv::Error),
    EmptyCode,
    EmptyTerm,
    InvalidFilePath,
}

impl From<io::Error> for CodeListError {
    fn from(err: io::Error) -> Self {
        CodeListError::IOError(err)
    }
}

impl From<serde_json::Error> for CodeListError {
    fn from(err: serde_json::Error) -> Self {
        CodeListError::JSONError(err)
    }
}

impl From<csv::Error> for CodeListError {
    fn from(err: csv::Error) -> Self {
        CodeListError::CSVError(err)
    }
}

impl fmt::Display for CodeListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCodeListType(invalid_type) => write!(f, "Invalid codelist type provided: {}", invalid_type),
            Self::JSONError(err) => write!(f, "JSON error: {}", err),
            Self::IOError(err) => write!(f, "IO error: {}", err),
            Self::EntryNotFound(code) => write!(f, "Entry not found: {}", code),
            Self::CSVError(err) => write!(f, "CSV error: {}", err),
            Self::EmptyCode => write!(f, "Code is an empty string"),
            Self::EmptyTerm => write!(f, "Term is an empty string"),
            Self::InvalidFilePath => write!(f, "Invalid file path"),
        }
    }
}
