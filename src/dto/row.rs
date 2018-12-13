use {CellValue, ColumnId, RowId};
use dto::Cell;

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    id: RowId,
    cells: Vec<Cell>,
}

impl Row {
    pub fn new(id: RowId, cell: Cell) -> Self {
        Row {
            id,
            cells: vec![cell],
        }
    }

    pub fn get_cell_value(&self, column_id: &ColumnId) -> Option<&CellValue> {
        self.cells.iter()
            .find(|cell| &cell.get_column_id() == column_id)
            .map(|cell| cell.get_value())
    }

    pub fn get_row_id(&self) -> RowId {
        self.id.clone()
    }
}
