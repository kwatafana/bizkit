//! BizKit -- staff accounts

use crate::StaffStatus;
use chrono::{DateTime, Utc};
use kommon::Gender;
use serde::{Deserialize, Serialize};

/// Staff accounts
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Staff {
    /// Username of the staff
    pub username: String,
    /// First name of the staff
    pub firstname: String,
    /// Middle names of the staff
    pub middlenames: Option<String>,
    /// Last name of the staff
    pub lastname: String,
    /// Email address of the staff
    pub email: String,
    /// Phone number to contact staff
    pub phone: Option<String>,
    /// Short bio of staff
    pub bio: Option<String>,
    /// Unique staff id
    pub staffid: Option<String>,
    /// Indicates if staff is admin or not
    pub isadmin: bool,
    /// Indicates the groups the staff member belongs too
    pub groups: Vec<String>,
    /// Staff's job position
    pub position: String,
    /// Date when staff started working in company
    pub joined: DateTime<Utc>,
    /// Status of staff
    #[serde(serialize_with = "StaffStatus::serialize")]
    #[serde(deserialize_with = "StaffStatus::deserialize")]
    pub status: StaffStatus,
    /// Date when staff last logged in
    pub lastlogin: Option<DateTime<Utc>>,
    /// Gender of staff member
    #[serde(serialize_with = "Gender::serialize")]
    #[serde(deserialize_with = "Gender::deserialize")]
    pub gender: Gender,
    /// Data type schema version
    pub version: u16,
    /// Hash of password
    pub password_hash: String,
}
