use cgmath::{Point3, Vector3};

use crate::{
    tools::{add::Add, del::Del},
    voxel_instance::VoxelInstance,
    voxel_scene::VoxelScene,
};

pub trait Tool {
    fn operate_with_voxel_and_intersect(
        &mut self,
        voxel_positon: Vector3<f32>,
        intersect_pos: Point3<f32>,
        scene: &mut VoxelScene,
    );
}

pub fn tool_from_name(name: &'static str) -> Option<Box<dyn Tool>> {
    match name {
        "Add" => return Some(Box::new(Add {})),
        "Del" => return Some(Box::new(Del {})),
        _ => return None,
    };
}
