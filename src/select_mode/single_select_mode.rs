use cgmath::Vector3;

use crate::{
    brushes::brush::Brush,
    color::VoxelColor,
    raycast::{find_first_voxel_to_intersect_ray, raycast_compute_from_mouse_position},
    select_mode::select_mode::SelectMode,
    tools::{key_input_manager::ModifierKeysStatus, tool::Tool},
};

pub struct SingleSelectMode {
    press_flag: bool,
    previous_voxel_pos_drawn: Option<Vector3<f32>>,
}
impl SingleSelectMode {
    pub fn new() -> Self {
        Self {
            press_flag: false,
            previous_voxel_pos_drawn: None,
        }
    }
}
impl SelectMode for SingleSelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        _modifier_key_status: ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
        brush: &Brush,
    ) {
        if !self.press_flag {
            self.press_flag = true;
            let ray = raycast_compute_from_mouse_position(
                camera,
                mouse_x,
                mouse_y,
                config.width as f64,
                config.height as f64,
            );
            let hit = find_first_voxel_to_intersect_ray(ray, scene);

            if let Some((intersect_position, voxel_position)) = hit {
                match tool.name() {
                    "Add" => {
                        if let Some(voxel) = scene
                            .get_voxel_from_position_prioritizing_first_selected_set(voxel_position)
                        {
                            let point =
                                voxel.point_on_voxel_grid_closest_to_point(intersect_position);
                            let op_position = Vector3::new(point.x, point.y, point.z);
                            tool.operate_with_position(op_position, scene, brush);
                            self.previous_voxel_pos_drawn = Some(op_position);
                        }
                    }
                    _ => {
                        tool.operate_with_position(voxel_position, scene, brush);
                        self.previous_voxel_pos_drawn = Some(voxel_position);
                    }
                }
            }
        }
    }

    fn mouse_up(
        &mut self,
        _mouse_x: f64,
        _mouse_y: f64,
        _modifier_key_status: ModifierKeysStatus,
        _camera: &crate::camera::Camera,
        _scene: &mut crate::voxel_scene::VoxelScene,
        _config: &wgpu::SurfaceConfiguration,
        _tool: &mut Box<dyn Tool>,
        _brush: &Brush,
    ) {
        self.press_flag = false;
    }

    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn Tool>,
        brush: &Brush,
    ) {
        let ray = raycast_compute_from_mouse_position(
            camera,
            mouse_x,
            mouse_y,
            config.width as f64,
            config.height as f64,
        );
        let hit = find_first_voxel_to_intersect_ray(ray, scene);
        let transparent_vers_off_brush = &Brush {
            color: VoxelColor::new(brush.color.r, brush.color.g, brush.color.b, 0.5),
        };
        if let Some((intersect_position, voxel_position)) = hit {
            match tool.name() {
                "Add" => {
                    if let Some(voxel) = scene
                        .get_voxel_from_position_prioritizing_first_selected_set(voxel_position)
                    {
                        let point = voxel.point_on_voxel_grid_closest_to_point(intersect_position);
                        let op_position = Vector3::new(point.x, point.y, point.z);
                        if self.press_flag && modifier_key_status.shift_modifier {
                            if let Some(prev_drawn) = self.previous_voxel_pos_drawn {
                                if prev_drawn == voxel_position {
                                    return;
                                }
                            }
                            tool.operate_with_position(op_position, scene, brush);
                            self.previous_voxel_pos_drawn = Some(op_position);
                        } else {
                            tool.temp_operate_with_position(
                                op_position,
                                scene,
                                transparent_vers_off_brush,
                            );
                        }
                    }
                }
                _ => {
                    if self.press_flag && modifier_key_status.shift_modifier {
                        if let Some(prev_drawn) = self.previous_voxel_pos_drawn {
                            if prev_drawn == voxel_position {
                                return;
                            }
                        }
                        tool.operate_with_position(voxel_position, scene, brush);
                        self.previous_voxel_pos_drawn = Some(voxel_position);
                    } else {
                        tool.temp_operate_with_position(
                            voxel_position,
                            scene,
                            &transparent_vers_off_brush,
                        )
                    }
                }
            }
        } else {
            scene.force_voxel_scene_update();
        }
    }

    fn cursor_name(&self) -> &'static str {
        "x1"
    }
    fn name(&self) -> &'static str {
        "Single"
    }

    fn tooltip(&self) -> &'static str {
        "Single Select Mode (Shortcut:Q). For continuous operation, hold (Shift)"
    }
}
