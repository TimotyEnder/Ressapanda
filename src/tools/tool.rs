use cgmath::{Point3, Vector3};

use crate::{
    brushes::brush::Brush,
    tools::{add::Add, del::Del, subs::Subs},
    voxel_scene::VoxelScene,
};

pub trait Tool {
    fn operate_with_voxel_and_intersect(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        brush: &Brush,
    );
    fn temp_operate_with_voxel_and_intersect(
        &mut self,
        operating_position: Vector3<f32>,
        scene: &mut VoxelScene,
        brush: &Brush,
    );
    fn name(&self) -> &'static str;
    fn cursor_name(&self) -> &'static str;
}

pub fn tool_from_name(name: &'static str) -> Option<Box<dyn Tool>> {
    match name {
        "Add" => return Some(Box::new(Add {})),
        "Del" => return Some(Box::new(Del {})),
        "Subs" => return Some(Box::new(Subs {})),
        _ => return None,
    };
}
