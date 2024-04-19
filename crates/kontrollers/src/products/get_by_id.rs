//! # Get single product kontroller

use bizkitdatabase::{Database, DatabaseError, ProductsSql};
use kong::{server, ErrorResponse, Kong, Kontrol, Method};
use kong_kontrollers::accounts::database::DB_Type;
use std::sync::{Arc, Mutex};

/// Get a single product kontroller
pub struct GetProductByIdKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: DB_Type,
}

impl Kontrol for GetProductByIdKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    /// Get blogs
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(url_params) = &kong.url_parameters {
            if let Some(id) = url_params.find("id") {
                let id: i32 = id.parse().unwrap();
                let mut client = self.database.lock().unwrap();
                let res = ProductsSql::read(&mut client, id);

                match res {
                    Ok(post) => server::Response::json(&post).with_status_code(200),
                    Err(err) => match err {
                        DatabaseError::Field => ErrorResponse::bad_request(),
                        _ => ErrorResponse::internal(),
                    },
                }
            } else {
                ErrorResponse::bad_request()
            }
        } else {
            ErrorResponse::bad_request()
        }
    }
}
