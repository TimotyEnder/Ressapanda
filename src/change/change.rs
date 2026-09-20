use std::collections::BTreeMap;

use cgmath::{Vector3, vec3};
use egui::accesskit::Role::Grid;

use crate::{
    change::change::Step::{
        AddGroup, GroupInfoChange, RemoveGroup, VoxelChange, VoxelGridResize, VoxelGroupDownShift,
        VoxelGroupUpShift,
    },
    voxel_instance::VoxelInstance,
    voxel_scene::{VoxelGroup, VoxelGroupId, VoxelScene, VoxelScenePosition},
};

pub struct VoxelSnapshot {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
pub enum Step {
    VoxelChange {
        // position-level delta (cheap hot path)
        group_id: VoxelGroupId,
        changes: BTreeMap<VoxelScenePosition, (Option<VoxelSnapshot>, Option<VoxelSnapshot>)>, // before, after
    },
    GroupInfoChange {
        group_id: VoxelGroupId,
        name: Option<(String, String)>,
        visible: Option<(bool, bool)>,
        center: Option<(Vector3<f32>, Vector3<f32>)>,
    },
    AddGroup {
        group_id: VoxelGroupId,
        name: String,
        visible: bool,
        center: Vector3<f32>,
    },
    RemoveGroup {
        group_id: VoxelGroupId,
        name: String,
        visible: bool,
        center: Vector3<f32>,
        voxels: Vec<VoxelSnapshot>,
    },
    VoxelGridResize {
        diff: ((f32, f32), (f32, f32)),
    },
    VoxelGroupDownShift {
        indices: Vec<usize>,
    },
    VoxelGroupUpShift {
        indices: Vec<usize>,
    },
}
impl Step {
    pub fn undo(&self, scene: &mut VoxelScene) {
        match self {
            VoxelChange { group_id, changes } => {
                for (pos, (before_opt, after_opt)) in changes.iter() {
                    if let Some(after) = after_opt {
                        let working_group_opt = scene
                            .voxel_groups_ref_mut()
                            .iter_mut()
                            .find(|group| group.id == *group_id);
                        if let Some(working_group) = working_group_opt {
                            let delete_pos = VoxelScenePosition {
                                x: after.x,
                                y: after.y,
                                z: after.z,
                            };
                            working_group.position_to_voxel.remove(&delete_pos);
                        }
                    }
                    if let Some(before) = before_opt {
                        let working_group = scene
                            .voxel_groups_ref_mut()
                            .iter_mut()
                            .find(|group| group.id == *group_id);
                        if let Some(group) = working_group {
                            let insert_pos = VoxelScenePosition {
                                x: before.x,
                                y: before.y,
                                z: before.z,
                            };
                            group
                                .position_to_voxel
                                .insert(insert_pos, VoxelInstance::from_snapshot(before));
                        }
                    }
                }
            }
            GroupInfoChange {
                group_id,
                name,
                visible,
                center,
            } => {
                let working_group_opt = scene
                    .voxel_groups_ref_mut()
                    .iter_mut()
                    .find(|group| group.id == *group_id);
                if let Some(working_group) = working_group_opt {
                    if let Some((before, after)) = name {
                        working_group.name = String::from(before);
                    }
                    if let Some((before, after)) = visible {
                        working_group.visible = *before
                    }
                    if let Some((before, after)) = center {
                        working_group.center = *before;
                    }
                }
            }
            AddGroup {
                group_id,
                name,
                visible,
                center,
            } => {
                let index_of_group_to_remove_opt = scene
                    .voxel_groups_ref_mut()
                    .iter_mut()
                    .position(|group| group.id == *group_id);
                if let Some(index_of_group_to_remove) = index_of_group_to_remove_opt {
                    scene
                        .voxel_groups_ref_mut()
                        .remove(index_of_group_to_remove);
                }
            }
            RemoveGroup {
                group_id,
                name,
                visible,
                center,
                voxels,
            } => {
                let mut voxel_group = VoxelGroup::new(String::from(name), *group_id);
                voxels
                    .iter()
                    .map(|snap_voxel| VoxelInstance::from_snapshot(snap_voxel))
                    .map(|voxel| {
                        (
                            VoxelScenePosition::from_voxel_position(voxel.get_position()),
                            voxel,
                        )
                    })
                    .for_each(|(key, value)| {
                        voxel_group.position_to_voxel.insert(key, value);
                    });
                voxel_group.visible = *visible;
                voxel_group.center = *center;
                scene.voxel_groups_ref_mut().push(voxel_group);
            }
            VoxelGridResize { diff } => {
                let ((before_w, before_l), (_)) = diff;
                scene.resize_voxel_grid_dimensions(&before_w.to_string(), &before_l.to_string());
            }
            VoxelGroupUpShift { indices } => {
                scene.set_current_voxel_group(indices.clone());
                scene.shift_voxel_group_down();
            }
            VoxelGroupDownShift { indices } => {
                scene.set_current_voxel_group(indices.clone());
                scene.shift_voxel_group_up();
            }
        }
    }
    pub fn redo(&self, scene: &mut VoxelScene) {
        match self {
            VoxelChange { group_id, changes } => {
                for (pos, (before_opt, after_opt)) in changes.iter() {
                    if let Some(before) = before_opt {
                        let working_group_opt = scene
                            .voxel_groups_ref_mut()
                            .iter_mut()
                            .find(|group| group.id == *group_id);
                        if let Some(working_group) = working_group_opt {
                            let delete_pos = VoxelScenePosition {
                                x: before.x,
                                y: before.y,
                                z: before.z,
                            };
                            working_group.position_to_voxel.remove(&delete_pos);
                        }
                    }
                    if let Some(after) = after_opt {
                        let working_group = scene
                            .voxel_groups_ref_mut()
                            .iter_mut()
                            .find(|group| group.id == *group_id);
                        if let Some(group) = working_group {
                            let insert_pos = VoxelScenePosition {
                                x: after.x,
                                y: after.y,
                                z: after.z,
                            };
                            group
                                .position_to_voxel
                                .insert(insert_pos, VoxelInstance::from_snapshot(after));
                        }
                    }
                }
            }
            GroupInfoChange {
                group_id,
                name,
                visible,
                center,
            } => {
                let working_group_opt = scene
                    .voxel_groups_ref_mut()
                    .iter_mut()
                    .find(|group| group.id == *group_id);
                if let Some(working_group) = working_group_opt {
                    if let Some((before, after)) = name {
                        working_group.name = String::from(after);
                    }
                    if let Some((before, after)) = visible {
                        working_group.visible = *after
                    }
                    if let Some((before, after)) = center {
                        working_group.center = *after;
                    }
                }
            }
            AddGroup {
                group_id,
                name,
                visible,
                center,
            } => {
                let mut voxel_group = VoxelGroup::new(String::from(name), *group_id);
                voxel_group.center = *center;
                voxel_group.visible = *visible;
                scene.voxel_groups_ref_mut().push(voxel_group);
            }
            RemoveGroup {
                group_id,
                name,
                visible,
                center,
                voxels,
            } => {
                let index_of_group_to_remove_opt = scene
                    .voxel_groups_ref_mut()
                    .iter_mut()
                    .position(|group| group.id == *group_id);
                if let Some(index_of_group_to_remove) = index_of_group_to_remove_opt {
                    scene
                        .voxel_groups_ref_mut()
                        .remove(index_of_group_to_remove);
                }
            }
            VoxelGridResize { diff } => {
                let ((_, _), (after_w, after_l)) = diff;
                scene.resize_voxel_grid_dimensions(&after_w.to_string(), &after_l.to_string());
            }
            VoxelGroupUpShift { indices } => {
                scene.set_current_voxel_group(indices.clone());
                scene.shift_voxel_group_up();
            }
            VoxelGroupDownShift { indices } => {
                scene.set_current_voxel_group(indices.clone());
                scene.shift_voxel_group_down();
            }
        }
    }
}
pub struct Change {
    steps: Vec<Step>,
}
