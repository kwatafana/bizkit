//! BizKit Core Data Types

#![doc(html_favicon_url = "https://bizkit.kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://bizkit.kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub use error::Error;
pub use staff::Staff;
pub use staff_status::StaffStatus;

mod error;
mod staff;
mod staff_status;
pub use categories::{Category, CategoryDatabaseInput, CategoryInput};
pub use product_categories::{ProductCategories, ProductCategoriesDatabaseInput};
pub use product_images::{ProductImages, ProductImagesDatabaseInput};
pub use products::{Product, ProductDatabaseInput, ProductImage, ProductInput};

mod categories;
mod product_categories;
mod product_images;
mod products;
pub mod versions;

/// Entity unique identifier
pub type ID = i32;
