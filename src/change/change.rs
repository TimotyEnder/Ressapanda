use std::collections::BTreeMap;

use cgmath::Vector3;

use crate::{
    voxel_instance::VoxelInstance,
    voxel_scene::{VoxelGroupId, VoxelScenePosition},
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
    DeleteGroup {
        group_id: VoxelGroupId,
        name: String,
        visible: bool,
        center: Vector3<f32>,
        voxels: Vec<VoxelSnapshot>,
    },
    VoxelGridResize {
        diff: ((f32, f32), (f32, f32)),
    },
}
pub struct Change {
    steps: Vec<Step>,
}
