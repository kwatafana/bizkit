//! bizkit -- Product management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
/// Generic product
pub struct Product {
    /// Name of the product
    pub name: String,
    /// Manufacturer of the product
    pub manufacturer: String,
    /// Barcode
    pub barcode: String,
    /// Categories that the product belongs to
    pub categories: Option<String>,
    /// Unit of measurements
    pub unit: Option<String>,
    /// Weight of the product
    pub weight: Option<f64>,
    /// The price of the product
    pub price: Option<f64>,
    /// Is the product taxable or not
    pub taxable: Option<bool>,
    /// Product description
    pub description: String,
    /// Product Images
    pub images: String,
    /// Warehouse of the product
    pub warehouse: Option<String>,
    /// User who added the product
    pub added_by: String,
    /// Is the product sold or not
    pub sold: Option<DateTime<Utc>>,
    /// User who sold the product
    pub sold_by: Option<String>,
    /// Date when product was added,
    pub added: DateTime<Utc>,
}
