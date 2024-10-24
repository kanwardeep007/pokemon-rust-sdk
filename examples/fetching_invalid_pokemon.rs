use pokemon_api_sdk::pokemon_sdk;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let production_env = pokemon_sdk::Environment::Production;
    let pokemon_sdk = pokemon_sdk::PokemonSdkBuilder::new(production_env, None)?.build();

    // Use pokemon name or id in string format
    let pokemon_id = String::from("invalid");
    let pokemon = pokemon_sdk.pokemon().get_pokemon(pokemon_id).await;
    match pokemon {
        Ok(inner_pokemon) => {
            dbg!(inner_pokemon);
        }
        Err(ref e) => {
            dbg!(e);
            // Error handling code goes here
        }
    };
    return Ok(());
}
