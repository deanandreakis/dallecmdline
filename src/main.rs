use reqwest::{Client, header::HeaderMap};
use std::env;
use std::io::{self, stdout, Write};
use crate::types::types::{ImageResponse, ImageRequest};
pub mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");
    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap());

    headers.insert(reqwest::header::CONTENT_TYPE,
        format!("application/json").parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let mut input = String::new();

    loop {
        print!("Request an Image from Dall-E: (Ctrl-C to exit) ");
        stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let image_request = ImageRequest {
            prompt: input.trim().to_string(),
        };

        let response:ImageResponse = client.post("https://api.openai.com/v1/images/generations")
            .json(&image_request)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", response);
        open::that(&response.data.first().unwrap().url)?;
       input.clear();
    }
}
