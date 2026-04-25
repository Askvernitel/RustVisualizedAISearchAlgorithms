use crate::models::{map::MapCityNode, overpass::OverpassElement};


pub fn to_map_city_nodes(raw_nodes:Vec<OverpassElement>) -> Vec<MapCityNode>{
    let mut map_city_nodes = vec![];
    for raw_node in raw_nodes{ 
        if let (Some(tags), Some(lat), Some(lon)) = (raw_node.tags, raw_node.lat, raw_node.lon) {
            if let Some(name) = tags.get("name:en") {
                map_city_nodes.push(
                    MapCityNode{
                        lat:lat,
                        lon:lon,
                        city:name.clone(),
                    }
                );
            }
        }
    }

    map_city_nodes
}