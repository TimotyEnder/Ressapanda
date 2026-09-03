use crate::{color::VoxelColor, tools::tool::Tool};

pub struct Subs {}
impl Tool for Subs {
    fn operate_with_voxel_and_intersect(
        &mut self,
        operating_position: cgmath::Vector3<f32>,
        scene: &mut crate::voxel_scene::VoxelScene,
    ) {
        scene.remove_voxel(operating_position);
        scene.add_voxel(operating_position, VoxelColor::default());
    }

    fn name(&self) -> &'static str {
        "Subs"
    }
}
