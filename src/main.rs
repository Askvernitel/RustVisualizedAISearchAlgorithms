
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::services::map::Map;

pub mod services;
pub mod utils;
pub mod models;
pub mod components;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let query = r#"
        [out:json];
        area["ISO3166-1"="GE"]->.a;
        node["place"="city"]["name:en"](area.a);
        out;
        "#;

    let client =Client::new();
    let mut map_service = Map::new(client);
    print!("{:?}", map_service.get_map_nodes(String::from(query)));
    
    Ok(())


}