use cgmath::{Point3, Vector3};

use crate::{
    brushes::brush::Brush,
    filler::{fill_line_from_a_to_b, fill_positions_from_a_to_b},
    raycast::{find_first_voxel_to_intersect_ray, raycast_compute_from_mouse_position},
    select_mode::select_mode::SelectMode,
    tools::key_input_manager::ModifierKeysStatus,
};

pub struct LineSelectMode {
    previous_hit: Option<(Point3<f32>, Vector3<f32>)>,
}
impl LineSelectMode {
    pub fn new() -> Self {
        Self { previous_hit: None }
    }
}
impl SelectMode for LineSelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        _modifier_key_status: ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        _tool: &mut Box<dyn crate::tools::tool::Tool>,
        _brush: &Brush,
    ) {
        let ray = raycast_compute_from_mouse_position(
            camera,
            mouse_x,
            mouse_y,
            config.width as f64,
            config.height as f64,
        );
        self.previous_hit = find_first_voxel_to_intersect_ray(ray, scene);
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
        if let Some((prev_intersect_position, prev_voxel_position)) = self.previous_hit {
            if let Some((intersect_position, voxel_position)) =
                find_first_voxel_to_intersect_ray(ray, scene)
            {
                let mut prev_operating_point = prev_voxel_position;
                let mut operating_point = voxel_position;
                if tool.name().contains("Add") {
                    if let Some(prev_voxel) = scene
                        .get_voxel_from_position_prioritizing_first_selected_set(
                            prev_voxel_position,
                        )
                    {
                        prev_operating_point = prev_voxel
                            .point_on_voxel_grid_closest_to_point(prev_intersect_position);

                        tool.operate_with_position(prev_operating_point, scene, brush);
                    }
                    if let Some(voxel) = scene
                        .get_voxel_from_position_prioritizing_first_selected_set(voxel_position)
                    {
                        operating_point =
                            voxel.point_on_voxel_grid_closest_to_point(intersect_position);
                        tool.operate_with_position(
                            Vector3::new(operating_point.x, operating_point.y, operating_point.z),
                            scene,
                            brush,
                        );
                    }
                }
                for list_voxel in fill_line_from_a_to_b(
                    prev_operating_point,
                    Vector3::new(operating_point.x, operating_point.y, operating_point.z),
                ) {
                    tool.operate_with_position(list_voxel, scene, brush);
                }
            }
            self.previous_hit = None;
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
        if let Some((intersect_position, voxel_position)) =
            find_first_voxel_to_intersect_ray(ray, scene)
        {
            if let Some((prev_intersect_position, prev_voxel_position)) = self.previous_hit {
                let mut prev_operating_point = prev_voxel_position;
                let mut operating_point = voxel_position;
                if tool.name().contains("Add") {
                    if let Some(prev_voxel) = scene
                        .get_voxel_from_position_prioritizing_first_selected_set(
                            prev_voxel_position,
                        )
                    {
                        prev_operating_point = prev_voxel
                            .point_on_voxel_grid_closest_to_point(prev_intersect_position);

                        tool.temp_operate_with_position(prev_operating_point, scene, brush);
                    }
                    if let Some(voxel) = scene
                        .get_voxel_from_position_prioritizing_first_selected_set(voxel_position)
                    {
                        operating_point =
                            voxel.point_on_voxel_grid_closest_to_point(intersect_position);
                        tool.temp_operate_with_position(
                            Vector3::new(operating_point.x, operating_point.y, operating_point.z),
                            scene,
                            brush,
                        );
                    }
                }
                for list_voxel in fill_line_from_a_to_b(
                    prev_operating_point,
                    Vector3::new(operating_point.x, operating_point.y, operating_point.z),
                ) {
                    tool.temp_operate_with_position(list_voxel, scene, brush);
                }
            } else {
                match tool.name() {
                    "Add" => {
                        if let Some(voxel) = scene
                            .get_voxel_from_position_prioritizing_first_selected_set(voxel_position)
                        {
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
    fn cursor_name(&self) -> &'static str {
        "xln"
    }

    fn name(&self) -> &'static str {
        "Line"
    }

    fn tooltip(&self) -> &'static str {
        "Line Select Mode: drag to form uniform line between two voxels. Selects the voxels in between (Shortcut:T)"
    }
}
