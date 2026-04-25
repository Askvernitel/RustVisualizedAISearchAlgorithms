use reqwest::blocking::Client;
use serde::Deserialize;

pub mod services;
pub mod utils;
pub mod models;
fn main() -> Result<(), Box<dyn std::error::Error>> {
   let query = r#"
        [out:json];
        area["name"="Georgia"]->.a;
        node["place"="city"](area.a);
        out;
        "#;
    Ok(())
}