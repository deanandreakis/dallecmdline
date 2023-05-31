use reqwest::{Client, header::HeaderMap};
use serde::{Serialize, Deserialize};
use std::env;
use std::io::{self, Write, stdout};

#[derive(Debug, Serialize)]
struct ImageRequest {
    prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct ImageResponse {
    pub created: i64,
    pub data: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(default)]
    pub url: String,
}

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
       input.clear();
    }
}
