use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiLinkResponse {
    #[serde(rename(deserialize = "statusCode"))]
    pub status_code: usize,
    pub message: String,
    pub data: ApiLinkData,
}

#[derive(Debug, Deserialize)]
pub struct ApiLinkData {
    #[serde(rename(deserialize = "dlLink"))]
    pub dl_link: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiLinkResponseError {
    #[serde(rename(deserialize = "statusCode"))]
    pub status_code: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiFileResponse {
    #[serde(rename(deserialize = "statusCode"))]
    pub status_code: usize,
    pub message: String,
    pub data: ApiFileData,
}

#[derive(Debug, Deserialize)]
pub struct ApiFileData {
    pub list: Vec<ApiFileList>,
}

#[derive(Debug, Deserialize)]
pub struct ApiFileList {
    pub file_name: String,
}
