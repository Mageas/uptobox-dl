#[derive(Debug)]
pub enum ClientError {
    BadLink(String),
    BadRequest(reqwest::Error),
    Deserialize(serde_json::Error),
    RetrieveLink(String),
    DownloadFile(String),
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        Self::BadRequest(error)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(error: serde_json::Error) -> Self {
        Self::Deserialize(error)
    }
}
