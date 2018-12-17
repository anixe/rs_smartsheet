use {Result, SheetId};
use dto::{ApiResult, Error, IndexResult, Row, Sheet, SheetHeader};
use reqwest::{Client as ReqwestClient, RequestBuilder};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use std::rc::Rc;

const DEFAULT_URL: &str = "https://api.smartsheet.com/2.0";
const QUERY_DO_NOT_PAGINATE: &[(&str, &str)] = &[("includeAll", "true")];

#[derive(Clone, Debug)]
pub struct Client {
    url: Rc<str>,
    token: Rc<str>,
}

impl Client {
    pub fn new<T: Into<String>>(token: T) -> Self {
        Self::new_with_url(DEFAULT_URL, token)
    }

    pub fn new_with_url<T: Into<String>, U: Into<String>>(url: T, token: U) -> Self {
        Self {
            url: url.into().into(),
            token: token.into().into(),
        }
    }

    crate fn fetch_sheet(&self, id: &SheetId) -> Result<Sheet> {
        let builder = ReqwestClient::new()
            .get(&format!("{}/sheets/{}", self.url, id));
        self.fetch_json(builder)
    }

    pub fn fetch_sheets(&self) -> Result<Vec<SheetHeader>> {
        let builder = ReqwestClient::new()
            .get(&format!("{}/sheets", self.url))
            .query(QUERY_DO_NOT_PAGINATE);
        let result: IndexResult<_> = self.fetch_json(builder)?;
        Ok(result.into_data())
    }

    crate fn update_cell(&self, sheet_id: &SheetId, row: Row) -> Result<Vec<Row>> {
        let builder = ReqwestClient::new()
            .put(&format!("{}/sheets/{}/rows", self.url, sheet_id))
            .json(&row);
        let result: ApiResult<_> = self.fetch_json(builder)?;
        Ok(result.result)
    }

    pub fn get_json<T: DeserializeOwned>(&self, urn: &str) -> Result<T> {
        let builder = ReqwestClient::new()
            .get(&format!("{}/{}", self.url, urn));
        self.fetch_json(builder)
    }

    pub fn post_json<S: Serialize + ?Sized, T: DeserializeOwned>(&self, urn: &str, body: &S) -> Result<T> {
        let builder = ReqwestClient::new()
            .post(&format!("{}/{}", self.url, urn))
            .json(body);
        self.fetch_json(builder)
    }

    fn fetch_json<T: DeserializeOwned>(&self, builder: RequestBuilder) -> Result<T> {
        let response = builder.bearer_auth(&self.token)
            .send()?;
        if !response.status().is_success() {
            let error: Error = serde_json::from_reader(response)?;
            return Err(error.into())
        }
        Ok(serde_json::from_reader(response)?)
    }
}

#[cfg(test)]
impl Client {
    pub fn new_mocked() -> Self {
        Self::new_with_url(::mockito::SERVER_URL, "TEST_TOKEN")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Error as ResError;
    use mockito::{self, Matcher, Mock};

    mod fetch_sheets {
        use super::*;

        #[test]
        fn returns_all_sheets() {
            let mock = mockito::mock("GET", "/sheets?includeAll=true")
                .match_header("authorization", "Bearer TEST_TOKEN")
                .with_body(r#"{
                        "data": [
                            {
                                "id": 11,
                                "name": "my_sheet"
                            },
                            {
                                "id": 12,
                                "name": "my_other_sheet"
                            }
                        ]
                    }"#)
                .create();
            let client = Client::new_mocked();

            let result = client.fetch_sheets();

            mock.assert();
            let actual = result.unwrap();
            assert_eq!(2, actual.len());
            assert_eq!("my_sheet", actual[0].get_name());
            assert_eq!(SheetId::from(11), actual[0].get_sheet_id());
            assert_eq!("my_other_sheet", actual[1].get_name());
            assert_eq!(SheetId::from(12), actual[1].get_sheet_id());
        }
    }

    mod get_json {
        use super::*;

        #[test]
        fn returns_all_sheets() {
            let mock = mockito::mock("GET", "/new_route")
                .match_header("authorization", "Bearer TEST_TOKEN")
                .with_body(r#"{
                        "result": null
                    }"#)
                .create();
            let client = Client::new_mocked();

            let result = client.get_json::<ApiResult<()>>("new_route");

            mock.assert();
            result.unwrap();
        }
    }

    mod post_json {
        use super::*;

        #[test]
        fn returns_all_sheets() {
            let mock = mockito::mock("POST", "/new_route")
                .match_header("authorization", "Bearer TEST_TOKEN")
                .match_header("content-type", "application/json")
                .match_body(Matcher::Json(serde_json::from_str(r#"{
                        "key": "value"
                    }"#).unwrap()))
                .with_body(r#"{
                        "result": null
                    }"#)
                .create();
            let client = Client::new_mocked();

            let result = client.post_json::<_, ApiResult<()>>("new_route", &serde_json::from_str::<serde_json::Value>(r#"{
                    "key": "value"
                }"#).unwrap());

            mock.assert();
            result.unwrap();
        }
    }

    mod fetch_json {
        use super::*;

        fn create_sheets_mock(with_status: usize, with_body: &str) -> Mock {
            mockito::mock("GET", "/sheets?includeAll=true")
                .match_header("authorization", "Bearer TEST_TOKEN")
                .with_status(with_status)
                .with_body(with_body)
                .create()
        }

        mod when_api_returns_valid_response {
            use super::*;

            #[test]
            fn then_returns_dto() {
                let mock = create_sheets_mock(200, r#"{
                        "data": [
                            {
                                "id": 123,
                                "name": "my_sheet"
                            }
                        ]
                    }"#);
                let client = Client::new_mocked();

                let result = client.fetch_sheets();

                mock.assert();
                assert!(result.is_ok());
            }
        }

        mod when_api_returns_error {
            use super::*;

            #[test]
            fn then_returns_error() {
                let mock = create_sheets_mock(500, r#"{
                        "errorCode": 4004,
                        "message": "Test error"
                    }"#);
                let client = Client::new_mocked();

                let result = client.fetch_sheets();

                mock.assert();
                let actual = result.unwrap_err();
                let expected = ResError::SmartsheetOther {
                    code: 4004,
                    message: "Test error".to_string(),
                };
                assert_eq!(expected, actual);
            }
        }

        mod when_api_returns_unexpected_json {
            use super::*;

            #[test]
            fn then_returns_error() {
                let mock = create_sheets_mock(200, r#"{
                        "Unexpected": "data"
                    }"#);
                let client = Client::new_mocked();

                let result = client.fetch_sheets();

                mock.assert();
                let actual = result.unwrap_err();
                let expected = ResError::InvalidJson("missing field `data` at line 3 column 21".to_string());
                assert_eq!(expected, actual);
            }
        }

        mod when_api_is_unreachable {
            use super::*;

            #[test]
            fn then_returns_error() {
                // Port 9 always discards all data
                let client = Client::new_with_url("http://127.0.0.1:9/", "TEST_TOKEN");

                let result = client.fetch_sheets();

                let actual = result.unwrap_err();
                match actual {
                    ResError::Network(_) => (),
                    _ => panic!("Invalid error: '{:?}'", actual),
                }
            }
        }
    }
}
