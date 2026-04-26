use bevy::ecs::component::Component;

#[derive(Clone)]
pub struct NeighbourNode{
    pub index:usize,
    pub distance:f64,
}

#[derive(Component, Clone)]
pub struct VisualNode{
    pub x: f64,
    pub y: f64,
    pub city: String,

    pub visited: bool,
    pub is_visiting: bool,

    pub neighbours: Vec<NeighbourNode>,
}







