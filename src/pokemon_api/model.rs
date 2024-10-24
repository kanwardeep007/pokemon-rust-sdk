use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub base_experience: i32,
    pub height: i32,
    pub weight: i32,
    pub abilities: Vec<PokemonAbility>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonList {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<PokemonResource>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: i32,
    pub ability: AbilityDetails,
}

#[derive(Deserialize, Debug)]
pub struct AbilityDetails {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PokemonResource {
    pub name: String,
    pub url: String,
}
