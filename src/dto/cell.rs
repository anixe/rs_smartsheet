use {CellValue, ColumnId};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    column_id: ColumnId,
    #[serde(default)]
    value: CellValue,
}

impl Cell {
    pub fn new(column_id: ColumnId, value: CellValue) -> Cell {
        Cell { column_id, value, }
    }

    pub fn get_column_id(&self) -> ColumnId {
        self.column_id.clone()
    }

    pub fn get_value(&self) -> &CellValue {
        &self.value
    }
}
