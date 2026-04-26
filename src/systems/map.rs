use bevy::{asset::Assets, camera::Camera2d, color::{Color, palettes::css::RED}, ecs::system::{Commands, Query, ResMut}, math::primitives::Circle, mesh::{Mesh, Mesh2d}, sprite_render::{ColorMaterial, MeshMaterial2d}, transform::components::Transform};

use crate::{models::map::{self, MapQuery}, services::map::Map, utils};


const SCALE:f64 = 150.0;

pub fn startup(mut commands:Commands, 
    mut map_service:ResMut<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query:ResMut<MapQuery>){

    //camera
    commands.spawn(Camera2d::default());

    //show map nodes
    let map_nodes = map_service.get_map_nodes(query.query.clone());
    let visual_nodes = utils::map::to_visual_nodes(map_nodes);
    for visual_node in visual_nodes{
        print!("x:{:?}, y:{:?} \n", visual_node.x, visual_node.y);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            Transform::from_xyz((SCALE*(visual_node.x  - 82.0)) as f32, (SCALE*(visual_node.y - 80.5)) as f32, 0.0),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ));
    }
    draw_nodes();
}


pub fn draw_nodes(&mut map_service:ResMut<Map>){

}
pub fn draw_lines(){

}

pub fn traverse_nodes(commands:Commands){
}