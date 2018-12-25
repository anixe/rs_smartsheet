#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ColumnId {
    id: u64,
}

impl From<u64> for ColumnId {
    fn from(id: u64) -> Self {
        ColumnId { id }
    }
}
