use cgmath::prelude::*;
use cgmath::{Quaternion, Vector3};
use std::collections::{BTreeMap, HashMap};

use crate::{
    color::VoxelColor,
    voxel_instance::{RawVoxelInstance, VoxelInstance},
};

pub struct VoxelScene {
    position_to_voxel_instance_index: BTreeMap<VoxelScenePosition, isize>,
    voxel_instance_list: Vec<VoxelInstance>,
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
            VoxelColor::new(1.0, 0.0, 0.0, 1.0),
        );
        let voxels = vec![init_cube];
        let mut voxel_map = BTreeMap::new();
        voxel_map.insert(VoxelScenePosition { x: 0, y: 0, z: 0 }, 0);
        Self {
            position_to_voxel_instance_index: voxel_map,
            voxel_instance_list: voxels,
            raw_voxel_instance_list: Vec::new(),
            voxels_changed: true,
        }
    }
    pub fn prepare_buffer_contents(&mut self) -> &Vec<RawVoxelInstance> {
        if self.voxels_changed {
            self.voxels_changed = false;
            self.raw_voxel_instance_list = self
                .voxel_instance_list
                .iter()
                .map(|voxel| voxel.to_raw())
                .collect();
        }
        &self.raw_voxel_instance_list
    }
    pub fn is_voxel_scene_changed(&self) -> bool {
        self.voxels_changed
    }
    pub fn get_voxel_instance_count(&self) -> usize {
        self.voxel_instance_list.len()
    }
    pub fn add_voxel(&mut self, position: Vector3<f32>, color: VoxelColor) -> bool {
        let voxel_scene_position = VoxelScenePosition {
            x: position.x as i32,
            y: position.y as i32,
            z: position.z as i32,
        };
        /*if !self
            .position_to_voxel_instance_index
            .contains_key(&voxel_scene_position)
        {*/
        let voxel_to_add =
            VoxelInstance::new(position, Quaternion::from_angle_y(cgmath::Deg(0.0)), color);
        self.voxels_changed = true;
        self.voxel_instance_list.push(voxel_to_add);
        self.position_to_voxel_instance_index.insert(
            voxel_scene_position,
            self.voxel_instance_list.len() as isize - 1,
        );
        return true;
        /* }*/
        //return false;
    }
    pub fn remove_voxel(&mut self, position: Vector3<f32>) -> bool {
        let voxel_scene_position = VoxelScenePosition {
            x: position.x as i32,
            y: position.y as i32,
            z: position.z as i32,
        };
        if self
            .position_to_voxel_instance_index
            .contains_key(&voxel_scene_position)
        {
            self.voxels_changed = true;
            if let Some(pos) = self
                .position_to_voxel_instance_index
                .get(&voxel_scene_position)
            {
                self.voxel_instance_list.remove((*pos) as usize);
                self.position_to_voxel_instance_index
                    .remove(&voxel_scene_position);
                return true;
            }
            return false;
        }
        return false;
    }
    pub fn get_voxels(&self) -> &Vec<VoxelInstance> {
        &self.voxel_instance_list
    }
}
#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct VoxelScenePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
