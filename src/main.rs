mod api;
mod error;
mod uptobox;

use std::io::Write;

use error::ClientError;
use uptobox::Client;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Uptobox api token
    #[clap(short = 't', long)]
    token: String,

    /// Uptobox links
    #[clap(short = 'l', long)]
    links: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    for link in args.links.split(" ") {
        writeln!(std::io::stdout(), " -> Processing: {}", link).expect("Unable to write to stdout");
        let result = process_link(link, &args.token).await;
        if let Err(err) = result {
            writeln!(std::io::stderr(), "Error: {:?}", err).expect("Unable to write to stdout");
        };
    }
}

async fn process_link(link: &str, user_token: &str) -> Result<(), ClientError> {
    let file_code = get_file_code(&link)?;
    let client = Client::new(&file_code, &user_token);
    let download_link = client.get_download_link().await?;
    let file_name = client.get_file_name().await?;
    client.download_file(&download_link, &file_name).await?;
    Ok(())
}

fn get_file_code(link: &str) -> Result<&str, ClientError> {
    let capture = match Regex::new(r"(uptobox|uptostream).+(?P<file_code>[a-zA-Z0-9]{12})")
        .unwrap()
        .captures(link)
    {
        Some(c) => c,
        None => {
            return Err(ClientError::BadLink(
                "Please provide a valid uptobox link".to_string(),
            ))
        }
    };
    let file_code = match capture.name("file_code") {
        Some(g) => g,
        None => {
            return Err(ClientError::BadLink(
                "Please provide a valid uptobox link".to_string(),
            ))
        }
    };

    Ok(&link[file_code.start()..file_code.end()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uptobox_get_file_code() {
        let file_code = get_file_code("https://uptobox.com/m5f0ce9h197j").unwrap();
        assert_eq!(file_code, "m5f0ce9h197j");
    }

    #[test]
    fn uptostream_get_file_code() {
        let file_code = get_file_code("https://uptostream.com/m5f0ce9h197j").unwrap();
        assert_eq!(file_code, "m5f0ce9h197j");
    }
}
