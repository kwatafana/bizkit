//! database management
mod categories;
mod error;
mod product_categories;
mod product_images;
mod products;

pub use categories::CategoriesSql;
pub use error::DatabaseError;
use postgres::{Client, NoTls};
pub use product_categories::ProductCategoriesSql;
pub use product_images::ProductImagesSql;
pub use products::ProductsSql;

/// Students database management
pub struct Database {
    /// postgres client
    pub client: Option<Client>,
}

impl Database {
    /// create new database
    pub fn new(db: &str, user: &str, password: &str) -> Result<Self, DatabaseError> {
        let params = format!("postgresql://localhost/{db}?user={user}&password={password}");
        let client = Client::connect(&params, NoTls).map_err(|_| DatabaseError::SQL)?;

        Ok(Database {
            client: Some(client),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::Config;

    #[test]
    fn connect_db() {
        let config = Config::from_file("example-config.toml").unwrap();
        let db = Database::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();
        match db.client {
            Some(_conn) => assert!(true),
            _ => assert!(false),
        }
    }
}
