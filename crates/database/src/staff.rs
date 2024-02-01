//! Bizkit - staff database management

use rusqlite::{params, Connection};
use crate::Error;

/// SQL queries for staff accounts
mod sql{
    /// Create staff table
    pub(crate) const CREATE_STAFF_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS staff (
        id INTEGER PRIMARY KEY,                       -- The Identifier of the product, the Rust Type is `i64`
        firstname TEXT NOT NULL,                      -- First name of the staff
        middlenames TEXT,                             -- Middle names of the staff
        lastname TEXT NOT NULL,                       -- Last name of the staff
        username TEXT NOT NULL,                       -- Username of the staff
        email TEXT NOT NULL,                          -- Email address of the staff
        phone TEXT,                                   -- Phone number to contact staff
        bio TEXT,                                     -- Short bio of staff
        staffid TEXT,                                 -- Unique staff id
        isadmin INTEGER NOT NULL,                     -- Indicates if staff is admin or not
        groups TEXT DEFAULT('[]') NOT NULL,           -- Indicates the groups the staff member belongs too
        position TEXT NOT NULL,                       -- Staff's job position
        joined TEXT DEFAULT(date('now')) NOT NULL,    -- Date when staff started working in company
        status TEXT DEFAULT('Active') NOT NULL,       -- Status of staff
        lastlogin TEXT DEFAULT('NEVER'),              -- Date when staff last logged in
        gender TEXT,                                  -- Gender of staff member
        version TEXT NOT NULL,                        -- Data type schema version
        password_hash TEXT NOT NULL)                  -- Hash of password";
}

/// database controller
pub struct Database {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Self {
        Database {
            path: path.to_string(),
            conn: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        // Open database connection
        let conn =
            Connection::open(self.path.clone()).map_err(|_| Error::Connection)?;
        self.conn = Some(conn);

        // Create database tables if they do not already exist
        match &mut self.conn {
            Some(conn) => {
                let tx = conn
                    .transaction()
                    .map_err(|_| Error::Transaction)?;

                tx.execute(sql::CREATE_STAFF_TABLE, ())
                    .map_err(|_| Error::TableCreation)?;

                tx.commit().map_err(|_| Error::TableCreation)?;

                Ok(())
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
}
