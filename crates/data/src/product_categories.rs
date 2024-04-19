use kong::{inputs::UserInput, json, validate::ValidationError, JsonValue};
use serde::{Deserialize, Serialize};

use super::ID;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProductCategories {
    pub id: ID,
    pub product: ID,
    pub category: ID,
    pub version: i16,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProductCategoriesDatabaseInput {
    pub product: ID,
    pub category: ID,
    pub version: i16,
}
