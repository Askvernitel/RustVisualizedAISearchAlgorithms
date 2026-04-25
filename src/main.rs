
use bevy::{DefaultPlugins, app::{App, Startup}};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{models::map::MapQuery, services::map::Map, systems::map, utils::map::to_visual_nodes};

pub mod services;
pub mod utils;
pub mod models;
pub mod components;
pub mod systems;



fn main() -> Result<(), Box<dyn std::error::Error>> {
   let query = String::from(r#"
        [out:json];
        area["ISO3166-1"="GE"]->.a;
        node["place"="city"]["name:en"](area.a);
        out;
        "#);

    let client =Client::new();
    let mut map_service = Map::new(client);
    //let map_nodes = map_service.get_map_nodes(String::from(query));

    //let visual_nodes = to_visual_nodes(map_nodes);

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(MapQuery{query})
    .insert_resource(map_service)
    .add_systems(Startup, map::startup)
    .run();
    
    Ok(())

}