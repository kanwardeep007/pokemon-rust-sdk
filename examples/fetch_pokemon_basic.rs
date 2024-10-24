use pokemon_api_sdk::{pokemon_sdk, PaginationConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let production_env = pokemon_sdk::Environment::Production;
    let pokemon_sdk = pokemon_sdk::PokemonSdkBuilder::new(production_env, None)?.build();

    // Fetch single pokemon
    // Use pokemon name or id in string format
    let pokemon_id = String::from("1");
    let pokemon = pokemon_sdk.pokemon().get_pokemon(pokemon_id).await;
    match pokemon {
        Ok(inner_pokemon) => {
            dbg!(inner_pokemon);
        }
        Err(ref e) => {
            dbg!(e);
        }
    };

    // Fetch list of pokemons
    let pokemons = pokemon_sdk
        .pokemon()
        .list_pokemons(Some(PaginationConfig::get_default()))
        .await;
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
