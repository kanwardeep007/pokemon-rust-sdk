use std::time::Duration;

use pokemon_api_sdk::{
    pokemon_sdk::{Environment, PokemonSdkBuilder},
    PaginationConfig,
};

#[test_log::test(tokio::test)]
async fn get_pokemon_detail_fetches_details_of_pokemon() {
    // Arrange
    let id = 1;
    let new_pokemon_sdk =
        PokemonSdkBuilder::new(Environment::Production, Some(Duration::from_secs(3)))
            .expect("Unable to build Pokemon SDK")
            .build();

    // Act
    let pokemon_details = new_pokemon_sdk.pokemon().get_pokemon(id.to_string()).await;

    // Assert
    assert!(pokemon_details.is_ok());
    let pokemon = pokemon_details.expect("Expected to find a pokemon");
    assert_eq!(pokemon.id, 1);
    assert_eq!(pokemon.name, "bulbasaur");
    assert_eq!(pokemon.base_experience, 64);
    assert_eq!(pokemon.height, 7);
    assert_eq!(pokemon.weight, 69);
}

#[test_log::test(tokio::test)]
async fn get_generation_detail_fetches_details_of_a_specific_pokemon_generation() {
    // Arrange
    let id = 1;
    let new_pokemon_sdk =
        PokemonSdkBuilder::new(Environment::Production, Some(Duration::from_secs(3)))
            .expect("Unable to build Pokemon SDK")
            .build();

    // Act
    let generation_details = new_pokemon_sdk.games().get_generation(id.to_string()).await;

    // Assert
    assert!(generation_details.is_ok());
    assert_eq!(generation_details.unwrap().id, 1);
}

#[test_log::test(tokio::test)]
async fn get_pokemons_returns_a_list_of_pokemons() {
    // Arrange
    let new_pokemon_sdk =
        PokemonSdkBuilder::new(Environment::Production, Some(Duration::from_secs(3)))
            .expect("Unable to build Pokemon SDK")
            .build();

    // Act
    let list_pokemon_response = new_pokemon_sdk.pokemon().list_pokemons(None).await;

    // Assert
    assert!(list_pokemon_response.is_ok());
    let pokemons = list_pokemon_response.expect("Failed to fetch pokemon list");

    #[allow(unused_comparisons)]
    let x = pokemons.count >= 0;
    assert!(x);
}

#[test_log::test(tokio::test)]
async fn get_pokemons_using_pagination_config_returns_requested_number_of_pokemons() {
    // Arrange
    let new_pokemon_sdk =
        PokemonSdkBuilder::new(Environment::Production, Some(Duration::from_secs(3)))
            .expect("Unable to build Pokemon SDK")
            .build();

    let number_of_pokemons = 10;
    let page_config = PaginationConfig::get_default()
        .with_page_size(number_of_pokemons)
        .expect("Unable to generate pagination config");

    // Act
    let list_pokemon_response = new_pokemon_sdk
        .pokemon()
        .list_pokemons(Some(page_config))
        .await;

    // Assert
    assert!(list_pokemon_response.is_ok());
    let pokemons = list_pokemon_response.expect("Failed to fetch pokemon list");

    assert_eq!(pokemons.results.len() as u32, number_of_pokemons)
}

#[test_log::test(tokio::test)]
async fn get_generations_returns_a_list_of_generations() {
    // Arrange
    let new_pokemon_sdk =
        PokemonSdkBuilder::new(Environment::Production, Some(Duration::from_secs(3)))
            .expect("Unable to build Pokemon SDK")
            .build();

    // Act
    let list_generations_response = new_pokemon_sdk.games().list_generations(None).await;

    // Assert
    assert!(list_generations_response.is_ok());
    let generations = list_generations_response.expect("Failed to fetch Generation list");

    #[allow(unused_comparisons)]
    let x = generations.count >= 0;
    assert!(x);
}
#[test_log::test(tokio::test)]
async fn get_generations_returns_the_number_of_requested_generations() {
    // Arrange
    let new_pokemon_sdk =
        PokemonSdkBuilder::new(Environment::Production, Some(Duration::from_secs(3)))
            .expect("Unable to build Pokemon SDK")
            .build();

    let number_of_generations = 2;
    let offset = 2;

    let page_config = PaginationConfig::new(number_of_generations, offset)
        .expect("Failed to generate page config");
    // Act
    let list_generations_response = new_pokemon_sdk
        .games()
        .list_generations(Some(page_config))
        .await;

    // Assert
    assert!(list_generations_response.is_ok());
    let generations = list_generations_response.expect("Failed to fetch Generation list");

    assert_eq!(generations.results.len() as u32, number_of_generations)
}
