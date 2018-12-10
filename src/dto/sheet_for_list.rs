use SheetId;

#[derive(Debug, Deserialize)]
pub struct SheetForList {
    id: SheetId,
    name: String
}

impl SheetForList {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_sheet_id(&self) -> SheetId {
        self.id.clone()
    }
}
