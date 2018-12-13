use {CellValue, ColumnId, Row, RowId, SheetId};
use dto::{Column, SheetHeader};
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Sheet {
    #[serde(flatten)]
    header: SheetHeader,
    columns: Vec<Column>,
    #[serde(deserialize_with = "deserialize_rows")]
    rows: BTreeMap<RowId, Row>,
}

impl Sheet {
    pub fn get_sheet_id(&self) -> SheetId {
        self.header.get_sheet_id()
    }

    pub fn get_column_id(&self, title: &str) -> Option<ColumnId> {
        self.columns.iter()
            .find(|column| column.get_title() == title)
            .map(|column| column.get_column_id())
    }

    pub fn find_row_id(&self, mut predicate: impl FnMut(&Row) -> bool) -> Option<RowId> {
        self.rows.values()
            .find(|row| predicate(*row))
            .map(|row| row.get_row_id())
    }

    pub fn update_rows(&mut self, rows: impl IntoIterator<Item = Row>) {
        let extended = rows.into_iter()
            .map(|row| (row.get_row_id(), row));
        self.rows.extend(extended);
    }

    pub fn get_cell_value(&self, column_id: &ColumnId, row_id: &RowId) -> Option<&CellValue> {
        self.rows.get(row_id)?
            .get_cell_value(column_id)
    }
}

fn deserialize_rows<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BTreeMap<RowId, Row>, D::Error> {
    let rows = Vec::<Row>::deserialize(deserializer)?
        .into_iter()
        .map(|row| (row.get_row_id(), row))
        .collect();
    Ok(rows)
}
