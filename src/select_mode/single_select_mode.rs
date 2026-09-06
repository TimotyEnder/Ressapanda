use cgmath::Vector3;

use crate::{
    brushes::brush::Brush,
    color::VoxelColor,
    raycast::{find_first_voxel_to_intersect_ray, raycast_compute_from_mouse_position},
    select_mode::select_mode::SelectMode,
    tools::{add::Add, tool::Tool},
};

pub struct SingleSelectMode {
    press_flag: bool,
}
impl SingleSelectMode {
    pub fn new() -> Self {
        Self { press_flag: false }
    }
}
impl SelectMode for SingleSelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
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
                        if let Some(voxel) = scene.get_voxel_from_position(voxel_position) {
                            let point =
                                voxel.point_on_voxel_grid_closest_to_point(intersect_position);
                            tool.operate_with_voxel_and_intersect(
                                Vector3::new(point.x, point.y, point.z),
                                scene,
                                brush,
                            );
                        }
                    }
                    _ => tool.operate_with_voxel_and_intersect(voxel_position, scene, brush),
                }
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
        tool: &mut Box<dyn Tool>,
        brush: &Brush,
    ) {
        self.press_flag = false;
    }

    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
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
                    if let Some(voxel) = scene.get_voxel_from_position(voxel_position) {
                        let point = voxel.point_on_voxel_grid_closest_to_point(intersect_position);
                        tool.temp_operate_with_voxel_and_intersect(
                            Vector3::new(point.x, point.y, point.z),
                            scene,
                            transparent_vers_off_brush,
                        );
                    }
                }
                _ => tool.temp_operate_with_voxel_and_intersect(
                    voxel_position,
                    scene,
                    &transparent_vers_off_brush,
                ),
            }
        } else {
            scene.force_voxel_scene_update();
        }
    }

    fn cursor_name(&self) -> &'static str {
        "x1_"
    }
}
