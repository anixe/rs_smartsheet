use SheetId;

#[derive(Debug, Deserialize)]
pub struct Sheet {
    id: SheetId,
    name: String
}

impl Sheet {
    pub fn get_sheet_id(&self) -> SheetId {
        self.id.clone()
    }
}
