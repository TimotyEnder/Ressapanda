use crate::{
    raycast::{find_first_voxel_to_intersect_ray, raycast_compute_from_mouse_position},
    select_mode::select_mode::SelectMode,
    tools::tool::Tool,
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
                tool.operate_with_voxel_and_intersect(voxel_position, intersect_position, scene);
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
    ) {
        todo!("Implement selecting a voxel")
    }
}
