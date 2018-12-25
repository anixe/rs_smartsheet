use SheetId;

#[derive(Debug, Deserialize)]
pub struct SheetHeader {
    id: SheetId,
    name: String
}

impl SheetHeader {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_sheet_id(&self) -> SheetId {
        self.id.clone()
    }
}
