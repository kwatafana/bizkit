//! # Add product kontroller

use bizkitdata::{
    ProductCategoriesDatabaseInput, ProductDatabaseInput, ProductImagesDatabaseInput, ProductInput,
    ID,
};
use bizkitdatabase::{DatabaseError, ProductCategoriesSql, ProductImagesSql, ProductsSql};
use chrono::Datelike;
use kong::{
    inputs::UserInput, json_from_str, server, ErrorResponse, JsonError, JsonValue, Kong, Kontrol,
    Method,
};
use kong_kontrollers::accounts::database::DB_Type;
use kong_kontrollers::login::is_admin;
use postgres::Client;
use rust_decimal::prelude::*;
use rust_decimal::Decimal as Money;

/// Add product kontroller
pub struct AddProductKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: DB_Type,
}

impl AddProductKontroller {
    /// Store uploaded property photos
    fn store_photos(
        dir_name: &str,
        photos: Vec<server::input::post::BufferedFile>,
    ) -> std::io::Result<String> {
        let directory = format!("./www/uploads/products_photos/{dir_name}");
        // file paths to store in database
        let mut file_paths = "".to_string();

        // create upload directory if it does not exist
        std::fs::create_dir_all(&directory)?;

        for photo in photos {
            let photo_name = if let Some(photo_name) = &photo.filename {
                photo_name.clone()
            } else {
                "".to_string()
            };
            let timestamp = chrono::Utc::now().timestamp();
            let file = format!("{directory}/{timestamp}-{photo_name}");

            // Store photos in directory
            std::fs::write(&file, &photo.data)?;

            // remeber file path
            let directory = format!("uploads/products_photos/{dir_name}");
            let file = format!("{directory}/{timestamp}-{photo_name}");
            if file_paths == "" {
                file_paths = file;
            } else {
                file_paths = format!("{file_paths},{file}");
            }
        }

        Ok(file_paths)
    }
}

impl Kontrol for AddProductKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    fn get_input(&self, request: &server::Request) -> Option<JsonValue> {
        let input = server::post_input!(request, {
            name: String,
            manufacturer: String,
            barcode: String,
            unit: Option<String>,
            weight: Option<f32>,
            price: Option<String>,
            taxable: Option<bool>,
            description: String,
            photo_0: Option<server::input::post::BufferedFile>,
            photo_1: Option<server::input::post::BufferedFile>,
            photo_2: Option<server::input::post::BufferedFile>,
            photo_3: Option<server::input::post::BufferedFile>,
            photo_4: Option<server::input::post::BufferedFile>,
            category: Option<ID>
        });

        if let Ok(input) = input {
            let mut raws = vec![];
            let mut mimes = vec![];

            // extract Photo 0
            if let Some(photo_0) = input.photo_0 {
                raws.push(photo_0.data);
                mimes.push(photo_0.mime);
            }

            // extract Photo 1
            if let Some(photo_1) = input.photo_1 {
                raws.push(photo_1.data);
                mimes.push(photo_1.mime);
            }
            // extract Photo 2
            if let Some(photo_2) = input.photo_2 {
                raws.push(photo_2.data);
                mimes.push(photo_2.mime);
            }
            // extract Photo 3
            if let Some(photo_3) = input.photo_3 {
                raws.push(photo_3.data);
                mimes.push(photo_3.mime);
            }

            // extract Photo 4
            if let Some(photo_4) = input.photo_4 {
                raws.push(photo_4.data);
                mimes.push(photo_4.mime);
            }

            let current_date = chrono::Utc::now();
            let year = current_date.year();
            let month = current_date.month();
            let day = current_date.day();
            let price: Option<Money> = if let Some(price) = input.price {
                let d = Money::from_str(&price);

                match d {
                    Ok(d) => Some(d),
                    _ => None,
                }
            } else {
                None
            };

            if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                let input = ProductInput {
                    name: input.name,
                    manufacturer: input.manufacturer,
                    barcode: input.barcode,
                    unit: input.unit,
                    weight: input.weight,
                    price: price,
                    taxable: input.taxable,
                    description: input.description,
                    raw_images: raws,
                    images_mime_types: mimes,
                    category: input.category,
                };

                Some(input.as_json())
            } else {
                None
            }
        } else {
            // Invalid input
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            let res: Result<ProductInput, JsonError> = json_from_str(&input.to_string());

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

    /// Add product
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(k) = &kong.kpassport {
            if let Ok(admin) = is_admin(k, self.database.clone()) {
                if admin {
                    if let Some(input) = &kong.input {
                        let res: Result<ProductInput, JsonError> =
                            json_from_str(&input.to_string());
                        let category;
                        let raws;
                        let mimes;

                        // Derive product from string
                        let product: ProductDatabaseInput = if let Ok(input) = res {
                            raws = input.raw_images.clone();
                            mimes = input.images_mime_types.clone();
                            category = input.category;

                            // Images cannot be empty
                            if input.raw_images.is_empty() {
                                return ErrorResponse::bad_request();
                            }

                            input.into()
                        } else {
                            return ErrorResponse::bad_request();
                        };

                        let mut client = self.database.lock().unwrap();

                        // Store product into the database
                        let res = ProductsSql::create(&mut client, product.clone());

                        match res {
                            Ok(()) => {
                                // store images
                                let res = store_images(&mut client, &product.barcode, raws, mimes);

                                match res {
                                    Ok(()) => {
                                        if let Some(category) = category {
                                            // Store categories
                                            let res = store_category(
                                                &mut client,
                                                &product.barcode,
                                                category,
                                            );
                                            match res {
                                                Ok(()) => server::Response::json(&product)
                                                    .with_status_code(201),
                                                Err(_) => {
                                                    // TODO: An error occured, revert changes
                                                    ErrorResponse::internal()
                                                }
                                            }
                                        } else {
                                            server::Response::json(&product).with_status_code(201)
                                        }
                                    }
                                    Err(_) => {
                                        // TODO: An error occured, revert changes by deleting the
                                        // images

                                        ErrorResponse::internal()
                                    }
                                }
                            }
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

/// Store product category in joining table

fn store_category(client: &mut Client, barcode: &str, category: ID) -> Result<(), DatabaseError> {
    let product_id = ProductsSql::read_by_id_barcode(client, barcode)?;
    match product_id {
        Some(id) => {
            let prod_cat = ProductCategoriesDatabaseInput {
                product: id,
                category,
                version: bizkitdata::versions::PRODUCT_CATEGORIES_SCHEMA,
            };
            ProductCategoriesSql::create(client, prod_cat)?;
            Ok(())
        }
        None => Err(DatabaseError::SQL),
    }
}

/// Store product category in joining table
fn store_images(
    client: &mut Client,
    barcode: &str,
    raw: Vec<Vec<u8>>,
    mimes: Vec<String>,
) -> Result<(), DatabaseError> {
    let product_id = ProductsSql::read_by_id_barcode(client, barcode)?;

    match product_id {
        Some(id) => {
            let prod_img = ProductImagesDatabaseInput {
                product: id,
                raw,
                mimes,
                version: bizkitdata::versions::PRODUCT_IMAGES_SCHEMA,
            };

            ProductImagesSql::create(client, prod_img)?;
            Ok(())
        }
        None => Err(DatabaseError::SQL),
    }
}
