use {Client, Error, Result};
use dto::Sheet;

#[derive(Debug)]
pub struct Smartsheet {
    client: Client,
    sheet: Sheet,
}

impl Smartsheet {
    pub fn new(client: &Client, sheet_name: &str) -> Result<Smartsheet> {
        let sheet_id = client.fetch_sheets()?
            .into_data()
            .iter()
            .find(|sheet| sheet.get_name() == sheet_name)
            .ok_or_else(|| Error::InvalidSheetName(sheet_name.to_string()))?
            .get_sheet_id();
        let sheet = client.fetch_sheet(sheet_id)?;
        Ok(Smartsheet {
            client: client.clone(),
            sheet,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{self, Mock};

    mod new {
        use super::*;

        fn mock_sheets() -> Mock {
            mockito::mock("GET", "/sheets?includeAll=true")
                .match_header("authorization", "Bearer TEST_TOKEN")
                .with_body(r#"{
                        "data": [
                            {
                                "id": 123,
                                "name": "my_sheet"
                            }
                        ]
                    }"#)
                .create()
        }

        mod when_sheet_exists {
            use super::*;

            #[test]
            fn then_returns_smartsheet() {
                let mock_sheets = mock_sheets();
                let mock_sheet = mockito::mock("GET", "/sheets/123")
                    .match_header("authorization", "Bearer TEST_TOKEN")
                    .with_body(r#"{
                            "id": 123,
                            "name": "my_sheet"
                        }"#)
                    .create();
                let client = Client::new_mocked();

                let result = Smartsheet::new(&client, "my_sheet");

                mock_sheets.assert();
                mock_sheet.assert();
                let actual = result.unwrap();
                assert_eq!("123", actual.sheet.get_sheet_id().to_string());
            }
        }

        mod when_sheet_does_not_exist {
            use super::*;

            #[test]
            fn then_returns_error() {
                let mock = mock_sheets();
                let client = Client::new_mocked();

                let result = Smartsheet::new(&client, "nonexistent");

                mock.assert();
                let actual = result.unwrap_err();
                let expected = Error::InvalidSheetName("nonexistent".to_string());
                assert_eq!(expected, actual);
            }
        }
    }
}
