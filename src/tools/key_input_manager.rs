use std::{collections::HashMap, io::SeekFrom};

use cgmath::Vector3;
use winit::keyboard::KeyCode;

use crate::{
    select_mode::select_mode::{SelectMode, select_mode_from_name},
    state::State,
    tools::tool::{Tool, tool_from_name},
    voxel_scene::VoxelScene,
};

pub struct KeyInputManager {
    keycode_to_mapping: HashMap<KeyCode, &'static str>,
    keycode_to_flag: HashMap<KeyCode, bool>,
}
impl KeyInputManager {
    pub fn new() -> Self {
        let mut ret = Self {
            keycode_to_mapping: HashMap::new(),
            keycode_to_flag: HashMap::new(),
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
    pub fn tool_selection_inputs(&mut self, key: KeyCode, pressed: bool) -> Option<Box<dyn Tool>> {
        if let Some(name) = self.keycode_to_mapping.get(&key)
            && let Some(flag) = self.keycode_to_flag.get_mut(&key)
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
    pub fn move_commands_inputs(&mut self, key: KeyCode, pressed: bool, scene: &mut VoxelScene) {
        match key {
            KeyCode::ArrowUp => {
                if pressed {
                    scene.move_by_vector(Vector3::new(0.0, 0.0, 1.0));
                }
            }
            KeyCode::ArrowDown => {
                if pressed {
                    scene.move_by_vector(Vector3::new(0.0, 0.0, -1.0));
                }
            }
            KeyCode::ArrowLeft => {
                if pressed {
                    scene.move_by_vector(Vector3::new(-1.0, 0.0, 0.0));
                }
            }
            KeyCode::ArrowRight => {
                if pressed {
                    scene.move_by_vector(Vector3::new(1.0, 0.0, 0.0));
                }
            }
            KeyCode::Space => {
                if pressed {
                    scene.move_by_vector(Vector3::new(0.0, 1.0, 0.0));
                }
            }
            KeyCode::ControlLeft => {
                if pressed {
                    scene.move_by_vector(Vector3::new(0.0, -1.0, 0.0));
                }
            }
            _ => {}
        };
    }
}
