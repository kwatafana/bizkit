//! BizKit Core Data Types

#![doc(html_favicon_url = "https://bizkit.kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://bizkit.kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub use staff::Staff;
pub use staff_status::StaffStatus;
pub use product::Product;
pub use error::Error;

mod product;
mod staff;
mod error;
mod staff_status;
