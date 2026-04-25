use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct OverpassTags {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OverpassResponse {
    pub elements: Vec<OverpassElement>,
}
#[derive(Debug, Deserialize)]
pub struct OverpassElement {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub tags: Option<OverpassTags>,
}


