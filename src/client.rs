use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://whiskeyfyi.com/api";

/// Async client for the WhiskeyFYI API.
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new client with the default base URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: reqwest::Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, WhiskeyFYIError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(WhiskeyFYIError::Api {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Search across all content.
    pub async fn search(&self, query: &str) -> Result<SearchResult, WhiskeyFYIError> {
        let encoded = urlencoding(query);
        self.get(&format!("/search/?q={}", encoded)).await
    }

    /// Get details for a expression by slug.
    pub async fn entity(&self, slug: &str) -> Result<EntityDetail, WhiskeyFYIError> {
        self.get(&format!("/expression/{}/", slug)).await
    }

    /// Get a glossary term by slug.
    pub async fn glossary_term(&self, slug: &str) -> Result<GlossaryTerm, WhiskeyFYIError> {
        self.get(&format!("/glossary/{}/", slug)).await
    }

    /// Get a random expression.
    pub async fn random(&self) -> Result<EntityDetail, WhiskeyFYIError> {
        self.get("/random/").await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
