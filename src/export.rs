use std::{collections::HashSet, fmt::Write, path::Path};

use anyhow::Ok;

use crate::{change::change::VoxelSnapshot, color::VoxelColor, voxel_scene::VoxelScenePosition};
pub enum ObjExportType {
    Obj,
    ObjAndMtl,
}
pub struct ObjVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::fmt::Display for ObjVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v {} {} {}", self.x, self.y, self.z)
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
    pub color_index: Option<usize>,
}

impl std::fmt::Display for ObjFaceQuad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "f {}//{} {}//{} {}//{} {}//{}",
            self.f1, self.vn1, self.f2, self.vn2, self.f3, self.vn3, self.f4, self.vn4
        )
    }
}

pub struct MtlMaterial {
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub shininess: u16,
    pub dissolve: f32,
}
impl MtlMaterial {
    pub fn from_voxel_color(color: &VoxelColor) -> Self {
        Self {
            ambient_color: [0.0, 0.0, 0.0],
            diffuse_color: [color.r, color.g, color.b],
            specular_color: [0.0, 0.0, 0.0],
            shininess: 100,
            dissolve: color.a,
        }
    }
    pub fn material_def_to_string(&self, material_index: usize) -> String {
        format!(
            "newmtl Color{}\n Ka {} {} {}\n Kd {} {} {}\n Ks {} {} {}\n Ns {}\n d {}\n",
            material_index,
            self.ambient_color[0],
            self.ambient_color[1],
            self.ambient_color[2],
            self.diffuse_color[0],
            self.diffuse_color[1],
            self.diffuse_color[2],
            self.specular_color[0],
            self.specular_color[1],
            self.specular_color[2],
            self.shininess,
            self.dissolve
        )
    }
}

#[derive(Default)]
pub struct ObjExport {
    pub vertices: Vec<ObjVertex>,
    pub face_quads: Vec<ObjFaceQuad>,
    pub materials: Vec<MtlMaterial>,
}

impl ObjExport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_string(&self, mtl_file_name: String) -> String {
        let mut init_statement = String::new();
        if !self.materials.is_empty() {
            let _ = writeln!(init_statement, "mtllib {mtl_file_name}");
        }
        let mut full_string = format!(
            "{}vn 0 0 -1\nvn 0 0 1\nvn -1 0 0\nvn 1 0 0\nvn 0 -1 0\nvn 0 1 0\ns off\n",
            init_statement
        );
        for vertex in self.vertices.iter() {
            let _ = writeln!(full_string, "{vertex}");
        }
        for quad in self.face_quads.iter() {
            if let Some(index) = quad.color_index {
                let _ = writeln!(full_string, "usemtl Color{index}");
            }
            let _ = writeln!(full_string, "{quad}");
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
    pub fn convert_colors_into_mats(&mut self, colors: Vec<VoxelColor>) {
        let mats = colors
            .iter()
            .map(MtlMaterial::from_voxel_color)
            .collect::<Vec<MtlMaterial>>();
        self.materials.extend(mats);
    }

    pub fn decompose_voxel_list_to_obj_export_data(
        &mut self,
        voxels: Vec<VoxelSnapshot>,
        export_type: ObjExportType,
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
        let mut vertices = Vec::new();
        let mut quads = Vec::new();
        let mut colors = Vec::new();
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
                    color_index: {
                        match export_type {
                            ObjExportType::ObjAndMtl => {
                                if let Some(index) =
                                    colors.iter().position(|color| *color == voxel.color)
                                {
                                    Some(index)
                                } else {
                                    colors.push(voxel.color);
                                    Some(colors.len() - 1)
                                }
                            }
                            _ => None,
                        }
                    },
                });
            }
        }
        self.convert_colors_into_mats(colors);
        self.add_vertices_and_quads(vertices, quads);
    }
}
pub fn export_obj_to_path(path: &Path, export: &ObjExport) -> anyhow::Result<()> {
    let obj_string = export.to_string(String::from(
        path.with_extension("mtl")
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    ));
    std::fs::write(path, obj_string)?;
    Ok(())
}
pub fn export_materials_to_path(path: &Path, mats: &[MtlMaterial]) -> anyhow::Result<()> {
    if mats.is_empty() {
        return Ok(());
    }
    let mut full_mat_def = String::new();
    for (index, mat) in mats.iter().enumerate() {
        let _ = write!(full_mat_def, "{}", mat.material_def_to_string(index));
    }
    std::fs::write(path, full_mat_def)?;
    Ok(())
}
