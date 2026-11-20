use crate::{
    filler::magic_fill_from_a,
    raycast::{find_first_voxel_to_intersect_ray, raycast_compute_from_mouse_position},
    select_mode::select_mode::SelectMode,
};

pub struct MagicWandSelectMode {}
impl MagicWandSelectMode {
    pub fn new() -> Self {
        Self {}
    }
}
impl SelectMode for MagicWandSelectMode {
    fn mouse_down(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: crate::tools::key_input_manager::ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &crate::brushes::brush::Brush,
    ) {
        let ray = raycast_compute_from_mouse_position(
            camera,
            mouse_x,
            mouse_y,
            config.width as f64,
            config.height as f64,
        );
        let hit_opt = find_first_voxel_to_intersect_ray(ray, scene);
        let skewed = scene.get_skewed_mode_status();
        if let Some((intersect_pos, voxel_pos)) = hit_opt {
            if tool.name() == "Add" {
                if let Some(voxel) =
                    scene.get_voxel_from_position_prioritizing_first_selected_set(voxel_pos)
                {
                    let operating_pos =
                        voxel.point_on_voxel_grid_closest_to_point(intersect_pos, skewed);
                    tool.operate_with_position(operating_pos, scene, brush);
                }
            } else {
                for pos in magic_fill_from_a(voxel_pos, scene) {
                    tool.operate_with_position(pos, scene, brush);
                }
            }
        }
    }

    fn mouse_up(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: crate::tools::key_input_manager::ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &crate::brushes::brush::Brush,
    ) {
        // nothing
    }

    fn temp_draw_on_mouse_hover(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        modifier_key_status: crate::tools::key_input_manager::ModifierKeysStatus,
        camera: &crate::camera::Camera,
        scene: &mut crate::voxel_scene::VoxelScene,
        config: &wgpu::SurfaceConfiguration,
        tool: &mut Box<dyn crate::tools::tool::Tool>,
        brush: &crate::brushes::brush::Brush,
    ) {
        let ray = raycast_compute_from_mouse_position(
            camera,
            mouse_x,
            mouse_y,
            config.width as f64,
            config.height as f64,
        );
        let hit_opt = find_first_voxel_to_intersect_ray(ray, scene);
        let skewed = scene.get_skewed_mode_status();
        if let Some((intersect_pos, voxel_pos)) = hit_opt {
            if tool.name() == "Add" {
                if let Some(voxel) =
                    scene.get_voxel_from_position_prioritizing_first_selected_set(voxel_pos)
                {
                    let operating_pos =
                        voxel.point_on_voxel_grid_closest_to_point(intersect_pos, skewed);
                    tool.temp_operate_with_position(operating_pos, scene, brush);
                }
            } else {
                for pos in magic_fill_from_a(voxel_pos, scene) {
                    tool.temp_operate_with_position(pos, scene, brush);
                }
            }
        } else {
            scene.force_voxel_scene_update();
        }
    }

    fn cursor_name(&self) -> &'static str {
        "xm"
    }

    fn name(&self) -> &'static str {
        "Wand"
    }

    fn tooltip(&self) -> &'static str {
        "Magic Wand. Selects adjacent voxels with a color difference based on the tolerance setting.(Shortcut:Y)"
    }
}
