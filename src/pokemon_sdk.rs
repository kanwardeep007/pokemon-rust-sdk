use crate::api_error::PokemonSdkError;
use crate::core_client::CoreHttpClient;
use crate::games_api::api::GamesApi;
use crate::pokemon_api::api::PokemonApi;
use crate::retry_policy_mod::RetryStrategy;
use anyhow::anyhow;
use reqwest::Url;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub struct PokemonSdk {
    pokemon: PokemonApi,
    games: GamesApi,
}

impl PokemonSdk {
    pub fn pokemon(&self) -> &PokemonApi {
        &self.pokemon
    }
    pub fn games(&self) -> &GamesApi {
        &self.games
    }
}

#[derive(Debug)]
pub struct PokemonSdkBuilder {
    http_client: reqwest::Client,
    retry_strategy: Option<RetryStrategy>,
    server_url: Url,
    timeout: Option<Duration>,
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Sandbox,
}

impl Environment {
    fn url(&self) -> Result<Url, PokemonSdkError> {
        match self {
            Environment::Production => {
                Url::from_str("https://pokeapi.co/api/v2/").map_err(|e| PokemonSdkError::from(e))
            }
            Environment::Sandbox => {
                // This URL does not exist in real world. Its a mock.
                // In real world services have sandbox environment, which would be put here
                Url::from_str("https://sandbox-pokeapi.co/api/v2/")
                    .map_err(|e| PokemonSdkError::from(e))
            }
        }
    }
}

impl PokemonSdkBuilder {
    pub fn new(
        environment: Environment,
        timeout: Option<Duration>,
    ) -> Result<PokemonSdkBuilder, PokemonSdkError> {
        let mut client_builder = reqwest::ClientBuilder::new();
        match timeout {
            Some(inner_timeout) => client_builder = client_builder.timeout(inner_timeout),
            None => {
                let duration = Duration::from_secs(3);
                client_builder = client_builder.timeout(duration);
            }
        };

        let client = client_builder
            .build()
            .map_err(|e| anyhow!("Unable to generate client. Error: {}", e.to_string()))?;

        let url = environment.url()?;

        let backoff_retry_max_seconds = Duration::from_secs(8);
        Ok(PokemonSdkBuilder {
            http_client: client,
            retry_strategy: Some(RetryStrategy::ExponentialBackoffTimed {
                max_duration: backoff_retry_max_seconds,
            }),
            server_url: url,
            timeout,
        })
    }

    pub fn with_http_client(mut self, http_client: reqwest::Client) -> PokemonSdkBuilder {
        self.http_client = http_client;
        self
    }

    pub fn with_retry_policy(mut self, retry_policy: RetryStrategy) -> Self {
        self.retry_strategy = Some(retry_policy);
        self
    }

    pub fn get_url(&self) -> &Url {
        &self.server_url
    }

    pub fn get_retry_strategy(&self) -> Option<&RetryStrategy> {
        self.retry_strategy.as_ref()
    }

    pub fn get_client(&self) -> &reqwest::Client {
        &self.http_client
    }
    pub fn get_timeout(&self) -> Option<Duration> {
        self.timeout
    }

    #[tracing::instrument]
    pub fn build(self) -> PokemonSdk {
        let mut client_with_middleware = reqwest_middleware::ClientBuilder::new(self.http_client);

        if let Some(inner_retry_policy) = self.retry_strategy {
            match inner_retry_policy {
                RetryStrategy::ExponentialBackoffTimed { max_duration } => {
                    let retry_policy =
                        ExponentialBackoff::builder().build_with_total_retry_duration(max_duration);
                    let retry_middleware = RetryTransientMiddleware::new_with_policy(retry_policy);
                    client_with_middleware = client_with_middleware.with(retry_middleware);
                }
            }
        }

        let built_client_with_middleware = client_with_middleware.build();
        let inner = Arc::new(CoreHttpClient {
            client: built_client_with_middleware,
            url: self.server_url,
        });

        let built_sdk = PokemonSdk {
            pokemon: PokemonApi::new(inner.clone()),
            games: GamesApi::new(inner),
        };
        tracing::info!("Pokemon Sdk thats built -> {:?}", &built_sdk);
        return built_sdk;
    }
}
