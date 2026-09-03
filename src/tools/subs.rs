use crate::{color::VoxelColor, tools::tool::Tool};

pub struct Subs {}
impl Tool for Subs {
    fn operate_with_voxel_and_intersect(
        &mut self,
        voxel_positon: cgmath::Vector3<f32>,
        intersect_pos: cgmath::Point3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
    ) {
        scene.remove_voxel(voxel_positon);
        scene.add_voxel(voxel_positon, VoxelColor::default());
    }

    fn name(&self) -> &'static str {
        "Subs"
    }
}
