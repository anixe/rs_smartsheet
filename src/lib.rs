#[cfg(test)]
extern crate mockito;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

mod client;
mod column_id;
mod dto;
mod error;
mod row_id;
mod sheet_id;
mod smartsheet;

pub use self::client::Client;
pub use self::dto::{CellValue, Row};
pub use self::column_id::ColumnId;
pub use self::error::Error;
pub use self::row_id::RowId;
pub use self::sheet_id::SheetId;
pub use self::smartsheet::Smartsheet;

type Result<T> = ::std::result::Result<T, Error>;
