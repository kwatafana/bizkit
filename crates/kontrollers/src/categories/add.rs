//! # Add category kontroller

use bizkitdata::{CategoryDatabaseInput, CategoryInput};
use bizkitdatabase::{CategoriesSql, DatabaseError};
use kong::{
    inputs::UserInput, json_from_str, server, ErrorResponse, JsonError, JsonValue, Kong, Kontrol,
    Method,
};
use kong_kontrollers::accounts::database::DB_Type;
use kong_kontrollers::login::is_admin;

/// ✨ Create category kontroller
pub struct AddCategoryKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: DB_Type,
}

impl Kontrol for AddCategoryKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    fn get_input(&self, request: &server::Request) -> Option<JsonValue> {
        let input: Option<CategoryInput> = if let Ok(input) = server::input::json_input(request) {
            Some(input)
        } else {
            None
        };

        if let Some(input) = input {
            Some(input.as_json())
        } else {
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            let res: Result<CategoryInput, JsonError> = json_from_str(&input.to_string());

            if let Ok(input) = res {
                if input.is_valid().is_ok() {
                    Ok(Some(serde_json::json!(input)))
                } else {
                    // TODO: proper error handling
                    Err(())
                }
            } else {
                // TODO: proper error handling
                Err(())
            }
        } else {
            // TODO: proper error handling
            Err(())
        }
    }

    /// Add student
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(k) = &kong.kpassport {
            if let Ok(admin) = is_admin(k, self.database.clone()) {
                if admin {
                    if let Some(input) = &kong.input {
                        let res: Result<CategoryInput, JsonError> =
                            json_from_str(&input.to_string());

                        // Derive category from string
                        let category: CategoryDatabaseInput = if let Ok(input) = res {
                            input.into()
                        } else {
                            return ErrorResponse::bad_request();
                        };

                        let mut client = self.database.lock().unwrap();

                        // Store student into the database
                        let res = CategoriesSql::create(&mut client, category.clone());
                        match res {
                            Ok(()) => server::Response::json(&category).with_status_code(201),
                            Err(err) => match err {
                                DatabaseError::Field => ErrorResponse::bad_request(),
                                _ => ErrorResponse::internal(),
                            },
                        }
                    } else {
                        ErrorResponse::unauthorized()
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
