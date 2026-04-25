use bevy::ecs::component::Component;


#[derive(Component)]
pub struct VisualNode{
    pub x:f64,
    pub y:f64,
    pub visited: bool,
    pub is_visiting: bool,

    pub node:Node,
}

pub struct Node{
    pub id: String,
    pub next: Box<Node>,
}




