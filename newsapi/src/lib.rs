use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewsApiError {
    #[error("Request to fetch articles failed.")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseFailed(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApiError::FailedResponseToString(e))?;
    let articles: Articles =
        serde_json::from_str(&response).map_err(|e| NewsApiError::ArticleParseFailed(e))?;
    Ok(articles)
}
