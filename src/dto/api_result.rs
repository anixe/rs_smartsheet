use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize)]
pub struct ApiResult<T: DeserializeOwned> {
    // Annotation required because of https://github.com/serde-rs/serde/issues/1296
    // Remove when https://github.com/rust-lang/rust/issues/41617 is fixed
    #[serde(bound = "")]
    pub result: T,
}
