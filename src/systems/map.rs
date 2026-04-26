use bevy::{asset::Assets, camera::Camera2d, color::{Color, palettes::css::RED}, ecs::system::{Commands, Query, ResMut}, math::primitives::Circle, mesh::{Mesh, Mesh2d}, sprite_render::{ColorMaterial, MeshMaterial2d}, transform::components::Transform};

use crate::{components::node::VisualNode, models::map::{self, MapQuery}, services::map::Map, utils::{self, map::connect_visual_nodes}};


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
    let mut visual_nodes = utils::map::to_visual_nodes(map_nodes);
    let mesh = Mesh2d(meshes.add(Circle::new(10.0)));
    let mesh_material = MeshMaterial2d(materials.add(ColorMaterial::from_color(RED)));

    
    connect_visual_nodes(&mut visual_nodes);

    draw_visual_nodes(&mut commands, &visual_nodes, (mesh, mesh_material));

    
}


pub fn draw_visual_nodes(commands:&mut Commands, visual_nodes:&Vec<VisualNode>, mesh:(Mesh2d, MeshMaterial2d<ColorMaterial>)){
    for visual_node in visual_nodes{
        commands.spawn((
            mesh.0.clone(),
            mesh.1.clone(),
            Transform::from_xyz((SCALE*(visual_node.x  - 82.0)) as f32, (SCALE*(visual_node.y - 80.5)) as f32, 0.0),
        ));
    }
}
pub fn draw_lines(){

}

pub fn traverse_nodes(commands:Commands){
}