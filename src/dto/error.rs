#[derive(Debug, Deserialize)]
pub struct Error {
    #[serde(rename="errorCode")]
    code: u64,
    message: String,
}

impl Error {
    pub fn get_code(&self) -> u64 {
        self.code
    }

    pub fn into_message(self) -> String {
        self.message
    }
}
