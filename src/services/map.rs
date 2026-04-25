use reqwest::blocking::Client;
use serde::Deserialize;

use crate::utils::map::*;
use crate::models::overpass::*;
use crate::models::map::*;

pub struct Map{ 
    client: Client
}

impl Map{ 
    pub fn new(client: Client) -> Self {
        Map{client:client}
    }
    pub fn get_map_nodes(&mut self, query:String)->Vec<MapCityNode>{
     
        let response:OverpassResponse = self.client
            .post("https://overpass-api.de/api/interpreter")
            .body(query).send().unwrap().json().unwrap();
        
        to_map_city_nodes(response.elements)
    }

}


