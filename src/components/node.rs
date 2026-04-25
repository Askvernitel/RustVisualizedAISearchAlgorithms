use bevy::ecs::component::Component;


#[derive(Component)]
pub struct VisualNode{
    pub x: f64,
    pub y: f64,
    pub city: String,

    pub visited: bool,
    pub is_visiting: bool,

    pub neighbours: Vec<VisualNode>,
}






