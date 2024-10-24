mod api_error;
mod core_client;
mod games_api;
mod pokemon_api;
mod retry_policy_mod;

pub mod pokemon_sdk;
pub use api_error::PokemonSdkError;
pub use retry_policy_mod::RetryStrategy;

use anyhow::anyhow;
const DEFAULT_PAGE_SIZE: u32 = 50;
const MAX_ALLOWED_PAGE_SIZE: u32 = 500;
const MIN_ALLOWED_PAGE_SIZE: u32 = 1;
const DEFAULT_OFFSET: u32 = 50;

#[derive(Debug)]
pub struct PaginationConfig {
    page_size: u32,
    offset: u32,
}

impl PaginationConfig {
    pub fn new(page_size: u32, offset: u32) -> Result<Self, PokemonSdkError> {
        validate_page_size(page_size)?;
        Ok(PaginationConfig { page_size, offset })
    }
    pub fn get_default() -> Self {
        PaginationConfig {
            page_size: DEFAULT_PAGE_SIZE,
            offset: DEFAULT_OFFSET,
        }
    }

    pub fn with_page_size(mut self, page_size: u32) -> Result<Self, PokemonSdkError> {
        validate_page_size(page_size)?;
        self.page_size = page_size;
        Ok(self)
    }
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    pub fn get_page_size(&self) -> u32 {
        self.page_size
    }
    pub fn get_offset(&self) -> u32 {
        self.offset
    }
}
fn validate_page_size(page_size: u32) -> Result<(), PokemonSdkError> {
    if page_size > MAX_ALLOWED_PAGE_SIZE || page_size < MIN_ALLOWED_PAGE_SIZE {
        return Err(PokemonSdkError::Other(anyhow!(
            "Page size should be between 0 and 501"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_size_cannot_be_0_or_less() {
        let invalid_page_size = 0;
        let page_config = validate_page_size(invalid_page_size);
        assert!(page_config.is_err());
    }
    #[test]
    fn page_size_cannot_be_more_than() {
        let invalid_page_size = 5002;
        let page_config = validate_page_size(invalid_page_size);
        assert!(page_config.is_err());
    }
}
