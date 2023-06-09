pub mod types {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize)]
    pub struct ImageRequest {
        pub prompt: String,
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
}
