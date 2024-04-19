//! # Get all categories kontroller

use bizkitdatabase::{CategoriesSql, Database, DatabaseError};
use kong::{server, ErrorResponse, Kong, Kontrol, Method};
use kong_kontrollers::accounts::database::{DB_Type, Database as AccountsDB};
use std::sync::{Arc, Mutex};

/// Get all categories kontroller
pub struct GetAllCategoriesKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: DB_Type,
}

impl Kontrol for GetAllCategoriesKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    /// Get blogs
    fn kontrol(&self, _kong: &Kong) -> server::Response {
        let mut client = self.database.lock().unwrap();
        let res = CategoriesSql::read_all(&mut client);

        match res {
            Ok(post) => server::Response::json(&post).with_status_code(200),
            Err(err) => match err {
                DatabaseError::Field => ErrorResponse::bad_request(),
                _ => ErrorResponse::internal(),
            },
        }
    }
}
