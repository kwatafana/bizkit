use kong::{inputs::UserInput, json, validate::ValidationError, JsonValue};
use serde::{Deserialize, Serialize};

use super::ID;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Category {
    pub id: ID,
    pub name: String,
    pub version: i16,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CategoryDatabaseInput {
    pub name: String,
    pub version: i16,
}

/// Course input from user
#[derive(Serialize, Deserialize, Clone)]
pub struct CategoryInput {
    /// The name of the category
    pub name: String,
}

impl CategoryInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "name": self.name,
        })
    }
}

impl UserInput for CategoryInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        // TODO: validate input properly
        Ok(())
    }
}

impl From<CategoryInput> for CategoryDatabaseInput {
    fn from(input: CategoryInput) -> Self {
        Self {
            name: input.name,
            version: crate::versions::CATEGORY_SCHEMA,
        }
    }
}
