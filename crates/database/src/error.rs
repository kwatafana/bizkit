//! database errors

use std::fmt;

/// Errors
#[derive(Debug)]
pub enum DatabaseError {
    /// Database connection error
    Connection,
    /// Database table creation error
    TableCreation,
    /// Database transaction error
    Transaction,
    /// Database sql statement error
    SQL,
    /// Database field refferencing error
    Field,
}

impl std::error::Error for DatabaseError {}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connection => write!(f, "Database connection error"),
            Self::TableCreation => write!(
                f,
                "Ann error occured while trying to create a database table"
            ),
            Self::Transaction => write!(f, "Database transaction error"),
            Self::SQL => write!(f, "Something went wrong while processing the SQL statement"),
            Self::Field => write!(f, "Could not refference the database table field"),
        }
    }
}
