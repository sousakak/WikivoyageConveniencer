pub struct WikiClient {
    pub api_url: String,
}

impl WikiClient {
    pub fn ja_voyage() -> Self {
        Self {
            api_url: "https://ja.wikivoyage.org/w/api.php".to_string(),
        }
    }
}