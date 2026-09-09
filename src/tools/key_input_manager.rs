use std::{collections::HashMap, io::SeekFrom};

use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};
use winit::keyboard::KeyCode;

use crate::{
    camera::{Camera, CameraLookDirection},
    camera_controller::CameraController,
    conversion_utils::snap_vector_to_flat_direction,
    select_mode::select_mode::{SelectMode, select_mode_from_name},
    state::State,
    tools::tool::{Tool, tool_from_name},
    voxel_scene::{VoxelScene, VoxelSceneDirection},
};

pub struct KeyInputManager {
    keycode_to_mapping: HashMap<KeyCode, &'static str>,
    keycode_to_flag: HashMap<KeyCode, bool>,
    move_modifier: bool,
    rotate_modifier: bool,
}
impl KeyInputManager {
    pub fn new() -> Self {
        let mut ret = Self {
            keycode_to_mapping: HashMap::new(),
            keycode_to_flag: HashMap::new(),
            move_modifier: false,
            rotate_modifier: false,
        };
        //Tools
        ret.keycode_to_mapping.insert(KeyCode::KeyA, "Add");
        ret.keycode_to_flag.insert(KeyCode::KeyA, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyD, "Del");
        ret.keycode_to_flag.insert(KeyCode::KeyD, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyS, "Subs");
        ret.keycode_to_flag.insert(KeyCode::KeyS, false);

        //selection modes
        ret.keycode_to_mapping.insert(KeyCode::KeyQ, "Single");
        ret.keycode_to_flag.insert(KeyCode::KeyQ, false);
        ret.keycode_to_mapping.insert(KeyCode::KeyW, "Area");
        ret.keycode_to_flag.insert(KeyCode::KeyW, false);
        ret
    }
    pub fn modifier_inputs(&mut self, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::ShiftLeft => {
                self.rotate_modifier = pressed;
            }
            KeyCode::AltLeft => {
                self.move_modifier = pressed;
            }
            _ => {}
        };
    }
    pub fn tool_selection_inputs(&mut self, key: KeyCode, pressed: bool) -> Option<Box<dyn Tool>> {
        if let Some(name) = self.keycode_to_mapping.get(&key)
            && let Some(flag) = self.keycode_to_flag.get_mut(&key)
            && !self.move_modifier
            && !self.rotate_modifier
        {
            if !*flag && pressed {
                if let Some(tool) = tool_from_name(name) {
                    return Some(tool);
                }
            } else if !pressed {
                *flag = false;
            }
        }
        return None;
    }
    pub fn select_mode_selection_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
    ) -> Option<Box<dyn SelectMode>> {
        if let Some(name) = self.keycode_to_mapping.get(&key)
            && let Some(flag) = self.keycode_to_flag.get_mut(&key)
            && !self.move_modifier
            && !self.rotate_modifier
        {
            if !*flag && pressed {
                if let Some(select_mode) = select_mode_from_name(name) {
                    return Some(select_mode);
                }
            } else if !pressed {
                *flag = false;
            }
        }
        return None;
    }
    pub fn camera_preset_positions_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        camera: &mut Camera,
        camera_controller: &mut CameraController,
    ) {
        if pressed {
            let target = Point3::new(0.0, 0.0, 0.0);
            let look_direction = match key {
                KeyCode::Digit1 => {
                    // x (view along +X)
                    CameraLookDirection::Xplus
                }
                KeyCode::Digit2 => {
                    // z (view along +Z)
                    CameraLookDirection::Zplus
                }
                KeyCode::Digit3 => {
                    // x backwards (view along -X)
                    CameraLookDirection::Xminus
                }
                KeyCode::Digit4 => {
                    // z backwards (view along -Z)
                    CameraLookDirection::Zminus
                }
                KeyCode::Digit5 => {
                    // straight up (view straight down)
                    CameraLookDirection::Down
                }
                _ => CameraLookDirection::None,
            };
            if look_direction != CameraLookDirection::None {
                let eye = look_direction.eye_position();
                camera_controller.set_preset_orbit(eye);
                camera.set_position(target, target + eye);
            }
        }
    }
    pub fn move_commands_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        scene: &mut VoxelScene,
        camera: &Camera,
    ) {
        if self.move_modifier {
            self.move_inputs(key, pressed, scene, camera);
        } else if self.rotate_modifier {
            self.rotate_inputs(key, pressed, scene, camera);
        }
    }
    fn move_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        scene: &mut VoxelScene,
        camera: &Camera,
    ) {
        let direction = match key {
            KeyCode::KeyW => VoxelSceneDirection::Forwards,
            KeyCode::KeyS => VoxelSceneDirection::Backwards,
            KeyCode::KeyA => VoxelSceneDirection::Left,
            KeyCode::KeyD => VoxelSceneDirection::Right,
            KeyCode::KeyQ => VoxelSceneDirection::UpLeftSteer,
            KeyCode::KeyE => VoxelSceneDirection::DownRightSteer,
            _ => VoxelSceneDirection::None,
        };
        if pressed && direction != VoxelSceneDirection::None {
            let move_vec = direction.move_vector(camera);
            scene.move_by_vector(move_vec);
        }
    }
    fn rotate_inputs(
        &mut self,
        key: KeyCode,
        pressed: bool,
        scene: &mut VoxelScene,
        camera: &Camera,
    ) {
        let direction = match key {
            KeyCode::KeyW => VoxelSceneDirection::Forwards,
            KeyCode::KeyS => VoxelSceneDirection::Backwards,
            KeyCode::KeyA => VoxelSceneDirection::Left,
            KeyCode::KeyD => VoxelSceneDirection::Right,
            KeyCode::KeyQ => VoxelSceneDirection::UpLeftSteer,
            KeyCode::KeyE => VoxelSceneDirection::DownRightSteer,
            _ => VoxelSceneDirection::None,
        };
        if pressed && direction != VoxelSceneDirection::None {
            let (axis, deg) = direction.rotate_parameters(camera);
            scene.rotate_around_center(axis, deg);
        }
    }
}
