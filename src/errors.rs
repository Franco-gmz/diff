//! This module contains custom errors to be returned in a Result.
/// Custom errors for when there are insufficient parameters or a file couldn't be opened.
#[derive(Debug, PartialEq)]
pub enum Errors {
    ArgError(String),
    FileError(String),
}
