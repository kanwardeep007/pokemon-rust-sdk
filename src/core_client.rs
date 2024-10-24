use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;

#[derive(Debug)]
pub struct CoreHttpClient {
    pub client: ClientWithMiddleware,
    pub url: Url,
}

pub fn get_query_string(offset: u32, page_size: u32) -> String {
    format!(
        "limit={}&offset={}",
        page_size.to_string(),
        offset.to_string()
    )
}
