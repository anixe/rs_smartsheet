use ColumnId;

#[derive(Debug, Deserialize)]
pub struct Column {
    id: ColumnId,
    title: String,
}

impl Column {
    pub fn get_column_id(&self) -> ColumnId {
        self.id.clone()
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
}
