//! Bizkit - staff database management

use crate::Error;
use bizkitdata::{Staff, StaffStatus};
use kommon::Gender;
use rusqlite::{params, Connection};

/// SQL queries for staff accounts
mod sql {
    /// Create staff table
    pub(crate) const CREATE_STAFF_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS staff (
        id INTEGER PRIMARY KEY,                       -- The Identifier of the staff, the Rust Type is `i64`
        username TEXT UNIQUE NOT NULL,                -- Username of the staff
        firstname TEXT NOT NULL,                      -- First name of the staff
        middlenames TEXT,                             -- Middle names of the staff
        lastname TEXT NOT NULL,                       -- Last name of the staff
        email TEXT UNIQUE NOT NULL,                   -- Email address of the staff
        phone TEXT,                                   -- Phone number to contact staff
        bio TEXT,                                     -- Short bio of staff
        staffid TEXT,                                 -- Unique staff id
        isadmin INTEGER DEFAULT(0) NOT NULL,          -- Indicates if staff is admin or not
        groups TEXT DEFAULT('[]') NOT NULL,           -- Indicates the groups the staff member belongs too
        position TEXT NOT NULL,                       -- Staff's job position
        joined TEXT DEFAULT(date('now')) NOT NULL,    -- Date when staff started working in company
        status TEXT DEFAULT('Active') NOT NULL,       -- Status of staff
        lastlogin TEXT,                               -- Date when staff last logged in
        gender TEXT,                                  -- Gender of staff member
        version INTEGER NOT NULL,                     -- Data type schema version
        password_hash TEXT NOT NULL)                  -- Hash of password";

    /// Add staff
    pub(crate) const ADD_STAFF: &str = "
      INSERT INTO staff (username,
        firstname,
        middlenames,
        lastname,
        email,
        phone,
        bio,
        staffid,
        isadmin,
        groups,
        position,
        joined,
        status,
        gender,
        version,
        password_hash)
      VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)";

    /// Get staff by username
    pub(crate) const GET_STAFF_BY_USERNAME: &str = "SELECT
        username,
        firstname,
        middlenames,
        lastname,
        email,
        phone,
        bio,
        staffid,
        isadmin,
        groups,
        position,
        joined,
        status,
        lastlogin,
        gender,
        version,
        password_hash
       FROM staff WHERE username = ?;";
}

/// Database controller
pub struct Database {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl Database {
    /// Create new database
    pub fn new(path: &str) -> Self {
        Database {
            path: path.to_string(),
            conn: None,
        }
    }

    /// Connect database
    pub fn connect(&mut self) -> Result<(), Error> {
        // Open database connection
        let conn = Connection::open(self.path.clone()).map_err(|_| Error::Connection)?;
        self.conn = Some(conn);

        // Create database tables if they do not already exist
        match &mut self.conn {
            Some(conn) => {
                let tx = conn.transaction().map_err(|_| Error::Transaction)?;

                tx.execute(sql::CREATE_STAFF_TABLE, ())
                    .map_err(|_| Error::TableCreation)?;

                tx.commit().map_err(|_| Error::TableCreation)?;

                Ok(())
            }
            None => Err(Error::Connection),
        }
    }

    /// Add staff
    pub fn add_staff(&self, staff: &Staff) -> Result<(), Error> {
        match &self.conn {
            Some(conn) => {
                if let Ok(groups) = serde_json::to_string(&staff.groups) {
                    conn.execute(
                        sql::ADD_STAFF,
                        params![
                            &staff.username,
                            &staff.firstname,
                            &staff.middlenames,
                            &staff.lastname,
                            &staff.email,
                            &staff.phone,
                            &staff.bio,
                            &staff.staffid,
                            &staff.isadmin,
                            groups,
                            &staff.position,
                            &staff.joined,
                            &staff.status,
                            &staff.gender,
                            &staff.version,
                            &staff.password_hash
                        ],
                    )
                    .map_err(|_| Error::Field)?;
                    Ok(())
                } else {
                    Err(Error::Field)
                }
            }
            None => Err(Error::Connection),
        }
    }

    /// Get staff by username
    pub fn get_staff_by_username(&self, username: &str) -> Result<Option<Staff>, Error> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_STAFF_BY_USERNAME)
                    .map_err(|_| Error::SQL)?;
                let mut staff: Vec<Staff> = vec![];

                let staff_iter = stmt
                    .query_map(params![username], |s| {
                        let groups_str: String = s.get(9).map_err(|_| Error::Field).unwrap();
                        let groups: Vec<String> = serde_json::from_str(&groups_str)
                            .map_err(|_| Error::Field)
                            .unwrap();

                        let status_str: String = s.get(12).map_err(|_| Error::Field).unwrap();
                        let status: StaffStatus =
                            status_str.trim().parse().map_err(|_| Error::Field).unwrap();

                        let gender_str: String = s.get(14).map_err(|_| Error::Field).unwrap();
                        let gender: Gender =
                            gender_str.trim().parse().map_err(|_| Error::Field).unwrap();

                        Ok(Staff {
                            username: s.get(0).map_err(|_| Error::Field).unwrap(),
                            firstname: s.get(1).map_err(|_| Error::Field).unwrap(),
                            middlenames: s.get(2).map_err(|_| Error::Field).unwrap(),
                            lastname: s.get(3).map_err(|_| Error::Field).unwrap(),
                            email: s.get(4).map_err(|_| Error::Field).unwrap(),
                            phone: s.get(5).map_err(|_| Error::Field).unwrap(),
                            bio: s.get(6).map_err(|_| Error::Field).unwrap(),
                            staffid: s.get(7).map_err(|_| Error::Field).unwrap(),
                            isadmin: s.get(8).map_err(|_| Error::Field).unwrap(),
                            groups,
                            position: s.get(10).map_err(|_| Error::Field).unwrap(),
                            joined: s.get(11).map_err(|_| Error::Field).unwrap(),
                            status,
                            lastlogin: s.get(13).map_err(|_| Error::Field).unwrap(),
                            gender,
                            version: s.get(15).map_err(|_| Error::Field).unwrap(),
                            password_hash: s.get(16).map_err(|_| Error::Field).unwrap(),
                        })
                    })
                    .map_err(|_| Error::Field)?;

                for c in staff_iter {
                    staff.push(c.unwrap());
                }

                if staff.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(staff[0].clone()))
                }
            }
            None => Err(Error::Connection),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_DB_PATH: &str = "STAFF.sqlite";

    #[test]
    fn connect_db() {
        let mut db = Database::new(TEST_DB_PATH.into());

        // Connect to database
        db.connect().unwrap();

        if db.conn.is_none() {
            panic!("Could not connect to database");
        }
    }

    #[test]
    fn test_store_get_staff() {
        remove_test_db();
        let mut db = Database::new(TEST_DB_PATH.into());

        let staff = Staff {
            username: "flyingrhino".to_string(),
            firstname: "John".to_string(),
            middlenames: None,
            lastname: "Pandeni".to_string(),
            email: "info.gmail.com".to_string(),
            phone: None,
            bio: None,
            staffid: None,
            isadmin: false,
            groups: vec![],
            position: "Manager".to_string(),
            joined: chrono::Utc::now(),
            status: bizkitdata::StaffStatus::Active,
            lastlogin: None,
            gender: kommon::Gender::Other,
            version: 0,
            password_hash: "kdfa;lkfjioadfkdfjklj".to_string(),
        };

        db.connect().unwrap();
        db.add_staff(&staff).unwrap();

        let pr = db.get_staff_by_username(&staff.username).unwrap();

        if let Some(s) = pr {
            assert_eq!(s, staff);
        } else {
            panic!("Could not get staff from database");
        }
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
