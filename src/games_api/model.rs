use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Generation {
    pub id: i32,
    pub name: String,
    pub abilities: Vec<PokemonAbility>,
    pub main_region: RegionResource,
    pub moves: Vec<MoveResource>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationList {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<GenerationResource>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationResource {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct RegionResource {
    pub name: String,
    pub url: String,
}
#[derive(Deserialize, Debug)]
pub struct MoveResource {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: i32,
    pub ability: String,
}
