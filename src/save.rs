use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::voxel_scene::{VoxelGroupId, VoxelScene};

#[derive(Serialize, Deserialize)]
pub struct SavedVoxel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[derive(Serialize, Deserialize)]
pub struct SavedVoxelGroup {
    pub visible: bool,
    pub name: String,
    pub id: VoxelGroupId,
    pub voxels: Vec<SavedVoxel>,
}
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub panda: String,
    pub version: String,
    pub groups: Vec<SavedVoxelGroup>,
    pub name_counter: usize,
    pub id_counter: VoxelGroupId,
    pub grid_voxel_dimensions_width: f32,
    pub grid_voxel_dimensions_length: f32,
}
pub fn save_to_file(scene: &VoxelScene, path: &Path) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(&scene.to_saved())?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn load_from_file(path: &Path) -> anyhow::Result<VoxelScene> {
    let json = std::fs::read_to_string(path)?;
    let saved: SaveFile = serde_json::from_str(&json)?;
    anyhow::ensure!(
        saved.panda == "RESSA!",
        "Panda is not RESSA! Filetype is not supported by Ressapanda!"
    );
    Ok(VoxelScene::from_saved(saved))
}
