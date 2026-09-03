use cgmath::prelude::*;
use cgmath::{Quaternion, Vector3};
use std::collections::{BTreeMap, HashMap};

use crate::{
    color::VoxelColor,
    voxel_instance::{RawVoxelInstance, VoxelInstance},
};

pub struct VoxelScene {
    position_to_voxel: BTreeMap<VoxelScenePosition, VoxelInstance>,
    raw_voxel_instance_list: Vec<RawVoxelInstance>,
    voxels_changed: bool,
}
impl VoxelScene {
    pub fn new() -> Self {
        let init_cube = VoxelInstance::new(
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            cgmath::Quaternion::from_axis_angle(Vector3::unit_z(), cgmath::Deg(0.0)),
            VoxelColor::default(),
        );
        let mut voxel_map = BTreeMap::new();
        voxel_map.insert(VoxelScenePosition { x: 0, y: 0, z: 0 }, init_cube);
        Self {
            position_to_voxel: voxel_map,
            raw_voxel_instance_list: Vec::new(),
            voxels_changed: true,
        }
    }
    pub fn prepare_buffer_contents(&mut self) -> &Vec<RawVoxelInstance> {
        if self.voxels_changed {
            self.voxels_changed = false;
            self.raw_voxel_instance_list = self
                .position_to_voxel
                .values()
                .map(|voxel| voxel.to_raw())
                .collect();
        }
        &self.raw_voxel_instance_list
    }
    pub fn is_voxel_scene_changed(&self) -> bool {
        self.voxels_changed
    }
    pub fn get_voxel_instance_count(&self) -> usize {
        self.position_to_voxel.len()
    }
    pub fn get_voxel_from_position(
        &mut self,
        position: Vector3<f32>,
    ) -> Option<&mut VoxelInstance> {
        let voxel_scene_position = VoxelScenePosition {
            x: position.x.round() as i32,
            y: position.y.round() as i32,
            z: position.z.round() as i32,
        };
        if let Some(voxel) = self.position_to_voxel.get_mut(&voxel_scene_position) {
            return Some(voxel);
        }
        return None;
    }
    pub fn add_voxel(&mut self, position: Vector3<f32>, color: VoxelColor) {
        let voxel_scene_position = VoxelScenePosition {
            x: position.x.round() as i32,
            y: position.y.round() as i32,
            z: position.z.round() as i32,
        };

        let voxel_to_add =
            VoxelInstance::new(position, Quaternion::from_angle_y(cgmath::Deg(0.0)), color);
        self.voxels_changed = true;
        self.position_to_voxel
            .insert(voxel_scene_position, voxel_to_add);
    }
    pub fn remove_voxel(&mut self, position: Vector3<f32>) -> bool {
        let voxel_scene_position = VoxelScenePosition {
            x: position.x.round() as i32,
            y: position.y.round() as i32,
            z: position.z.round() as i32,
        };
        if self.position_to_voxel.contains_key(&voxel_scene_position) {
            self.voxels_changed = true;
            self.position_to_voxel.remove(&voxel_scene_position);
        }
        return false;
    }
    pub fn get_voxels(&self) -> Vec<&VoxelInstance> {
        self.position_to_voxel.values().collect()
    }
}
#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct VoxelScenePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
