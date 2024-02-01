//! BizKit Database Management

#![doc(html_favicon_url = "https://bizkit.kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://bizkit.kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub use inventory::InventoryDatabase;
pub use error::Error;
pub use staff::Database;

mod inventory;
mod staff;
mod error;
