use bevy::platform::collections::HashMap;
use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct OverpassResponse {
    pub elements: Vec<OverpassElement>,
}
#[derive(Debug, Deserialize)]
pub struct OverpassElement {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub tags: Option<HashMap<String,String>>,
}


