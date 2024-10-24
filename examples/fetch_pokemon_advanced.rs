use pokemon_api_sdk::{pokemon_sdk, PaginationConfig, RetryStrategy};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let production_env = pokemon_sdk::Environment::Production;
    let timeout = Duration::from_secs(3);
    let max_backoff_limit = Duration::from_secs(6);
    let pokemon_sdk = pokemon_sdk::PokemonSdkBuilder::new(production_env, Some(timeout))?
        .with_retry_policy(RetryStrategy::ExponentialBackoffTimed {
            max_duration: max_backoff_limit,
        })
        .build();

    // Fetch a single pokemon
    // Use pokemon name or id in string format
    let pokemon_id = String::from("1");
    let pokemon = pokemon_sdk.pokemon().get_pokemon(pokemon_id).await;
    match pokemon {
        Ok(inner_pokemon) => {
            dbg!(inner_pokemon);
        }
        Err(e) => {
            dbg!(e);
        }
    };

    // Fetch list of pokemons
    let page_config = PaginationConfig::new(4, 2).expect("Unable to generate Pagination Config");
    let pokemons = pokemon_sdk.pokemon().list_pokemons(Some(page_config)).await;
    match pokemons {
        Ok(inner_pokemons) => {
            dbg!(inner_pokemons);
        }
        Err(e) => {
            dbg!(e);
        }
    }

    return Ok(());
}
