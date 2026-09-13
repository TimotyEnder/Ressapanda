use cgmath::{Deg, Quaternion, Vector3};
use cgmath::{prelude::*, vec3};
use std::collections::BTreeMap;

use crate::camera::Camera;
use crate::conversion_utils::snap_vector_to_flat_direction;
use crate::filler::fill_positions_from_a_to_b;
use crate::{
    color::VoxelColor,
    voxel_instance::{RawVoxelInstance, VoxelInstance},
};

pub struct VoxelScene {
    position_to_voxel: BTreeMap<VoxelScenePosition, VoxelInstance>,
    temporary_voxels: Vec<VoxelInstance>,
    raw_voxel_instance_list: Vec<RawVoxelInstance>,
    voxels_changed: bool,
    center: Vector3<f32>,
}
impl VoxelScene {
    pub fn new() -> Self {
        let voxel_map = Self::axis_grid();
        Self {
            position_to_voxel: voxel_map,
            raw_voxel_instance_list: Vec::new(),
            voxels_changed: true,
            temporary_voxels: Vec::new(),
            center: vec3(0.0, 0.0, 0.0),
        }
    }
    fn axis_grid() -> BTreeMap<VoxelScenePosition, VoxelInstance> {
        let mut map = BTreeMap::new();
        for position in fill_positions_from_a_to_b(
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
            self.center = self.find_center();
        }
    }
    pub fn remove_voxel(&mut self, position: Vector3<f32>) -> bool {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
        if let Some(voxel) = self.position_to_voxel.get(&voxel_scene_position) {
            if !voxel.is_grid() {
                self.voxels_changed = true;
                self.position_to_voxel.remove(&voxel_scene_position);
                self.center = self.find_center()
            }
        }
        return false;
    }
    pub fn get_voxels(&self) -> Vec<&VoxelInstance> {
        self.position_to_voxel.values().collect()
    }
    fn find_center(&self) -> Vector3<f32> {
        let mut sum = vec3(0.0, 0.0, 0.0);
        let mut n = 0.0;
        for voxel in self.position_to_voxel.values() {
            if !voxel.is_grid() {
                sum += voxel.get_position();
                n += 1.0;
            }
        }
        if n == 0.0 {
            return vec3(0.0, 0.0, 0.0);
        }
        vec3(
            (sum.x / n).round(),
            (sum.y / n).round(),
            (sum.z / n).round(),
        )
    }
    pub fn reposition_to_calculated_center(&mut self) {
        let center = self.find_center();
        self.move_by_vector(vec3(-center.x, 0.0, -center.z));
        while self.move_by_vector(vec3(0.0, -1.0, 0.0)) {}
    }
    pub fn rotate_around_center(&mut self, axis: Vector3<f32>, deg: cgmath::Deg<f32>) -> bool {
        let center = self.center;
        let rotation = Quaternion::from_axis_angle(axis.normalize(), deg);
        let old_keys: Vec<VoxelScenePosition> = self
            .position_to_voxel
            .iter()
            .filter_map(|(k, v)| (!v.is_grid()).then_some(*k))
            .collect();
        let mut smallest_y = f32::INFINITY;
        for voxel in self.position_to_voxel.values_mut() {
            if !voxel.is_grid() {
                let computed_float_pos = center + rotation * (voxel.get_position() - center);
                smallest_y = smallest_y.min(computed_float_pos.y.round());
                voxel.set_position(vec3(
                    computed_float_pos.x.round(),
                    computed_float_pos.y.round(),
                    computed_float_pos.z.round(),
                ));
            }
        }
        let necessary_lift_overlap_prevention = (1.0 - smallest_y).max(0.0);
        if smallest_y <= 0.0 {
            for voxel in self.position_to_voxel.values_mut() {
                if !voxel.is_grid() {
                    voxel.move_position_by_vector(vec3(
                        0.0,
                        necessary_lift_overlap_prevention,
                        0.0,
                    ));
                }
            }
        }
        let voxels: Vec<VoxelInstance> = old_keys
            .iter()
            .map(|k| self.position_to_voxel.remove(k).unwrap())
            .collect();

        for (_, voxel) in old_keys.into_iter().zip(voxels) {
            self.position_to_voxel.insert(
                VoxelScenePosition::from_voxel_position(voxel.get_position()),
                voxel,
            );
        }
        self.voxels_changed = true;
        return true;
    }
    pub fn move_by_vector(&mut self, move_vector: Vector3<f32>) -> bool {
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
            self.center = self.find_center();
            self.voxels_changed = true;
            return true;
        }
        return false;
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
#[derive(PartialEq, Eq)]
pub enum VoxelSceneDirection {
    UpLeftSteer,
    DownRightSteer,
    Left,
    Right,
    Forwards,
    Backwards,
    None,
}
impl VoxelSceneDirection {
    pub fn move_vector(&self, camera: &Camera) -> Vector3<f32> {
        match self {
            VoxelSceneDirection::UpLeftSteer => Vector3::new(0.0, 1.0, 0.0),
            VoxelSceneDirection::DownRightSteer => Vector3::new(0.0, -1.0, 0.0),
            VoxelSceneDirection::Left => snap_vector_to_flat_direction(camera.right()) * -1.0,
            VoxelSceneDirection::Right => snap_vector_to_flat_direction(camera.right()),
            VoxelSceneDirection::Forwards => snap_vector_to_flat_direction(camera.forward()),
            VoxelSceneDirection::Backwards => {
                snap_vector_to_flat_direction(camera.forward()) * -1.0
            }
            _ => vec3(0.0, 0.0, 0.0),
        }
    }
    pub fn rotate_parameters(&self, camera: &Camera) -> (Vector3<f32>, cgmath::Deg<f32>) {
        match self {
            VoxelSceneDirection::DownRightSteer => (
                snap_vector_to_flat_direction(camera.forward()),
                cgmath::Deg(90.0),
            ),
            VoxelSceneDirection::UpLeftSteer => (
                snap_vector_to_flat_direction(camera.forward()),
                cgmath::Deg(-90.0),
            ),
            VoxelSceneDirection::Right => (Vector3::new(0.0, 1.0, 0.0), cgmath::Deg(90.0)),
            VoxelSceneDirection::Left => (Vector3::new(0.0, 1.0, 0.0), cgmath::Deg(-90.0)),
            VoxelSceneDirection::Forwards => (
                snap_vector_to_flat_direction(camera.right()),
                cgmath::Deg(90.0),
            ),
            VoxelSceneDirection::Backwards => (
                snap_vector_to_flat_direction(camera.right()),
                cgmath::Deg(-90.0),
            ),
            _ => (Vector3::new(0.0, 1.0, 0.0), Deg(0.0)),
        }
    }
}
