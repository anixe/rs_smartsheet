use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct SheetId {
    id: u64,
}

impl From<u64> for SheetId {
    fn from(id: u64) -> Self {
        SheetId { id }
    }
}

impl Display for SheetId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.id.fmt(f)
    }
}
