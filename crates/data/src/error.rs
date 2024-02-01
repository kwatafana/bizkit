//! 🚨 bizkit data errors

use std::fmt;

/// BizKit data arrors
#[derive(Debug)]
pub enum Error {
    /// Staff status deserialization error
    StaffStatus,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaffStatus => write!(f, "Staff status deserialization or serializtion error"),
        }
    }
}
