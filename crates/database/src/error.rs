//! 🚨 bizkit database errors

use std::fmt;

#[derive(Debug)]
/// error management
pub enum Error {
    /// Database connection error
    Connection,
    /// Database transaction error
    Transaction,
    /// Database table creation error
    TableCreation,
    /// Database field error
    Field,
    /// SQL error
    SQL,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connection => {
                write!(f, "Database connection")
            }
            Self::Transaction => {
                write!(f, "Database transaction error")
            }
            Self::TableCreation => {
                write!(f, "Database table creation error")
            }
            Self::Field => {
                write!(f, "Database field error")
            }
            Self::SQL => {
                write!(f, "Database SQL error")
            }
        }
    }
}
