#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Bool(bool),
    Empty,
}

impl CellValue {
    pub fn as_text(&self) -> Option<&str> {
        match *self {
            CellValue::Text(ref text) => Some(text),
            _ => None,
        }
    }
}

impl Default for CellValue {
    fn default() -> CellValue {
        CellValue::Empty
    }
}

impl From<String> for CellValue {
    fn from(string: String) -> CellValue {
        CellValue::Text(string)
    }
}

impl<'a> From<&'a str> for CellValue {
    fn from(string: &str) -> CellValue {
        CellValue::Text(string.to_string())
    }
}
