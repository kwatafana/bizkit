use serde::{Deserialize, de::Deserializer, ser::Serializer};
use crate::Error;
use std::fmt;
use std::str::FromStr;
use rusqlite::types::{ToSql, ToSqlOutput, FromSql, ValueRef, FromSqlResult, FromSqlError};

/// The working status of a staff memeber
#[derive(PartialEq, Debug)]
pub enum StaffStatus{
    /// Active staff member
    Active,
    /// Retired staff member
    Retired,
    /// Resigned staff member
    Resigned,
    /// Absent staff member
    Absent,
    /// A staff memeber on holiday
    Holiday,
    /// A staff memeber on leave
    Leave,
    /// A staff memeber on vacation
    Vacation
}

impl StaffStatus{
    /// Deserialize
    pub fn deserialize <'de, D>(deserializer: D) -> Result<StaffStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;
	// TODO: don't use unwrap
        let out: StaffStatus = buf.parse::<StaffStatus>().unwrap();
	Ok(out)

    }

    /// Serialize
    pub fn serialize<S>(staff_status: &StaffStatus, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
	serializer.serialize_str(&staff_status.to_string())
    }
}

impl fmt::Display for StaffStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let status = match self{
	    StaffStatus::Active => "Active",
	    StaffStatus::Retired => "Retired",
	    StaffStatus::Resigned => "Resigned",
	    StaffStatus::Absent => "Absent",
	    StaffStatus::Holiday => "Holiday",
	    StaffStatus::Leave => "Leave",
	    StaffStatus::Vacation => "Vacation",

	};
        writeln!(f, "{status}")
    }
}

impl FromStr for StaffStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	match s{
            "Active" => Ok(StaffStatus::Active),
	    "Retired" => Ok(StaffStatus::Retired),
	    "Resigned" => Ok(StaffStatus::Resigned),
	    "Absent" => Ok(StaffStatus::Absent),
	    "Holiday" => Ok(StaffStatus::Holiday),
	    "Leave" => Ok(StaffStatus::Leave),
	    "Vacation" => Ok(StaffStatus::Vacation),
	    _ => Err(Error::StaffStatus),
	}
    }
}

impl ToSql for StaffStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for StaffStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
	value.as_str()?.parse()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
