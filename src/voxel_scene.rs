use cgmath::{Deg, Quaternion, Vector3};
use cgmath::{prelude::*, vec3};
use std::collections::BTreeMap;
use std::usize;

use crate::camera::Camera;
use crate::conversion_utils::snap_vector_to_flat_direction;
use crate::filler::fill_positions_from_a_to_b;
use crate::save::{SaveFile, SavedVoxelGroup};
use crate::{
    color::VoxelColor,
    voxel_instance::{RawVoxelInstance, VoxelInstance},
};
pub struct GridVoxelDimensions {
    pub width: f32,  //x coord size
    pub length: f32, //z coord size
}
#[derive(Clone)]
struct VoxelGroup {
    pub position_to_voxel: BTreeMap<VoxelScenePosition, VoxelInstance>,
    pub name: String,
    pub visible: bool,
    pub editing_name: bool,
    pub center: Vector3<f32>,
}
impl VoxelGroup {
    pub fn orientation_cross() -> Self {
        let voxels = vec![
            VoxelInstance::new(vec3(2.0, 0.0, 0.0), VoxelColor::new(1.0, 0.0, 0.0, 1.0)),
            VoxelInstance::new(vec3(3.0, 0.0, 0.0), VoxelColor::new(1.0, 0.0, 0.0, 1.0)),
            VoxelInstance::new(vec3(4.0, 0.0, 0.0), VoxelColor::new(1.0, 0.0, 0.0, 1.0)),
            VoxelInstance::new(vec3(0.0, 2.0, 0.0), VoxelColor::new(0.0, 1.0, 0.0, 1.0)),
            VoxelInstance::new(vec3(0.0, 3.0, 0.0), VoxelColor::new(0.0, 1.0, 0.0, 1.0)),
            VoxelInstance::new(vec3(0.0, 4.0, 0.0), VoxelColor::new(0.0, 1.0, 0.0, 1.0)),
            VoxelInstance::new(vec3(0.0, 0.0, 2.0), VoxelColor::new(0.0, 0.0, 1.0, 1.0)),
            VoxelInstance::new(vec3(0.0, 0.0, 3.0), VoxelColor::new(0.0, 0.0, 1.0, 1.0)),
            VoxelInstance::new(vec3(0.0, 0.0, 4.0), VoxelColor::new(0.0, 0.0, 1.0, 1.0)),
        ];
        let mut position_to_voxel = BTreeMap::new();
        for voxel in voxels {
            position_to_voxel.insert(
                VoxelScenePosition::from_voxel_position(voxel.get_position()),
                voxel,
            );
        }
        Self {
            position_to_voxel: position_to_voxel,
            name: String::from("OrientationCross"),
            visible: true,
            editing_name: false,
            center: vec3(0.0, 0.0, 0.0),
        }
    }
    pub fn to_saved(&self) -> SavedVoxelGroup {
        SavedVoxelGroup {
            visible: self.visible,
            name: self.name.clone(),
            voxels: self
                .position_to_voxel
                .values()
                .map(|voxel| voxel.to_saved())
                .collect(),
        }
    }
    pub fn from_saved(save: SavedVoxelGroup) -> Self {
        let mut position_to_voxel = BTreeMap::new();
        for voxel in save.voxels {
            let scene_pos = VoxelScenePosition {
                x: voxel.x,
                y: voxel.y,
                z: voxel.z,
            };
            let voxel = VoxelInstance::from_saved(voxel);
            position_to_voxel.insert(scene_pos, voxel);
        }
        Self {
            position_to_voxel: position_to_voxel,
            name: save.name,
            visible: save.visible,
            editing_name: false,
            center: vec3(0.0, 0.0, 0.0),
        }
    }
    pub fn grid_voxel(dimensions: &GridVoxelDimensions) -> Self {
        Self {
            position_to_voxel: Self::axis_grid(dimensions),
            name: String::from("Grid Voxel Group"),
            visible: true,
            editing_name: false,
            center: vec3(0.0, 0.0, 0.0),
        }
    }
    pub fn merge_with(&mut self, other: VoxelGroup) {
        self.position_to_voxel.extend(other.position_to_voxel);
    }
    pub fn as_a_copy_of(other: &VoxelGroup, name: String) -> Self {
        Self {
            position_to_voxel: BTreeMap::clone(&other.position_to_voxel),
            name: name,
            visible: other.visible,
            editing_name: true,
            center: other.center,
        }
    }
    pub fn new(name: String) -> Self {
        Self {
            position_to_voxel: BTreeMap::new(),
            name: name,
            visible: true,
            editing_name: true,
            center: vec3(0.0, 0.0, 0.0),
        }
    }
    fn axis_grid(dimensions: &GridVoxelDimensions) -> BTreeMap<VoxelScenePosition, VoxelInstance> {
        let mut map = BTreeMap::new();
        for position in fill_positions_from_a_to_b(
            Vector3 {
                x: -(dimensions.width / 2.0),
                y: 0.0,
                z: -(dimensions.length / 2.0),
            },
            Vector3 {
                x: dimensions.width / 2.0,
                y: 0.0,
                z: dimensions.length / 2.0,
            },
        ) {
            let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
            let voxel = VoxelInstance::new_grid_voxel(
                position,
                VoxelColor::new(
                    crate::color::srgb_to_linear(0.4845),
                    crate::color::srgb_to_linear(0.4845),
                    crate::color::srgb_to_linear(0.4845),
                    1.0,
                ),
            );
            map.insert(voxel_scene_position, voxel);
        }
        map
    }
}
pub struct VoxelScene {
    voxel_groups: Vec<VoxelGroup>,
    current_voxel_groups_selected: Vec<usize>,
    temporary_voxels: Vec<VoxelInstance>,
    raw_voxel_instance_list: Vec<RawVoxelInstance>,
    voxels_changed: bool,
    voxel_group_name_counter: usize,
    saved: bool,
    grid_voxel_dimensions: GridVoxelDimensions,
}
impl VoxelScene {
    pub fn orientating_cross_scene() -> Self {
        let orientation_cross = VoxelGroup::orientation_cross();

        Self {
            voxel_groups: vec![orientation_cross],
            current_voxel_groups_selected: vec![0],
            temporary_voxels: vec![],
            raw_voxel_instance_list: vec![],
            voxels_changed: true,
            voxel_group_name_counter: 0,
            saved: false,
            grid_voxel_dimensions: GridVoxelDimensions {
                width: 32.0,
                length: 32.0,
            },
        }
    }
    pub fn to_saved(&self) -> SaveFile {
        SaveFile {
            panda: String::from("RESSA!"),
            version: format!("{}", env!("CARGO_PKG_VERSION")),
            groups: self
                .voxel_groups
                .iter()
                .enumerate()
                .filter(|(index, _)| *index > 0)
                .map(|(_, group)| group)
                .map(|group| group.to_saved())
                .collect(),
            name_counter: self.voxel_group_name_counter,
            grid_voxel_dimensions_length: self.grid_voxel_dimensions.length,
            grid_voxel_dimensions_width: self.grid_voxel_dimensions.width,
        }
    }
    pub fn from_saved(save: SaveFile) -> Self {
        let grid_dim = GridVoxelDimensions {
            length: save.grid_voxel_dimensions_length,
            width: save.grid_voxel_dimensions_width,
        };
        let mut voxel_groups = vec![VoxelGroup::grid_voxel(&grid_dim)];
        for group in save.groups {
            voxel_groups.push(VoxelGroup::from_saved(group));
        }
        Self {
            voxel_groups: voxel_groups,
            current_voxel_groups_selected: vec![1],
            temporary_voxels: Vec::new(),
            raw_voxel_instance_list: Vec::new(),
            voxels_changed: true,
            voxel_group_name_counter: save.name_counter,
            saved: true,
            grid_voxel_dimensions: grid_dim,
        }
    }
    pub fn new() -> Self {
        let mut voxel_groups = Vec::new();
        let grid_dim = GridVoxelDimensions {
            length: 32.0,
            width: 32.0,
        };
        voxel_groups.push(VoxelGroup::grid_voxel(&grid_dim));
        voxel_groups.push(VoxelGroup::new(format!("Voxel Group:{}", 1)));
        voxel_groups[1].editing_name = false;
        Self {
            voxel_groups: voxel_groups,
            current_voxel_groups_selected: vec![1],
            raw_voxel_instance_list: Vec::new(),
            voxels_changed: true,
            temporary_voxels: Vec::new(),
            voxel_group_name_counter: 2,
            saved: false,
            grid_voxel_dimensions: grid_dim,
        }
    }

    pub fn prepare_buffer_contents(&mut self) -> &Vec<RawVoxelInstance> {
        if self.voxels_changed {
            self.voxels_changed = false;
            self.raw_voxel_instance_list.clear();
            for i in (0..self.voxel_groups.len()).rev() {
                if self.voxel_groups[i].visible {
                    self.raw_voxel_instance_list.extend(
                        self.voxel_groups[i]
                            .position_to_voxel
                            .values()
                            .map(|voxel| voxel.to_raw()),
                    );
                }
            }
            self.raw_voxel_instance_list
                .extend(self.temporary_voxels.iter().map(|voxel| voxel.to_raw()));
        }
        self.temporary_voxels.clear();
        for i in self.current_voxel_groups_selected.iter() {
            self.voxel_groups[*i]
                .position_to_voxel
                .values_mut()
                .for_each(|voxel| voxel.unselect());
        }

        &self.raw_voxel_instance_list
    }
    pub fn is_voxel_scene_changed(&self) -> bool {
        self.voxels_changed
    }
    pub fn force_voxel_scene_update(&mut self) {
        self.voxels_changed = true;
    }
    pub fn get_voxel_grid_dimensions(&self) -> (f32, f32) {
        (
            self.grid_voxel_dimensions.width,
            self.grid_voxel_dimensions.length,
        )
    }
    pub fn resize_voxel_grid_dimensions(&mut self, width: &str, length: &str) {
        let width_num_opt = width.parse::<f32>().ok();
        let length_num_opt = length.parse::<f32>().ok();
        if let Some(length_num) = length_num_opt
            && let Some(width_num) = width_num_opt
        {
            let new_dim = GridVoxelDimensions {
                length: length_num,
                width: width_num,
            };
            if (length_num, width_num)
                != (
                    self.grid_voxel_dimensions.length,
                    self.grid_voxel_dimensions.width,
                )
            {
                self.grid_voxel_dimensions = new_dim;
                self.voxel_groups[0] = VoxelGroup::grid_voxel(&self.grid_voxel_dimensions);
            }
        }
    }
    pub fn get_saved(&self) -> bool {
        self.saved
    }
    pub fn set_saved(&mut self) {
        self.saved = true;
    }
    pub fn get_voxel_instance_count(&self) -> usize {
        self.voxel_groups
            .iter()
            .filter(|voxel_group| voxel_group.visible)
            .map(|vox_group| vox_group.position_to_voxel.len())
            .sum::<usize>()
            + self.temporary_voxels.len()
    }
    pub fn select_voxel_at_position(&mut self, position: Vector3<f32>) {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
        for i in self.current_voxel_groups_selected.iter() {
            if let Some(voxel) = self.voxel_groups[*i]
                .position_to_voxel
                .get_mut(&voxel_scene_position)
            {
                voxel.select();
            }
        }

        self.voxels_changed = true;
    }
    pub fn get_voxel_from_position_prioritizing_first_selected_set(
        &mut self,
        position: Vector3<f32>,
    ) -> Option<&mut VoxelInstance> {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
        if self.voxel_groups[self.current_voxel_groups_selected[0]]
            .position_to_voxel
            .contains_key(&voxel_scene_position)
        {
            return self.voxel_groups[self.current_voxel_groups_selected[0]]
                .position_to_voxel
                .get_mut(&voxel_scene_position);
        }
        for voxel_group in self.voxel_groups.iter_mut() {
            if let Some(voxel) = voxel_group.position_to_voxel.get_mut(&voxel_scene_position) {
                return Some(voxel);
            }
        }
        None
    }
    pub fn add_temporary_voxels(&mut self, positions: Vec<Vector3<f32>>, color: &VoxelColor) {
        for pos in positions {
            let voxel_to_add =
                VoxelInstance::new(pos, VoxelColor::new(color.r, color.g, color.b, color.a));
            self.voxels_changed = true;
            self.temporary_voxels.push(voxel_to_add);
        }
    }
    pub fn add_voxel(&mut self, position: Vector3<f32>, color: &VoxelColor) {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);

        for i in self.current_voxel_groups_selected.iter() {
            if !self.voxel_groups[*i]
                .position_to_voxel
                .contains_key(&voxel_scene_position)
            {
                let voxel_to_add = VoxelInstance::new(
                    position,
                    VoxelColor {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                        a: color.a,
                    },
                );
                self.voxels_changed = true;
                self.saved = false;
                self.voxel_groups[*i]
                    .position_to_voxel
                    .insert(voxel_scene_position, voxel_to_add);
            }
        }
        self.find_center();
    }
    pub fn remove_voxel(&mut self, position: Vector3<f32>) -> bool {
        let voxel_scene_position = VoxelScenePosition::from_voxel_position(position);
        let mut at_least_one_deletion = false;
        for i in self.current_voxel_groups_selected.iter() {
            if let Some(voxel) = self.voxel_groups[*i]
                .position_to_voxel
                .get(&voxel_scene_position)
            {
                if !voxel.is_grid() {
                    self.voxels_changed = true;
                    self.saved = false;
                    self.voxel_groups[*i]
                        .position_to_voxel
                        .remove(&voxel_scene_position);
                    at_least_one_deletion = true;
                }
            }
        }
        if at_least_one_deletion {
            self.find_center();
        }
        return at_least_one_deletion;
    }
    pub fn get_all_voxels(&self) -> Vec<&VoxelInstance> {
        let mut all_voxels = Vec::new();
        for i in 0..self.voxel_groups.len() {
            if self.voxel_groups[i].visible {
                all_voxels.extend(self.voxel_groups[i].position_to_voxel.values());
            }
        }
        all_voxels
    }
    fn find_center(&mut self) {
        for i in self.current_voxel_groups_selected.iter() {
            self.voxel_groups[*i].center = self.find_center_for_voxel_group(*i);
        }
    }
    fn find_center_for_voxel_group(&self, index: usize) -> Vector3<f32> {
        let mut sum = vec3(0.0, 0.0, 0.0);
        let mut n = 0.0;
        for voxel in self.voxel_groups[index].position_to_voxel.values() {
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
    fn move_overrides_grid(&self, move_vector: Vector3<f32>) -> bool {
        let move_scene_vector = VoxelScenePosition::from_voxel_position(move_vector);
        return self.current_voxel_groups_selected.iter().any(|index| {
            self.voxel_groups[*index]
                .position_to_voxel
                .iter()
                .filter(|(_, v)| !v.is_grid())
                .any(|(pos, _)| {
                    let dest = VoxelScenePosition {
                        x: pos.x + move_scene_vector.x,
                        y: pos.y + move_scene_vector.y,
                        z: pos.z + move_scene_vector.z,
                    };
                    let mut overrides = false;
                    for i in 0..self.voxel_groups.len() {
                        overrides = overrides
                            || self.voxel_groups[i]
                                .position_to_voxel
                                .get(&dest)
                                .is_some_and(|v| v.is_grid());
                    }
                    overrides
                })
        });
    }
    pub fn reposition_to_calculated_center(&mut self) {
        self.find_center();
        for i in self.current_voxel_groups_selected.clone() {
            let center = self.voxel_groups[i].center;
            self.move_by_vector(vec3(-center.x, 0.0, -center.z));
            let mut move_overrides_grid = self.move_overrides_grid(vec3(0.0, -1.0, 0.0));
            while !move_overrides_grid {
                self.move_by_vector(vec3(0.0, -1.0, 0.0));
                move_overrides_grid = self.move_overrides_grid(vec3(0.0, -1.0, 0.0));
            }
        }
    }
    pub fn rotate_around_center(&mut self, axis: Vector3<f32>, deg: cgmath::Deg<f32>) -> bool {
        for i in self.current_voxel_groups_selected.clone() {
            let center = self.voxel_groups[i].center;
            let rotation = Quaternion::from_axis_angle(axis.normalize(), deg);
            let old_keys: Vec<VoxelScenePosition> = self.voxel_groups[i]
                .position_to_voxel
                .iter()
                .filter_map(|(k, v)| (!v.is_grid()).then_some(*k))
                .collect();
            let mut smallest_y = f32::INFINITY;
            for voxel in self.voxel_groups[i].position_to_voxel.values_mut() {
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
                for voxel in self.voxel_groups[i].position_to_voxel.values_mut() {
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
                .map(|k| self.voxel_groups[i].position_to_voxel.remove(k).unwrap())
                .collect();

            for (_, voxel) in old_keys.into_iter().zip(voxels) {
                self.voxel_groups[i].position_to_voxel.insert(
                    VoxelScenePosition::from_voxel_position(voxel.get_position()),
                    voxel,
                );
            }
            self.voxels_changed = true;
            self.saved = false;
        }
        return true;
    }
    pub fn move_by_vector(&mut self, move_vector: Vector3<f32>) {
        let move_scene_vector = VoxelScenePosition::from_voxel_position(move_vector);
        for i in self.current_voxel_groups_selected.clone() {
            let mut old_keys: Vec<VoxelScenePosition> = Vec::new();
            old_keys.extend(
                self.voxel_groups[i]
                    .position_to_voxel
                    .iter()
                    .filter_map(|(k, v)| (!v.is_grid()).then_some(*k)),
            );

            for voxel in self.voxel_groups[i].position_to_voxel.values_mut() {
                if !voxel.is_grid() {
                    voxel.move_position_by_vector(move_vector);
                }
            }

            let voxels: Vec<VoxelInstance> = old_keys
                .iter()
                .map(|k| self.voxel_groups[i].position_to_voxel.remove(k).unwrap())
                .collect();

            for (old, voxel) in old_keys.into_iter().zip(voxels) {
                self.voxel_groups[i].position_to_voxel.insert(
                    VoxelScenePosition {
                        x: old.x + move_scene_vector.x,
                        y: old.y + move_scene_vector.y,
                        z: old.z + move_scene_vector.z,
                    },
                    voxel,
                );
            }
        }
        self.find_center();
        self.voxels_changed = true;
        self.saved = false;
    }
    pub fn set_current_voxel_group(&mut self, working_set: Vec<usize>) {
        self.current_voxel_groups_selected = working_set;
    }
    pub fn toggle_presence_in_current_voxel_group_selection(&mut self, additional_set: usize) {
        if !self.current_voxel_groups_selected.contains(&additional_set) {
            self.current_voxel_groups_selected.push(additional_set);
        } else if self.current_voxel_groups_selected.len() > 1 {
            self.current_voxel_groups_selected = self
                .current_voxel_groups_selected
                .clone()
                .into_iter()
                .filter(|element| *element != additional_set)
                .collect();
        }
    }
    pub fn get_current_voxel_groups(&self) -> &Vec<usize> {
        &self.current_voxel_groups_selected
    }
    pub fn get_voxel_group_names_and_indexes(&self) -> Vec<(usize, String, bool)> {
        self.voxel_groups
            .iter()
            .enumerate()
            .filter(|(index, _)| *index > 0)
            .map(|(index, set)| (index, set.name.clone(), set.editing_name))
            .collect()
    }
    pub fn is_voxel_group_visible(&self, group: usize) -> bool {
        if group > 0 && !self.voxel_groups.is_empty() && self.voxel_groups.len() > group {
            return self.voxel_groups[group].visible;
        }
        return false;
    }
    pub fn set_voxel_group_visibility(&mut self, group: usize, visible: bool) {
        if group > 0 && !self.voxel_groups.is_empty() && self.voxel_groups.len() > group {
            self.voxel_groups[group].visible = visible;
        }
        self.voxels_changed = true;
        self.saved = false;
    }
    pub fn add_voxel_group(&mut self) {
        let unique_name = self.turn_name_unique(
            format!("Voxel Group:{}", self.voxel_group_name_counter),
            None,
        );
        self.voxel_groups.push(VoxelGroup::new(unique_name));
        self.voxel_group_name_counter += 1;
    }
    pub fn remove_voxel_group(&mut self) {
        self.voxel_groups = self
            .voxel_groups
            .iter()
            .enumerate()
            .filter(|(index, _)| !self.current_voxel_groups_selected.contains(index))
            .map(|(_, voxel)| voxel.clone())
            .collect();
        self.current_voxel_groups_selected = vec![1];
        if self.voxel_groups.len() <= 1 {
            self.add_voxel_group();
        }
    }
    pub fn shift_voxel_group_down(&mut self) {
        let largest_index = *(self
            .current_voxel_groups_selected
            .iter()
            .max()
            .unwrap_or(&0));
        let insertion_name = self.voxel_groups
            [(largest_index + 1).min(self.voxel_groups.len() - 1)]
        .name
        .clone();
        let selected_names: Vec<String> = self
            .voxel_groups
            .iter()
            .enumerate()
            .filter(|(index, _)| self.current_voxel_groups_selected.contains(index))
            .map(|(_, group)| group.name.clone())
            .collect();
        let to_shift: Vec<VoxelGroup> = self
            .voxel_groups
            .extract_if(0..self.voxel_groups.len(), |element| {
                selected_names.contains(&element.name)
            })
            .collect();
        let insert_index = self
            .voxel_groups
            .iter()
            .position(|group| group.name == insertion_name)
            .map_or(self.voxel_groups.len(), |index| index + 1);
        self.voxel_groups
            .splice(insert_index..insert_index, to_shift);
        self.current_voxel_groups_selected = self
            .voxel_groups
            .iter()
            .enumerate()
            .filter(|(_, group)| selected_names.contains(&group.name))
            .map(|(index, _)| index.clone())
            .collect::<Vec<usize>>();
    }
    pub fn shift_voxel_group_up(&mut self) {
        let smallest_index = *(self
            .current_voxel_groups_selected
            .iter()
            .min()
            .unwrap_or(&0));
        let insertion_name = self.voxel_groups[(smallest_index - 1).max(1)].name.clone();
        let selected_names: Vec<String> = self
            .voxel_groups
            .iter()
            .enumerate()
            .filter(|(index, _)| self.current_voxel_groups_selected.contains(index))
            .map(|(_, group)| group.name.clone())
            .collect();
        let to_shift: Vec<VoxelGroup> = self
            .voxel_groups
            .extract_if(0..self.voxel_groups.len(), |element| {
                selected_names.contains(&element.name)
            })
            .collect();
        let insert_index = self
            .voxel_groups
            .iter()
            .position(|group| group.name == insertion_name)
            .map_or(1, |index| index);
        self.voxel_groups
            .splice(insert_index..insert_index, to_shift);
        self.current_voxel_groups_selected = self
            .voxel_groups
            .iter()
            .enumerate()
            .filter(|(_, group)| selected_names.contains(&group.name))
            .map(|(index, _)| index.clone())
            .collect::<Vec<usize>>();
    }
    pub fn merge_voxel_group(&mut self) {
        let shift_index = self.current_voxel_groups_selected[0] + 1;
        if shift_index > 0 && shift_index < self.voxel_groups.len() {
            let to_merge = self.voxel_groups.remove(shift_index);
            self.voxel_groups[self.current_voxel_groups_selected[0]].merge_with(to_merge);
        }
        if let Some(shift_pos) = self
            .current_voxel_groups_selected
            .iter()
            .position(|element| *element == shift_index)
        {
            self.current_voxel_groups_selected.remove(shift_pos);
        }
        self.voxels_changed = true;
        self.saved = false;
    }
    pub fn duplicate_voxel_group(&mut self) {
        let mut duplicates = Vec::new();
        for i in self.current_voxel_groups_selected.clone() {
            let unique_name = self.turn_name_unique(String::from(&self.voxel_groups[i].name), None);
            let duplicate = VoxelGroup::as_a_copy_of(&self.voxel_groups[i], unique_name);
            duplicates.push(duplicate);
        }
        let mut insertion_index = *self
            .current_voxel_groups_selected
            .iter()
            .max()
            .unwrap_or(&0)
            + 1;
        for _ in 0..duplicates.len() {
            self.voxel_groups
                .insert(insertion_index, duplicates.remove(0));
            insertion_index += 1;
        }
    }
    pub fn get_current_voxel_group_names(&self) -> Vec<&str> {
        self.current_voxel_groups_selected
            .iter()
            .map(|index| self.voxel_groups[*index].name.as_str())
            .collect::<Vec<&str>>()
    }
    pub fn get_voxel_group_name_ref_mut(&mut self, voxel_group: usize) -> Option<&mut String> {
        if voxel_group > 0 && voxel_group < self.voxel_groups.len() {
            return Some(&mut self.voxel_groups[voxel_group].name);
        }
        None
    }
    pub fn make_voxel_group_name_editable(&mut self, voxel_group: usize) {
        if voxel_group > 0 && voxel_group < self.voxel_groups.len() {
            self.reset_input_state();
            self.voxel_groups[voxel_group].editing_name = true;
        }
    }
    pub fn stop_voxel_group_edit(&mut self, voxel_group: usize) {
        if voxel_group > 0 && voxel_group < self.voxel_groups.len() {
            self.voxel_groups[voxel_group].editing_name = false;
            self.voxel_groups[voxel_group].name = self.turn_name_unique(
                String::from(&self.voxel_groups[voxel_group].name),
                Some(voxel_group),
            );
        }
    }
    fn turn_name_unique(&mut self, name: String, voxel_group: Option<usize>) -> String {
        let mut other_names = self
            .voxel_groups
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &VoxelGroup)>>();
        if let Some(voxel_group) = voxel_group {
            other_names = other_names
                .into_iter()
                .filter(|(index, _)| *index != voxel_group)
                .collect::<Vec<(usize, &VoxelGroup)>>();
        }
        let mut name_counter = 1;
        let og_name = name;
        let mut name = format!("{}", og_name);
        while other_names.iter().any(|(_, group)| group.name == name) {
            name = format!("{} ({})", &og_name, name_counter);
            name_counter += 1;
        }
        name
    }
    pub fn reset_input_state(&mut self) {
        for group in self.voxel_groups.iter_mut() {
            group.editing_name = false;
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct VoxelScenePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl VoxelScenePosition {
    pub fn from_voxel_position(position: Vector3<f32>) -> Self {
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
            VoxelSceneDirection::Backwards => (
                snap_vector_to_flat_direction(camera.right()),
                cgmath::Deg(90.0),
            ),
            VoxelSceneDirection::Forwards => (
                snap_vector_to_flat_direction(camera.right()),
                cgmath::Deg(-90.0),
            ),
            _ => (Vector3::new(0.0, 1.0, 0.0), Deg(0.0)),
        }
    }
}
