use std::{collections::HashSet, path::Path};

use anyhow::Ok;

use crate::{change::change::VoxelSnapshot, export, voxel_scene::VoxelScenePosition};

pub struct ObjVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ObjVertex {
    pub fn to_string(&self) -> String {
        format!("v {} {} {}", self.x, self.y, self.z)
    }
}

pub struct ObjFaceQuad {
    pub f1: usize,
    pub f2: usize,
    pub f3: usize,
    pub f4: usize,
    pub vn1: usize,
    pub vn2: usize,
    pub vn3: usize,
    pub vn4: usize,
}

impl ObjFaceQuad {
    pub fn to_string(&self) -> String {
        format!(
            "f {}//{} {}//{} {}//{} {}//{}",
            self.f1, self.vn1, self.f2, self.vn2, self.f3, self.vn3, self.f4, self.vn4
        )
    }
}

pub struct ObjExport {
    pub vertices: Vec<ObjVertex>,
    pub face_quads: Vec<ObjFaceQuad>,
}

impl ObjExport {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            face_quads: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut full_string =
            String::from("vn 0 0 -1\nvn 0 0 1\nvn -1 0 0\nvn 1 0 0\nvn 0 -1 0\nvn 0 1 0\n");
        for vertex in self.vertices.iter() {
            full_string += &format!("{}\n", vertex.to_string());
        }
        for quad in self.face_quads.iter() {
            full_string += &format!("{}\n", quad.to_string());
        }
        full_string
    }

    pub fn add_vertices_and_quads(
        &mut self,
        vertices: Vec<ObjVertex>,
        mut quads: Vec<ObjFaceQuad>,
    ) {
        let offset = self.vertices.len();
        self.vertices.extend(vertices);
        for quad in &mut quads {
            quad.f1 += offset;
            quad.f2 += offset;
            quad.f3 += offset;
            quad.f4 += offset;
        }
        self.face_quads.extend(quads);
    }

    pub fn decompose_voxel_list_to_obj_export_data(&mut self, voxels: Vec<VoxelSnapshot>) {
        let positions_occupied = voxels
            .iter()
            .map(|voxel| VoxelScenePosition {
                x: voxel.x,
                y: voxel.y,
                z: voxel.z,
            })
            .collect::<HashSet<_>>();
        let mut positions_processed = HashSet::new();
        let mut vertices = Vec::new();
        let mut quads = Vec::new();

        for voxel in voxels {
            let position = VoxelScenePosition {
                x: voxel.x,
                y: voxel.y,
                z: voxel.z,
            };
            if !positions_processed.insert(position) {
                continue;
            }

            let faces: [(VoxelScenePosition, usize, [[f32; 3]; 4]); 6] = [
                (
                    VoxelScenePosition {
                        x: position.x,
                        y: position.y,
                        z: position.z - 1,
                    },
                    1,
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
                    2,
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
                    3,
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
                    4,
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
                    5,
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
                    6,
                    [
                        [-0.5, 0.5, -0.5],
                        [-0.5, 0.5, 0.5],
                        [0.5, 0.5, 0.5],
                        [0.5, 0.5, -0.5],
                    ],
                ),
            ];

            for (neighbor, normal_index, corners) in faces {
                if positions_occupied.contains(&neighbor) {
                    continue;
                }

                let first_vertex = vertices.len() + 1;
                for [x_offset, y_offset, z_offset] in corners {
                    vertices.push(ObjVertex {
                        x: position.x as f32 + x_offset,
                        y: position.y as f32 + y_offset,
                        z: position.z as f32 + z_offset,
                    });
                }
                quads.push(ObjFaceQuad {
                    f1: first_vertex,
                    f2: first_vertex + 1,
                    f3: first_vertex + 2,
                    f4: first_vertex + 3,
                    vn1: normal_index,
                    vn2: normal_index,
                    vn3: normal_index,
                    vn4: normal_index,
                });
            }
        }

        self.add_vertices_and_quads(vertices, quads);
    }
}
pub fn export_obj_to_path(path: &Path, export: ObjExport) -> anyhow::Result<()> {
    let obj_string = export.to_string();
    std::fs::write(path, obj_string)?;
    Ok(())
}
