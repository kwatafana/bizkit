//! # Delete single product kontroller

use bizkitdatabase::{DatabaseError, ProductsSql};
use kong::{server, ErrorResponse, Kong, Kontrol, Method};
use kong_kontrollers::accounts::database::DB_Type;
use kong_kontrollers::login::is_admin;

/// Delete a single product kontroller
pub struct DeleteProductByIdKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// Postgres database handle
    pub database: DB_Type,
}

impl Kontrol for DeleteProductByIdKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    /// Delete article
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(k) = &kong.kpassport {
            if let Ok(admin) = is_admin(k, self.database.clone()) {
                if admin {
                    if let Some(url_params) = &kong.url_parameters {
                        if let Some(id) = url_params.find("id") {
                            let id: i32 = id.parse().unwrap();
                            let mut client = self.database.lock().unwrap();
                            let res = ProductsSql::delete(&mut client, id);

                            match res {
                                Ok(post) => {
                                    server::Response::text("Product has been deleted, succesfully.")
                                        .with_status_code(200)
                                }
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
                } else {
                    ErrorResponse::unauthorized()
                }
            } else {
                ErrorResponse::internal()
            }
        } else {
            ErrorResponse::unauthorized()
        }
    }
}
