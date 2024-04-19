//! # Get all products kontroller

use bizkitdatabase::{DatabaseError, ProductsSql};
use kong::{server, ErrorResponse, Kong, Kontrol, Method};
use kong_kontrollers::accounts::database::DB_Type;

/// Get all products kontroller
pub struct GetAllProductsKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: DB_Type,
}

impl Kontrol for GetAllProductsKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    /// Get products
    fn kontrol(&self, _kong: &Kong) -> server::Response {
        let mut client = self.database.lock().unwrap();
        let res = ProductsSql::read_all(&mut client);

        match res {
            Ok(post) => server::Response::json(&post).with_status_code(200),
            Err(err) => match err {
                DatabaseError::Field => ErrorResponse::bad_request(),
                _ => ErrorResponse::internal(),
            },
        }
    }
}
