use reqwest::blocking::Client;
use serde::Deserialize;

use crate::utils::map::*;
use crate::models::overpass::*;
use crate::models::map::*;

struct Map{ 
    client: Client
}

impl Map{ 
    fn new(client: Client) -> Self {
        Map{client:client}
    }
    fn get_map_nodes(&mut self, query:String)->Vec<MapCityNode>{
     
        let response: OverpassResponse = self.client
            .post("https://overpass-api.de/api/interpreter")
            .body(query)
            .send()?
            .json()?;

        to_map_city_nodes(response.elements)
    }

}


