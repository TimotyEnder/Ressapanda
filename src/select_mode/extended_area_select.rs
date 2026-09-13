use cgmath::{Point3, Vector3, vec3};
use log::log;

use crate::{
    brushes::brush::Brush,
    filler::fill_positions_from_a_to_b,
    raycast::{
        find_first_voxel_to_intersect_ray, find_pos_of_ray_vectors_closest_point_to_voxel_pos,
        raycast_compute_from_mouse_position,
    },
    select_mode::select_mode::SelectMode,
};

pub struct ExtendedAreaSelectMode {
    first_hit: Option<(Point3<f32>, Vector3<f32>)>,
    base_voxel: Option<(Vector3<f32>)>,
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
}
impl SelectMode for ExtendedAreaSelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
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
            let closest_point = find_pos_of_ray_vectors_closest_point_to_voxel_pos(ray, voxel_pos);
            if let Some(base_voxel) = self.base_voxel {
                for voxel_pos in fill_positions_from_a_to_b(
                    base_voxel,
                    vec3(voxel_pos.x, closest_point.y.round(), voxel_pos.z),
                ) {
                    tool.operate_with_position(voxel_pos, scene, brush);
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
                if tool.name().contains("Add") {
                    if let Some(prev_voxel) = scene.get_voxel_from_position(prev_voxel_position) {
                        prev_operating_point = prev_voxel
                            .point_on_voxel_grid_closest_to_point(prev_intersect_position);

                        tool.operate_with_position(prev_operating_point, scene, brush);
                    }
                    if let Some(voxel) = scene.get_voxel_from_position(voxel_position) {
                        operating_point =
                            voxel.point_on_voxel_grid_closest_to_point(intersect_position);
                        tool.operate_with_position(
                            Vector3::new(operating_point.x, operating_point.y, operating_point.z),
                            scene,
                            brush,
                        );
                    }
                }
                for list_voxel in fill_positions_from_a_to_b(prev_operating_point, operating_point)
                {
                    tool.operate_with_position(list_voxel, scene, brush);
                }
                self.base_voxel = Some(voxel_position);
                self.area_selected = true;
            }
        } else {
            scene.force_voxel_scene_update();
        }
    }

    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
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
                let closest = find_pos_of_ray_vectors_closest_point_to_voxel_pos(ray, voxel_pos);
                println!("{}", closest.y);
                if let Some(base_voxel) = self.base_voxel {
                    for list_voxel in fill_positions_from_a_to_b(
                        base_voxel,
                        vec3(voxel_pos.x, closest.y.round(), voxel_pos.z),
                    ) {
                        tool.temp_operate_with_position(list_voxel, scene, brush);
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
                    if tool.name().contains("Add") {
                        if let Some(prev_voxel) = scene.get_voxel_from_position(prev_voxel_position)
                        {
                            prev_operating_point = prev_voxel
                                .point_on_voxel_grid_closest_to_point(prev_intersect_position);

                            tool.temp_operate_with_position(prev_operating_point, scene, brush);
                        }
                        if let Some(voxel) = scene.get_voxel_from_position(voxel_position) {
                            operating_point =
                                voxel.point_on_voxel_grid_closest_to_point(intersect_position);
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
                    match tool.name() {
                        "Add" => {
                            if let Some(voxel) = scene.get_voxel_from_position(voxel_position) {
                                let point =
                                    voxel.point_on_voxel_grid_closest_to_point(intersect_position);
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
        "xen"
    }

    fn name(&self) -> &'static str {
        "Extended"
    }

    fn tooltip(&self) -> &'static str {
        "Extended Area Select(Shortcut:E)"
    }
}
