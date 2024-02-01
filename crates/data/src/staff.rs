//! BizKit -- staff accounts

use chrono::{DateTime, Utc};
use kommon::Gender;

/// Staff accounts
pub struct Staff{
    /// First name of the staff
    pub firstname: String,
    /// Middle names of the staff
    pub middlenames: Option<String>,
    /// Last name of the staff
    pub lastname: String,
    /// Username of the staff
    pub username: String,
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
    pub status: StaffStatus,
    /// Date when staff last logged in
    pub lastlogin: DateTime<Utc>,
    /// Gender of staff member
    pub gender: Option<Gender>,
    /// Data type schema version
    pub version: u32,
    /// Hash of password
    pub password_hash: String,
}

/// The working status of a staff memeber
pub enum StaffStatus{
    /// Active staff member
    Active,
    /// Retired staff member
    Restired,
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
