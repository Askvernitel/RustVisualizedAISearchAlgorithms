use crate::{components::node::{NeighbourNode, VisualNode}, models::{map::MapCityNode, overpass::OverpassElement}};


const EARTH_RADIUS:f64 = 120.0;

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

pub fn get_x_by_longitude(radius:f64, lon:f64) -> f64{
    lon.to_radians().sin() * radius
}
pub fn get_y_by_latitude(radius:f64, lat:f64) -> f64{
    lat.to_radians().sin() * radius
}

pub fn get_euclidian_distance(v1: (f64, f64), v2: (f64, f64))->f64{
    return (v1.0-v2.0).abs().powi(2) + (v1.1-v2.1).abs().powi(2);
}

pub fn to_visual_nodes(map_city_nodes:Vec<MapCityNode>) -> Vec<VisualNode>{
    let mut visual_nodes: Vec<VisualNode> = vec![];

    for city_node in map_city_nodes{
        let x = get_x_by_longitude(EARTH_RADIUS, city_node.lon);
        let y = get_y_by_latitude(EARTH_RADIUS, city_node.lat);

        visual_nodes.push(
            VisualNode{
                x:x,
                y:y,
                city:city_node.city,
                visited:false,
                is_visiting:false,
                neighbours:vec![],
            }
        );

    }
    visual_nodes
}

pub fn connect_visual_nodes(visual_nodes:&mut Vec<VisualNode>){

    for i in 0..visual_nodes.len(){ 
        for j in 0..visual_nodes.len(){
            if i == j{
                continue;
            }

            let euclidian_distance = get_euclidian_distance((visual_nodes[i].x, visual_nodes[i].y), (visual_nodes[j].x, visual_nodes[j].y));

            visual_nodes[i].neighbours.push(
            NeighbourNode{
                index:j,
                distance:euclidian_distance,
            });
        }
    }
}
