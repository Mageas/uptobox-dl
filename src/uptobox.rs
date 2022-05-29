use crate::api::*;
use crate::error::ClientError;

use std::cmp::min;
use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client as RequestClient;

#[derive(Debug)]
pub struct Client<'a> {
    file_code: &'a str,
    user_token: &'a str,
}

impl Client<'_> {
    pub fn new<'a>(file_code: &'a str, user_token: &'a str) -> Client<'a> {
        Client {
            file_code,
            user_token,
        }
    }

    pub async fn get_download_link(&self) -> Result<String, ClientError> {
        let response = reqwest::get(format!(
            "https://uptobox.com/api/link?token={}&file_code={}",
            self.user_token, self.file_code
        ))
        .await?
        .text()
        .await?;

        let des = serde_json::from_str::<ApiLinkResponse>(&response);

        let response = match des {
            Ok(r) => r,
            Err(_) => match serde_json::from_str::<ApiLinkResponseError>(&response) {
                Ok(r) => return Err(ClientError::BadLink(format!("{:?}", r.message))),
                Err(e) => return Err(ClientError::Deserialize(e)),
            },
        };

        Ok(response.data.dl_link)
    }

    pub async fn get_file_name(&self) -> Result<String, ClientError> {
        let response = reqwest::get(format!(
            "https://uptobox.com/api/link/info?fileCodes={}",
            self.file_code
        ))
        .await?
        .json::<ApiFileResponse>()
        .await?;
        match response.data.list.get(0) {
            Some(name) => Ok(name.file_name.clone()),
            None => Err(ClientError::RetrieveLink("test".to_string())),
        }
    }

    pub async fn download_file(&self, url: &str, file_name: &str) -> Result<(), ClientError> {
        let client = RequestClient::new();

        let response = client
            .get(url)
            .send()
            .await
            .or(Err(ClientError::DownloadFile(format!(
                "Failed to GET from '{}'",
                &url
            ))))?;
        let file_size = response
            .content_length()
            .ok_or(ClientError::DownloadFile(format!(
                "Failed to get content length from '{}'",
                &url
            )))?;

        // Indicatif setup
        let progress_bar = ProgressBar::new(file_size);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-"));
        progress_bar.set_message(format!("Downloading {}", file_name));

        // download chunks
        let mut file = File::create(file_name).or(Err(ClientError::DownloadFile(format!(
            "Failed to create file '{}'",
            file_name
        ))))?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(ClientError::DownloadFile(format!(
                "Error while downloading file"
            ))))?;
            file.write_all(&chunk)
                .or(Err(ClientError::DownloadFile(format!(
                    "Error while writing to file"
                ))))?;
            let new = min(downloaded + (chunk.len() as u64), file_size);
            downloaded = new;
            progress_bar.set_position(new);
        }

        progress_bar.finish_with_message(format!("Downloaded {} to {}", url, file_name));
        return Ok(());
    }
}
