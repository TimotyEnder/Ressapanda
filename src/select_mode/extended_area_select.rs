use cgmath::{MetricSpace, Point3, Vector3, point3, vec3};

use crate::{
    brushes::brush::Brush,
    filler::fill_positions_from_a_to_b,
    raycast::{
        find_first_voxel_to_intersect_ray, find_pos_of_ray_vectors_closest_point_to_voxel_pos,
        raycast_compute_from_mouse_position,
    },
    select_mode::select_mode::SelectMode,
    tools::key_input_manager::ModifierKeysStatus,
};

pub struct ExtendedAreaSelectMode {
    first_hit: Option<(Point3<f32>, Vector3<f32>)>,
    base_voxel: Option<Vector3<f32>>,
    area_selected: bool,
}
impl ExtendedAreaSelectMode {
    pub fn new() -> Self {
        Self {
            first_hit: None,
            base_voxel: None,
            area_selected: false,
        }
    }
    fn fill_position(
        _base_voxel: Vector3<f32>,
        voxel_pos: Vector3<f32>,
        face_mid: Vector3<f32>,
        closest_point: Point3<f32>,
    ) -> Vector3<f32> {
        let dist = face_mid
            .distance(vec3(closest_point.x, closest_point.y, closest_point.z))
            .round();
        let mut positions = [
            (dist * vec3(1.0, 0.0, 0.0)),
            (dist * vec3(-1.0, 0.0, 0.0)),
            (dist * vec3(0.0, 1.0, 0.0)),
            (dist * vec3(0.0, -1.0, 0.0)),
            (dist * vec3(0.0, 0.0, 1.0)),
            (dist * vec3(0.0, 0.0, -1.0)),
        ];
        positions.sort_by(|posa, posb| {
            (face_mid + posa)
                .distance(vec3(closest_point.x, closest_point.y, closest_point.z))
                .total_cmp(&(face_mid + posb).distance(vec3(
                    closest_point.x,
                    closest_point.y,
                    closest_point.z,
                )))
        });
        voxel_pos + positions[0]
    }
}
impl SelectMode for ExtendedAreaSelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        _modifier_key_status: ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &Brush,
    ) {
        if !self.area_selected {
            let ray = raycast_compute_from_mouse_position(
                camera,
                mouse_x,
                mouse_y,
                config.width as f64,
                config.height as f64,
            );
            self.first_hit = find_first_voxel_to_intersect_ray(ray, scene);
        } else if self.area_selected
            && let Some((_, voxel_pos)) = self.first_hit
        {
            let ray = raycast_compute_from_mouse_position(
                camera,
                mouse_x,
                mouse_y,
                config.width as f64,
                config.height as f64,
            );

            if let Some(base_voxel) = self.base_voxel {
                let face_mid_point = vec3(
                    (base_voxel.x + voxel_pos.x) / 2.0,
                    (base_voxel.y + voxel_pos.y) / 2.0,
                    (base_voxel.z + voxel_pos.z) / 2.0,
                );
                let closest_to_face_mid_point =
                    find_pos_of_ray_vectors_closest_point_to_voxel_pos(ray, face_mid_point);
                let fill_point = point3(
                    if closest_to_face_mid_point.x == face_mid_point.x {
                        face_mid_point.x
                    } else {
                        closest_to_face_mid_point.x
                    },
                    if closest_to_face_mid_point.y == face_mid_point.y {
                        face_mid_point.y
                    } else {
                        closest_to_face_mid_point.y
                    },
                    if closest_to_face_mid_point.z == face_mid_point.z {
                        face_mid_point.z
                    } else {
                        closest_to_face_mid_point.z
                    },
                );
                for pos in fill_positions_from_a_to_b(
                    base_voxel,
                    Self::fill_position(base_voxel, voxel_pos, face_mid_point, fill_point),
                ) {
                    tool.operate_with_position(pos, scene, brush);
                }
                self.base_voxel = None;
                self.first_hit = None;
                self.area_selected = false;
            }
        }
    }
    fn mouse_up(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        _modifier_key_status: ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &Brush,
    ) {
        let ray = raycast_compute_from_mouse_position(
            camera,
            mouse_x,
            mouse_y,
            config.width as f64,
            config.height as f64,
        );
        if let Some((prev_intersect_position, prev_voxel_position)) = self.first_hit {
            if let Some((intersect_position, voxel_position)) =
                find_first_voxel_to_intersect_ray(ray, scene)
            {
                let mut prev_operating_point = prev_voxel_position;
                let mut operating_point = voxel_position;
                let skewed = scene.get_skewed_mode_status();
                if tool.name().contains("Add") {
                    if let Some(prev_voxel) = scene
                        .get_voxel_from_position_prioritizing_first_selected_set(
                            prev_voxel_position,
                        )
                    {
                        prev_operating_point = prev_voxel
                            .point_on_voxel_grid_closest_to_point(prev_intersect_position, skewed);
                    }
                    if let Some(voxel) = scene
                        .get_voxel_from_position_prioritizing_first_selected_set(voxel_position)
                    {
                        operating_point =
                            voxel.point_on_voxel_grid_closest_to_point(intersect_position, skewed);
                    }
                }
                self.base_voxel = Some(operating_point);
                self.first_hit = Some((intersect_position, prev_operating_point));
                self.area_selected = true;
                if voxel_position.x != prev_voxel_position.x
                    && voxel_position.y != prev_voxel_position.y
                    && voxel_position.z != prev_voxel_position.z
                {
                    for list_voxel in
                        fill_positions_from_a_to_b(prev_operating_point, operating_point)
                    {
                        tool.operate_with_position(list_voxel, scene, brush);
                    }
                    self.base_voxel = None;
                    self.first_hit = None;
                    self.area_selected = false;
                }
            }
        } else {
            scene.force_voxel_scene_update();
        }
    }

    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        _modifier_key_status: ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &Brush,
    ) {
        let ray = raycast_compute_from_mouse_position(
            camera,
            mouse_x,
            mouse_y,
            config.width as f64,
            config.height as f64,
        );
        if self.area_selected {
            if let Some((_, voxel_pos)) = self.first_hit {
                let closest_point =
                    find_pos_of_ray_vectors_closest_point_to_voxel_pos(ray, voxel_pos);
                if let Some(base_voxel) = self.base_voxel {
                    let face_mid_point = vec3(
                        (base_voxel.x + voxel_pos.x) / 2.0,
                        (base_voxel.y + voxel_pos.y) / 2.0,
                        (base_voxel.z + voxel_pos.z) / 2.0,
                    );
                    let closest_to_face_mid_point =
                        find_pos_of_ray_vectors_closest_point_to_voxel_pos(ray, face_mid_point);
                    let fill_point = point3(
                        if closest_to_face_mid_point.x == face_mid_point.x {
                            face_mid_point.x
                        } else {
                            closest_to_face_mid_point.x
                        },
                        if closest_to_face_mid_point.y == face_mid_point.y {
                            face_mid_point.y
                        } else {
                            closest_to_face_mid_point.y
                        },
                        if closest_to_face_mid_point.z == face_mid_point.z {
                            face_mid_point.z
                        } else {
                            closest_to_face_mid_point.z
                        },
                    );
                    for pos in fill_positions_from_a_to_b(
                        base_voxel,
                        Self::fill_position(base_voxel, voxel_pos, face_mid_point, fill_point),
                    )
                    .iter()
                    .chain(fill_positions_from_a_to_b(base_voxel, voxel_pos).iter())
                    {
                        tool.temp_operate_with_position(*pos, scene, brush);
                    }
                }
            }
        } else {
            if let Some((intersect_position, voxel_position)) =
                find_first_voxel_to_intersect_ray(ray, scene)
            {
                if let Some((prev_intersect_position, prev_voxel_position)) = self.first_hit {
                    let mut prev_operating_point = prev_voxel_position;
                    let mut operating_point = voxel_position;
                    let skewed = scene.get_skewed_mode_status();
                    if tool.name().contains("Add") {
                        if let Some(prev_voxel) = scene
                            .get_voxel_from_position_prioritizing_first_selected_set(
                                prev_voxel_position,
                            )
                        {
                            prev_operating_point = prev_voxel.point_on_voxel_grid_closest_to_point(
                                prev_intersect_position,
                                skewed,
                            );

                            tool.temp_operate_with_position(prev_operating_point, scene, brush);
                        }
                        if let Some(voxel) = scene
                            .get_voxel_from_position_prioritizing_first_selected_set(voxel_position)
                        {
                            operating_point = voxel
                                .point_on_voxel_grid_closest_to_point(intersect_position, skewed);
                            tool.temp_operate_with_position(
                                Vector3::new(
                                    operating_point.x,
                                    operating_point.y,
                                    operating_point.z,
                                ),
                                scene,
                                brush,
                            );
                        }
                    }
                    for list_voxel in fill_positions_from_a_to_b(
                        prev_operating_point,
                        Vector3::new(operating_point.x, operating_point.y, operating_point.z),
                    ) {
                        tool.temp_operate_with_position(list_voxel, scene, brush);
                    }
                } else {
                    let skewed = scene.get_skewed_mode_status();
                    match tool.name() {
                        "Add" => {
                            if let Some(voxel) = scene
                                .get_voxel_from_position_prioritizing_first_selected_set(
                                    voxel_position,
                                )
                            {
                                let point = voxel.point_on_voxel_grid_closest_to_point(
                                    intersect_position,
                                    skewed,
                                );
                                tool.temp_operate_with_position(
                                    Vector3::new(point.x, point.y, point.z),
                                    scene,
                                    &Brush {
                                        color: crate::color::VoxelColor {
                                            r: brush.color.r,
                                            g: brush.color.g,
                                            b: brush.color.b,
                                            a: 0.5,
                                        },
                                    },
                                );
                            }
                        }
                        _ => tool.temp_operate_with_position(voxel_position, scene, brush),
                    }
                }
            } else {
                scene.force_voxel_scene_update();
            }
        }
    }
    fn cursor_name(&self) -> &'static str {
        if self.area_selected { "xee" } else { "xe" }
    }

    fn name(&self) -> &'static str {
        "Extended"
    }

    fn tooltip(&self) -> &'static str {
        "Extended Area Select(Shortcut:E)"
    }
}
