use kong::{inputs::UserInput, json, validate::ValidationError, JsonValue};
use serde::{Deserialize, Serialize};

use super::ID;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProductImages {
    pub raw: Vec<Vec<u8>>,
    pub mimes: Vec<String>,
    pub version: i16,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProductImagesDatabaseInput {
    pub product: ID,
    pub raw: Vec<Vec<u8>>,
    pub mimes: Vec<String>,
    pub version: i16,
}
