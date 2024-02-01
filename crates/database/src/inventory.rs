//! bizkit -- Inventory management

use rusqlite::{params, Connection};
use std::path::PathBuf;
use bizkitdata::Product;
use crate::Error;

mod sql {
    /// Create inventory table
    pub(crate) const CREATE_INVENTORY_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS inventory (
        id INTEGER PRIMARY KEY,                       -- The Identifier of the product, the Rust Type is `i64`
        name TEXT NOT NULL,                           -- Name of the product
        manufacturer TEXT,                            -- Manufacturer of the product
        barcode TEXT,                                 -- Product barcode
        categories TEXT,                              -- Categories that the product belongs to
        unit TEXT,                                    -- Unit of measurements
        weight REAL,                                  -- Weight of the product
        price REAL,                                   -- The price of the product
        taxable INTEGER,                              -- Is the product taxable or not
        description TEXT,                             -- Product description
        images BLOB,                                  -- Product Images
        warehouse TEXT,                               -- Warehouse of the product
        added_by TEXT,                                -- User who added the product
        sold TEXT,                                    -- Is the product sold or not
        sold_by String,                               -- User who sold the product
        added TEXT)                                   -- The date when the product was added, the Rust Type is `chrono::DateTime`";

    /// Add a product
    pub(crate) const ADD_PRODUCT: &str = "
      INSERT INTO inventory (name,
        manufacturer,
        barcode,
        categories,
        unit,
        weight,
        price,
        taxable,
        description,
        images,
        warehouse,
        added_by,
        sold,
        sold_by,
        added
       )
      VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)";

    /// Get product by id
    pub(crate) const GET_PRODUCT_BY_ID: &str = "SELECT
      name,
        manufacturer,
        barcode,
        categories,
        unit,
        weight,
        price,
        taxable,
        description,
        images,
        warehouse,
        added_by,
        sold,
        sold_by,
        added
       FROM inventory WHERE id = ?;";

    /// Get all products
    pub(crate) const GET_ALL_PRODUCTS: &str = "
    SELECT
      name,
      manufacturer,
      barcode,
      categories,
      unit,
      weight,
      price,
      taxable,
      description,
      images,
      warehouse,
      added_by,
      sold,
      sold_by
 FROM inventory;";
}

/// Inventory database controller
pub struct InventoryDatabase {
    /// Inventory Database file path
    path: PathBuf,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl InventoryDatabase {
    /// Setup database
    pub fn new(path: PathBuf) -> Self {
        InventoryDatabase { path, conn: None }
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

                tx.execute(sql::CREATE_INVENTORY_TABLE, ())
                    .map_err(|_| Error::TableCreation)?;

                tx.commit().map_err(|_| Error::TableCreation)?;

                Ok(())
            }
            None => Err(Error::Connection),
        }
    }

    /// Add product
    pub fn add_product(&self, product: &Product) -> Result<(), Error> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::ADD_PRODUCT,
                    params![
                        &product.name,
                        &product.manufacturer,
                        &product.barcode,
                        &product.categories,
                        &product.unit,
                        &product.weight,
                        &product.price,
                        &product.taxable,
                        &product.description,
                        &product.images,
                        &product.warehouse,
                        &product.added_by,
                        &product.sold,
                        &product.sold_by,
                        &product.added
                    ],
                )
                .map_err(|_| Error::Field)?;
                Ok(())
            }
            None => Err(Error::Connection),
        }
    }

    /// Get product by id
    pub fn get_product_by_id(&self, id: i64) -> Result<Option<Product>, Error> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_PRODUCT_BY_ID)
                    .map_err(|_| Error::SQL)?;
                let mut properties: Vec<Product> = vec![];

                let property_iter = stmt
                    .query_map(params![id], |s| {
                        Ok(Product {
                            name: s.get(0).map_err(|_| Error::Field).unwrap(),
                            manufacturer: s.get(1).map_err(|_| Error::Field).unwrap(),
                            barcode: s.get(2).map_err(|_| Error::Field).unwrap(),
                            categories: s.get(3).map_err(|_| Error::Field).unwrap(),
                            unit: s.get(4).map_err(|_| Error::Field).unwrap(),
                            weight: s.get(5).map_err(|_| Error::Field).unwrap(),
                            price: s.get(6).map_err(|_| Error::Field).unwrap(),
                            taxable: s.get(7).map_err(|_| Error::Field).unwrap(),
                            description: s.get(8).map_err(|_| Error::Field).unwrap(),
                            images: s.get(9).map_err(|_| Error::Field).unwrap(),
                            warehouse: s.get(10).map_err(|_| Error::Field).unwrap(),
                            added_by: s.get(11).map_err(|_| Error::Field).unwrap(),
                            sold: s.get(12).map_err(|_| Error::Field).unwrap(),
                            sold_by: s.get(13).map_err(|_| Error::Field).unwrap(),
                            added: s.get(14).map_err(|_| Error::Field).unwrap(),
                        })
                    })
                    .map_err(|_| Error::Field)?;

                for c in property_iter {
                    properties.push(c.unwrap());
                }

                if properties.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(properties[0].clone()))
                }
            }
            None => Err(Error::Connection),
        }
    }

    /// Get all properties
    pub fn get_all_properties(&self) -> Result<Vec<Product>, Error> {
        match &self.conn {
            Some(conn) => {
                let mut properties: Vec<Product> = vec![];
                let mut stmt = conn.prepare(sql::GET_ALL_PRODUCTS).unwrap();
                let property_iter = stmt
                    .query_map([], |row| {
                        Ok(Product {
                            name: row.get(1).unwrap(),
                            manufacturer: row.get(2).map_err(|_| Error::Field).unwrap(),
                            barcode: row.get(3).map_err(|_| Error::Field).unwrap(),
                            categories: row.get(4).map_err(|_| Error::Field).unwrap(),
                            unit: row.get(5).map_err(|_| Error::Field).unwrap(),
                            weight: row.get(6).map_err(|_| Error::Field).unwrap(),
                            price: row.get(7).map_err(|_| Error::Field).unwrap(),
                            taxable: row.get(8).map_err(|_| Error::Field).unwrap(),
                            description: row.get(9).map_err(|_| Error::Field).unwrap(),
                            images: row.get(10).map_err(|_| Error::Field).unwrap(),
                            warehouse: row.get(11).map_err(|_| Error::Field).unwrap(),
                            added_by: row.get(12).map_err(|_| Error::Field).unwrap(),
                            sold: row.get(13).map_err(|_| Error::Field).unwrap(),
                            sold_by: row.get(14).map_err(|_| Error::Field).unwrap(),
                            added: row.get(15).map_err(|_| Error::Field).unwrap(),
                        })
                    })
                    .unwrap();

                for product in property_iter {
                    properties.push(product.unwrap());
                }

                Ok(properties)
            }
            None => Err(Error::Connection),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    const TEST_DB_PATH: &str = "INVENTORY.sqlite";

    #[test]
    fn connect_db() {
	let mut db = InventoryDatabase::new(TEST_DB_PATH.into());

	// Connect to database
	db.connect();

	if db.conn.is_none() {
	    panic!("Could not connect to database");
	}
    }
    #[test]
    fn test_store_get_property() {
	let mut db = InventoryDatabase::new(TEST_DB_PATH.into());

	let product = Product {
	    name: "Luxury Hill".to_string(),
	    manufacturer: "Apple".to_string(),
	    barcode: "3223223".to_string(),
	    categories: Some("Hardware, Mobile".to_string()),
	    unit: Some("meters".to_string()),
	    weight: None,
	    price: None,
	    taxable: None,
	    description: "Cool product".to_string(),
	    images: "img1.jpg, img2.jpg".to_string(),
	    warehouse: None,
	    added_by: "Jon".to_string(),
	    sold: None,
	    sold_by: None,
	    added: Utc::now(),
	};

	db.connect().unwrap();
	db.add_product(&product).unwrap();

	let pr = db.get_product_by_id(1).unwrap();

	if let Some(p) = pr {
	    assert_eq!(&p.name, &product.name);
	} else {
	    panic!("Could not get product from database");
	}
    }
}
