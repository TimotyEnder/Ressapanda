use std::{collections::HashSet, path::PathBuf};

use egui::Color32;
use egui_file_dialog::FileDialog;

use crate::icon_loader::IconLoader;

#[derive(Clone, Copy)]
pub enum FileAction {
    Open,
    Save,
}
pub struct UIData {
    pub ui_brush_color: Color32,
    pub last_color_added: Color32,
    pub last_used_colors: LastUsedColorsQueue,
    pub show_color_picker: bool,
    pub color_selected: bool,
    pub color_hex_input_string: String,
    pub icon_loader: IconLoader,
    pub voxel_group_removal_popup: bool,
    pub voxel_group_rename_request_focus_flag: bool,
    pub current_save_path: Option<PathBuf>,
    pub file_dialog: egui_file_dialog::FileDialog,
}
impl UIData {
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            ui_brush_color: Color32::from_rgb(255, 255, 255),
            show_color_picker: false,
            color_selected: false,
            last_used_colors: LastUsedColorsQueue::with_max_cap(10),
            last_color_added: Color32::from_rgb(255, 255, 255),
            color_hex_input_string: Color32::from_rgb(255, 255, 255).to_hex(),
            icon_loader: IconLoader::new(ctx),
            voxel_group_removal_popup: false,
            voxel_group_rename_request_focus_flag: false,
            current_save_path: None,
            file_dialog: FileDialog::new()
                .add_file_filter_extensions("Ressapanda Scene", vec!["rspnd"])
                .default_file_filter("Ressapanda Scene")
                .default_file_name("new_scene.rspnd")
                .add_save_extension("Ressapanda Scene", "rspnd")
                .default_save_extension("rspnd")
                .allow_file_overwrite(true),
        }
    }
}
pub struct LastUsedColorsQueue {
    queue: Vec<Color32>,
    max_cap: usize,
}
impl LastUsedColorsQueue {
    pub fn with_max_cap(max_cap: usize) -> Self {
        Self {
            queue: Vec::new(),
            max_cap: max_cap,
        }
    }
    pub fn push(&mut self, color: Color32) {
        self.queue.push(color);
        while self.queue.len() > self.max_cap {
            self.queue.remove(self.queue.len() - 1);
        }
    }
    pub fn get_colors_mut(&mut self) -> &mut Vec<Color32> {
        &mut self.queue
    }
}
