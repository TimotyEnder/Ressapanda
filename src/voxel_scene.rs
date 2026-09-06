use cgmath::prelude::*;
use cgmath::{Quaternion, Vector3};
use std::collections::{BTreeMap, HashMap};

use crate::filler::voxel_and_intersect_positions_from_a_to_b;
use crate::{
    color::VoxelColor,
    voxel_instance::{RawVoxelInstance, VoxelInstance},
};

pub struct VoxelScene {
    position_to_voxel: BTreeMap<VoxelScenePosition, VoxelInstance>,
    temporary_voxels: Vec<VoxelInstance>,
    raw_voxel_instance_list: Vec<RawVoxelInstance>,
    voxels_changed: bool,
}
impl VoxelScene {
    pub fn new() -> Self {
        let voxel_map = Self::axis_grid();
        Self {
            position_to_voxel: voxel_map,
            raw_voxel_instance_list: Vec::new(),
            voxels_changed: true,
            temporary_voxels: Vec::new(),
        }
    }
    fn axis_grid() -> BTreeMap<VoxelScenePosition, VoxelInstance> {
        let mut map = BTreeMap::new();
        for position in voxel_and_intersect_positions_from_a_to_b(
            Vector3 {
                x: -16.0,
                y: 0.0,
                z: -16.0,
            },
            Vector3 {
                x: 16.0,
                y: 0.0,
                z: 16.0,
            },
        ) {
            let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
            let voxel = VoxelInstance::new_grid_voxel(
                position,
                Quaternion::from_angle_z(cgmath::Deg(0.0)),
                VoxelColor::from_hex("#333333").unwrap_or_default(),
            );
            map.insert(voxel_scene_position, voxel);
        }
        map
    }
    pub fn prepare_buffer_contents(&mut self) -> &Vec<RawVoxelInstance> {
        if self.voxels_changed {
            self.voxels_changed = false;
            self.raw_voxel_instance_list = self
                .position_to_voxel
                .values()
                .map(|voxel| voxel.to_raw())
                .chain(self.temporary_voxels.iter().map(|voxel| voxel.to_raw()))
                .collect();
        }
        self.temporary_voxels.clear();
        self.position_to_voxel
            .values_mut()
            .for_each(|voxel| voxel.unselect());
        &self.raw_voxel_instance_list
    }
    pub fn is_voxel_scene_changed(&self) -> bool {
        self.voxels_changed
    }
    pub fn force_voxel_scene_update(&mut self) {
        self.voxels_changed = true;
    }
    pub fn get_voxel_instance_count(&self) -> usize {
        self.position_to_voxel.len() + self.temporary_voxels.len()
    }
    pub fn select_voxel_at_position(&mut self, position: Vector3<f32>) {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
        if let Some(voxel) = self.position_to_voxel.get_mut(&voxel_scene_position) {
            voxel.select();
        }
        self.voxels_changed = true;
    }
    pub fn get_voxel_from_position(
        &mut self,
        position: Vector3<f32>,
    ) -> Option<&mut VoxelInstance> {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
        if let Some(voxel) = self.position_to_voxel.get_mut(&voxel_scene_position) {
            return Some(voxel);
        }
        return None;
    }
    pub fn add_temporary_voxels(&mut self, positions: Vec<Vector3<f32>>, color: &VoxelColor) {
        for pos in positions {
            let voxel_to_add = VoxelInstance::new(
                pos,
                Quaternion::from_angle_y(cgmath::Deg(0.0)),
                VoxelColor::new(color.r, color.g, color.b, color.a),
            );
            self.voxels_changed = true;
            self.temporary_voxels.push(voxel_to_add);
        }
    }
    pub fn add_voxel(&mut self, position: Vector3<f32>, color: &VoxelColor) {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);

        if !self.position_to_voxel.contains_key(&voxel_scene_position) {
            let voxel_to_add = VoxelInstance::new(
                position,
                Quaternion::from_angle_y(cgmath::Deg(0.0)),
                VoxelColor {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );
            self.voxels_changed = true;
            self.position_to_voxel
                .insert(voxel_scene_position, voxel_to_add);
        }
    }
    pub fn remove_voxel(&mut self, position: Vector3<f32>) -> bool {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
        if let Some(voxel) = self.position_to_voxel.get(&voxel_scene_position) {
            if !voxel.is_grid() {
                self.voxels_changed = true;
                self.position_to_voxel.remove(&voxel_scene_position);
            }
        }
        return false;
    }
    pub fn get_voxels(&self) -> Vec<&VoxelInstance> {
        self.position_to_voxel.values().collect()
    }
    pub fn move_by_vector(&mut self, move_vector: Vector3<f32>) {
        let move_scene_vector = VoxelScenePosition::from_voxel_position(move_vector);
        let move_overrides_grid = self
            .position_to_voxel
            .iter()
            .filter(|(_, v)| !v.is_grid())
            .any(|(pos, _)| {
                let dest = VoxelScenePosition {
                    x: pos.x + move_scene_vector.x,
                    y: pos.y + move_scene_vector.y,
                    z: pos.z + move_scene_vector.z,
                };
                self.position_to_voxel
                    .get(&dest)
                    .is_some_and(|v| v.is_grid())
            });

        if !move_overrides_grid {
            let old_keys: Vec<VoxelScenePosition> = self
                .position_to_voxel
                .iter()
                .filter_map(|(k, v)| (!v.is_grid()).then_some(*k))
                .collect();

            for voxel in self.position_to_voxel.values_mut() {
                if !voxel.is_grid() {
                    voxel.move_position_by_vector(move_vector);
                }
            }

            let voxels: Vec<VoxelInstance> = old_keys
                .iter()
                .map(|k| self.position_to_voxel.remove(k).unwrap())
                .collect();

            for (old, voxel) in old_keys.into_iter().zip(voxels) {
                self.position_to_voxel.insert(
                    VoxelScenePosition {
                        x: old.x + move_scene_vector.x,
                        y: old.y + move_scene_vector.y,
                        z: old.z + move_scene_vector.z,
                    },
                    voxel,
                );
            }
            self.voxels_changed = true;
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct VoxelScenePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl VoxelScenePosition {
    fn from_voxel_position(position: Vector3<f32>) -> Self {
        Self {
            x: position.x.round() as i32,
            y: position.y.round() as i32,
            z: position.z.round() as i32,
        }
    }
}
