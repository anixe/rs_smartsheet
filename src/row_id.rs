#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct RowId {
    id: u64,
}

impl From<u64> for RowId {
    fn from(id: u64) -> Self {
        RowId { id }
    }
}
