#[cfg(test)]
extern crate mockito;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod client;
mod dto;
mod error;
mod sheet_id;
mod smartsheet;

use self::client::Client;

pub use self::error::Error;
pub use self::sheet_id::SheetId;
pub use self::smartsheet::Smartsheet;

type Result<T> = ::std::result::Result<T, Error>;
