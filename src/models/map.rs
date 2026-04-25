use bevy::ecs::resource::Resource;

#[derive(Debug)]
pub struct MapCityNode{
    pub lat:f64,
    pub lon:f64,
    pub city:String
}


#[derive(Resource)]
pub struct MapQuery{
    pub query:String,
}
