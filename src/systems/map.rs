use bevy::{asset::Assets, camera::Camera2d, color::{Color, palettes::css::RED}, ecs::system::{Commands, Query, ResMut}, math::primitives::Circle, mesh::{Mesh, Mesh2d}, sprite_render::{ColorMaterial, MeshMaterial2d}, transform::components::Transform};

use crate::{models::map::{self, MapQuery}, services::map::Map, utils};


pub fn startup(mut commands:Commands, 
    mut map_service:ResMut<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query:ResMut<MapQuery>){

    //camera
    commands.spawn(Camera2d::default());

    //get map nodes
    let map_nodes = map_service.get_map_nodes(query.query.clone());
    let visual_nodes = utils::map::to_visual_nodes(map_nodes);
    for visual_node in visual_nodes{
        print!("Printing");
        print!("x:{}, y:{}", visual_node.x, visual_node.y);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(1.0))),
            Transform::from_xyz((visual_node.x * 10.0 - 200.0) as f32, (visual_node.y * 10.0 -200.0)as f32, 0.0),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ));
    }
}



pub fn draw_nodes(commands:Commands){
}


pub fn traverse_nodes(commands:Commands){
}