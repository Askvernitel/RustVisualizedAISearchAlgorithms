use bevy::ecs::component::Component;


pub struct NeighbourNode{
    pub visual_node:VisualNode,
    pub cost:i32,
}

#[derive(Component)]
pub struct VisualNode{
    pub x: f64,
    pub y: f64,
    pub city: String,

    pub visited: bool,
    pub is_visiting: bool,

    pub neighbours: Vec<VisualNode>,
}







