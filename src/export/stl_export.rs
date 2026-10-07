use std::{collections::HashSet, mem, path::Path};

use cgmath::{Vector3, vec3};

use crate::{
    change::change::VoxelSnapshot, color::VoxelColor, ui_data::FileExportType,
    voxel_scene::VoxelScenePosition,
};
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct StlVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct StlFace {
    pub normal: Vector3<f32>,
    pub v1: StlVertex,
    pub v2: StlVertex,
    pub v3: StlVertex,
    pub attribute_byte_count: StlColorInAttributeValue,
}
impl StlFace {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        bytes.extend(bytemuck::cast_slice(&[
            self.normal.x,
            self.normal.y,
            self.normal.z,
        ]));
        bytes.extend(bytemuck::cast_slice(&[self.v1, self.v2, self.v3]));
        let attribute_byte_count: u16 = 0;
        bytes.extend(bytemuck::cast_slice(&[attribute_byte_count]));
        bytes
    }
}
pub struct StlColorInAttributeValue {
    pub color: VoxelColor,
}
impl StlColorInAttributeValue {
    // pub fn into_bytes(&self) -> u16 {}
}
pub struct StlExport {
    pub triangles: Vec<StlFace>,
}
impl StlExport {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let header_text = b"Ressapanda STL header";
        let mut header = [0u8; 80];
        header[..header_text.len()].copy_from_slice(header_text);
        bytes.extend(header);
        let triangle_count = self.triangles.len() as u32;
        bytes.extend(bytemuck::cast_slice(&[triangle_count]));
        bytes.extend(self.triangles.iter().map(|tri| tri.to_bytes()).flatten());
        bytes
    }
    pub fn decompose_voxel_list_to_stl_export_data(
        &mut self,
        voxels: Vec<VoxelSnapshot>,
        export_type: FileExportType,
    ) {
        let positions_occupied = voxels
            .iter()
            .map(|voxel| VoxelScenePosition {
                x: voxel.x,
                y: voxel.y,
                z: voxel.z,
            })
            .collect::<HashSet<_>>();
        let mut positions_processed = HashSet::new();
        for voxel in voxels {
            let position = VoxelScenePosition {
                x: voxel.x,
                y: voxel.y,
                z: voxel.z,
            };
            if !positions_processed.insert(position) {
                continue;
            }

            let faces: [(VoxelScenePosition, Vector3<f32>, [[f32; 3]; 4]); 6] = [
                (
                    VoxelScenePosition {
                        x: position.x,
                        y: position.y,
                        z: position.z - 1,
                    },
                    vec3(0.0, 0.0, -1.0),
                    [
                        [-0.5, -0.5, -0.5],
                        [-0.5, 0.5, -0.5],
                        [0.5, 0.5, -0.5],
                        [0.5, -0.5, -0.5],
                    ],
                ),
                (
                    VoxelScenePosition {
                        x: position.x,
                        y: position.y,
                        z: position.z + 1,
                    },
                    vec3(0.0, 0.0, 1.0),
                    [
                        [-0.5, -0.5, 0.5],
                        [0.5, -0.5, 0.5],
                        [0.5, 0.5, 0.5],
                        [-0.5, 0.5, 0.5],
                    ],
                ),
                (
                    VoxelScenePosition {
                        x: position.x - 1,
                        y: position.y,
                        z: position.z,
                    },
                    vec3(-1.0, 0.0, 0.0),
                    [
                        [-0.5, -0.5, -0.5],
                        [-0.5, -0.5, 0.5],
                        [-0.5, 0.5, 0.5],
                        [-0.5, 0.5, -0.5],
                    ],
                ),
                (
                    VoxelScenePosition {
                        x: position.x + 1,
                        y: position.y,
                        z: position.z,
                    },
                    vec3(1.0, 0.0, 0.0),
                    [
                        [0.5, -0.5, -0.5],
                        [0.5, 0.5, -0.5],
                        [0.5, 0.5, 0.5],
                        [0.5, -0.5, 0.5],
                    ],
                ),
                (
                    VoxelScenePosition {
                        x: position.x,
                        y: position.y - 1,
                        z: position.z,
                    },
                    vec3(0.0, -1.0, 0.0),
                    [
                        [-0.5, -0.5, -0.5],
                        [0.5, -0.5, -0.5],
                        [0.5, -0.5, 0.5],
                        [-0.5, -0.5, 0.5],
                    ],
                ),
                (
                    VoxelScenePosition {
                        x: position.x,
                        y: position.y + 1,
                        z: position.z,
                    },
                    vec3(0.0, 1.0, 0.0),
                    [
                        [-0.5, 0.5, -0.5],
                        [-0.5, 0.5, 0.5],
                        [0.5, 0.5, 0.5],
                        [0.5, 0.5, -0.5],
                    ],
                ),
            ];

            for (neighbor, normal_vector, corners) in faces {
                if positions_occupied.contains(&neighbor) {
                    continue;
                }
                let vertices_as_stl = corners.map(|offset| StlVertex {
                    x: neighbor.x as f32 + offset[0],
                    y: neighbor.y as f32 + offset[1],
                    z: neighbor.z as f32 + offset[2],
                });

                self.triangles.push(StlFace {
                    normal: normal_vector,
                    v1: vertices_as_stl[0],
                    v2: vertices_as_stl[1],
                    v3: vertices_as_stl[2],
                    attribute_byte_count: StlColorInAttributeValue {
                        color: VoxelColor::new(0.0, 0.0, 0.0, 0.0),
                    }, //fornow
                });
            }
        }
    }
}
pub fn export_stl_to_path(path: &Path, export: &StlExport) -> anyhow::Result<()> {
    let stl_bytes = export.to_bytes();
    std::fs::write(path, stl_bytes)?;
    Ok(())
}
