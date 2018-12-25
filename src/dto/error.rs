#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    error_code: u64,
    message: String,
}

impl Error {
    pub fn get_code(&self) -> u64 {
        self.error_code
    }

    pub fn into_message(self) -> String {
        self.message
    }
}
