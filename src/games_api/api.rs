use crate::core_client::{get_query_string, CoreHttpClient};
use crate::games_api::model::{Generation, GenerationList};
use crate::{api_error, PaginationConfig};
use api_error::PokemonSdkError;
use std::sync::Arc;

#[derive(Debug)]
pub struct GamesApi {
    inner: Arc<CoreHttpClient>,
}

impl GamesApi {
    pub(crate) fn new(inner: Arc<CoreHttpClient>) -> Self {
        Self { inner }
    }

    #[tracing::instrument]
    pub async fn get_generation(&self, identifier: String) -> Result<Generation, PokemonSdkError> {
        let full_url = self.inner.url.join(&format!("generation/{}", identifier))?;

        let server_response = self.inner.client.get(full_url).send().await?;
        if !server_response.status().is_success() {
            let status_code = server_response.status();
            let response_text = server_response.text().await.unwrap_or_else(|e| {
                format!("Unable to get the response text. Error: {}", e.to_string())
            });
            return Err(PokemonSdkError::FailureResponse {
                status_code,
                response_text,
            });
        };

        let response: Generation = server_response.json().await?;
        tracing::info!("Successfully Fetched {} generation", response.name);

        return Ok(response);
    }

    #[tracing::instrument]
    pub async fn list_generations(
        &self,
        pagination_config: Option<PaginationConfig>,
    ) -> Result<GenerationList, PokemonSdkError> {
        let page_config = match pagination_config {
            Some(inner_config) => inner_config,
            None => PaginationConfig::get_default(),
        };

        let query_params = get_query_string(page_config.get_offset(), page_config.get_page_size());
        let mut full_url = self.inner.url.join(&format!("generation/"))?;
        full_url.set_query(Some(&query_params));

        let server_response = self.inner.client.get(full_url).send().await?;
        if !server_response.status().is_success() {
            let status_code = server_response.status();
            let response_text = server_response.text().await.unwrap_or_else(|e| {
                format!("Unable to get the response text. Error: {}", e.to_string())
            });
            return Err(PokemonSdkError::FailureResponse {
                status_code,
                response_text,
            });
        };

        let response: GenerationList = server_response.json().await?;

        return Ok(response);
    }
}
