use super::ID;
use chrono::{Datelike, NaiveDate};
use kong::{inputs::UserInput, json, validate::ValidationError, JsonValue};
use rust_decimal::Decimal as Money;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Product {
    pub id: ID,
    pub name: String,
    pub manufacturer: String,
    pub barcode: String,
    pub unit: Option<String>,
    pub weight: Option<f32>,
    pub price: Option<Money>,
    pub taxable: Option<bool>,
    pub description: String,
    pub images: Vec<ProductImage>,
    pub sold: bool,
    pub category: Option<ID>,
    pub added: NaiveDate,
    pub version: i16,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProductImage {
    pub raw: String,
    pub mime: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProductDatabaseInput {
    pub name: String,
    pub manufacturer: String,
    pub barcode: String,
    pub unit: Option<String>,
    pub weight: Option<f32>,
    pub price: Option<Money>,
    pub taxable: Option<bool>,
    pub description: String,
    pub sold: bool,
    pub added: NaiveDate,
    pub version: i16,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProductInput {
    pub name: String,
    pub manufacturer: String,
    pub barcode: String,
    pub unit: Option<String>,
    pub weight: Option<f32>,
    pub price: Option<Money>,
    pub taxable: Option<bool>,
    pub description: String,
    pub raw_images: Vec<Vec<u8>>,
    pub images_mime_types: Vec<String>,
    pub category: Option<ID>,
}

impl ProductInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "name": self.name,
            "manufacturer": self.manufacturer,
            "barcode": self.barcode,
            "unit": self.unit,
            "weight": self.weight,
            "price": self.price,
            "taxable": self.taxable,
            "description": self.description,
            "raw_images": self.raw_images,
            "images_mime_types": self.images_mime_types,
            "category": self.category
        })
    }
}

impl UserInput for ProductInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        // TODO: validate input properly
        Ok(())
    }
}

impl From<ProductInput> for ProductDatabaseInput {
    fn from(input: ProductInput) -> Self {
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        let month = current_date.month();
        let day = current_date.day();
        if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Self {
                name: input.name,
                manufacturer: input.manufacturer,
                barcode: input.barcode,
                unit: input.unit,
                weight: input.weight,
                price: input.price,
                taxable: input.taxable,
                description: input.description,
                sold: false,
                added: date,
                version: crate::versions::PRODUCT_SCHEMA,
            }
        } else {
            panic!("Date error");
        }
    }
}
