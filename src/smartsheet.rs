use {CellValue, Client, ColumnId, Error, Result, Row, RowId};
use dto::{Cell, Sheet};

#[derive(Debug)]
pub struct Smartsheet {
    client: Client,
    sheet: Sheet,
}

impl Smartsheet {
    pub fn fetch(client: &Client, sheet_name: &str) -> Result<Smartsheet> {
        let sheet_id = client.fetch_sheets()?
            .iter()
            .find(|sheet| sheet.get_name() == sheet_name)
            .ok_or_else(|| Error::InvalidSheetName(sheet_name.to_string()))?
            .get_sheet_id();
        let sheet = client.fetch_sheet(&sheet_id)?;
        Ok(Smartsheet {
            client: client.clone(),
            sheet,
        })
    }

    pub fn get_column_id(&self, title: &str) -> Option<ColumnId> {
        self.sheet.get_column_id(title)
    }

    pub fn find_row_id(&self, predicate: impl FnMut(&Row) -> bool) -> Option<RowId> {
        self.sheet.find_row_id(predicate)
    }

    pub fn push_cell_value(&mut self, column_id: &ColumnId, row_id: &RowId, cell_value: impl Into<CellValue>) -> Result<()> {
        let sheet_id = self.sheet.get_sheet_id();
        let cell = Cell::new(column_id.clone(), cell_value.into());
        let row = Row::new(row_id.clone(), cell);
        let updated_rows = self.client.update_cell(&sheet_id, row)?;
        self.sheet.update_rows(updated_rows);
        Ok(())
    }

    pub fn get_cell_value(&self, column_id: &ColumnId, row_id: &RowId) -> Option<&CellValue> {
        self.sheet.get_cell_value(column_id, row_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{self, Matcher, Mock};

    fn mock_sheets() -> Mock {
        mockito::mock("GET", "/sheets?includeAll=true")
            .match_header("authorization", "Bearer TEST_TOKEN")
            .with_body(json!({
                        "data": [
                            {
                                "id": 11,
                                "name": "my_sheet"
                            }
                        ]
                    }).to_string())
            .create()
    }

    fn mock_sheet() -> Mock {
        mockito::mock("GET", "/sheets/11")
            .match_header("authorization", "Bearer TEST_TOKEN")
            .with_body(json!({
                    "id": 11,
                    "name": "my_sheet",
                    "columns": [
                        {
                            "id": 21,
                            "title": "my_column"
                        },
                        {
                            "id": 22,
                            "title": "other_column"
                        }
                    ],
                    "rows": [
                        {
                            "id": 31,
                            "cells": [
                                {
                                    "columnId": 21,
                                    "value": "data_21_31"
                                },
                                {
                                    "columnId": 22,
                                    "value": "data_22_31"
                                }
                            ]
                        },
                        {
                            "id": 32,
                            "cells": [
                                {
                                    "columnId": 21,
                                    "value": "data_21_32"
                                }
                            ]
                        }
                    ]
                }).to_string())
            .create()
    }

    fn create_smartsheet() -> Smartsheet {
        let _mock_sheets = mock_sheets();
        let _mock_sheet = mock_sheet();
        let client = Client::new_mocked();
        Smartsheet::fetch(&client, "my_sheet")
            .unwrap()
    }

    fn assert_cell_value(expected_str: &str, smartsheet: &Smartsheet, column_id: u64, row_id: u64) {
        let assert_name = format!("for column {} and row {}", column_id, row_id);
        let expected = &CellValue::from(expected_str);
        let actual = smartsheet.get_cell_value(&ColumnId::from(column_id), &RowId::from(row_id))
            .expect(&format!("Cell not found {}", assert_name));
        assert_eq!(expected, actual, "Invalid cell value {}", assert_name);
    }

    mod fetch {
        use super::*;

        mod when_sheet_exists {
            use super::*;

            #[test]
            fn then_returns_smartsheet() {
                let mock_sheets = mock_sheets();
                let mock_sheet = mock_sheet();
                let client = Client::new_mocked();

                let result = Smartsheet::fetch(&client, "my_sheet");

                mock_sheets.assert();
                mock_sheet.assert();
                let actual = result.unwrap();
                assert_eq!("11", actual.sheet.get_sheet_id().to_string());
            }
        }

        mod when_sheet_does_not_exist {
            use super::*;

            #[test]
            fn then_returns_error() {
                let mock = mock_sheets();
                let client = Client::new_mocked();

                let result = Smartsheet::fetch(&client, "nonexistent");

                mock.assert();
                let actual = result.unwrap_err();
                let expected = Error::InvalidSheetName("nonexistent".to_string());
                assert_eq!(expected, actual);
            }
        }
    }

    mod get_column_id {
        use super::*;

        mod when_column_exists {
            use super::*;

            #[test]
            fn then_returns_id() {
                let smartsheet = create_smartsheet();

                let result = smartsheet.get_column_id("my_column");

                let expected = ColumnId::from(21);
                let actual = result.unwrap();
                assert_eq!(expected, actual);
            }
        }

        mod when_column_does_not_exist {
            use super::*;

            #[test]
            fn then_returns_none() {
                let smartsheet = create_smartsheet();

                let result = smartsheet.get_column_id("nonexistent");

                assert!(result.is_none());
            }
        }
    }

    mod find_row_id {
        use super::*;

        mod when_row_is_found {
            use super::*;

            #[test]
            fn then_returns_id() {
                let smartsheet = create_smartsheet();
                let column_id = &ColumnId::from(21);
                let searched = &CellValue::from("data_21_31");

                let result = smartsheet.find_row_id(|row| row.get_cell_value(column_id) == Some(searched));

                let expected = RowId::from(31);
                let actual = result.unwrap();
                assert_eq!(expected, actual);
            }
        }

        mod when_row_is_not_found {
            use super::*;

            #[test]
            fn then_returns_none() {
                let smartsheet = create_smartsheet();
                let column_id = &ColumnId::from(21);
                let searched = &CellValue::from("nonexistent");

                let result = smartsheet.find_row_id(|row| row.get_cell_value(column_id) == Some(searched));

                assert!(result.is_none());
            }
        }
    }

    mod push_cell_value {
        use super::*;

        #[test]
        fn updates_sheet() {
            let mut smartsheet = create_smartsheet();
            let mockito = mockito::mock("PUT", "/sheets/11/rows")
                .match_header("authorization", "Bearer TEST_TOKEN")
                .match_header("content-type", "application/json")
                .match_body(Matcher::Json(json!({
                    "id":31,
                    "cells": [
                        {
                            "columnId": 21,
                            "value": "new_data"
                        }
                    ]
                })))
                .with_body(json!({
                       "result": [
                            {
                                "id": 31,
                                "cells": [
                                    {
                                        "columnId": 21,
                                        "value": "new_data"
                                    },
                                    {
                                        "columnId": 22,
                                        "value": "data_22_31"
                                    }
                                ]
                            }
                       ]
                    }).to_string())
                .create();
            let column_id = &ColumnId::from(21);
            let row_id = &RowId::from(31);
            let cell_value = CellValue::from("new_data");

            let result = smartsheet.push_cell_value(column_id, row_id, cell_value);

            mockito.assert();
            assert!(result.is_ok());
            assert_cell_value("new_data",   &smartsheet, 21, 31);
            assert_cell_value("data_22_31", &smartsheet, 22, 31);
            assert_cell_value("data_21_32", &smartsheet, 21, 32);
        }
    }
}
