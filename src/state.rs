use std::sync::Arc;

use anyhow::Ok;
use winit::{
    dpi::PhysicalPosition, event::MouseButton, event_loop::ActiveEventLoop, keyboard::KeyCode,
    window::Window,
};

pub struct State {
    window: Arc<Window>,
}
impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        Ok(Self { window })
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        if key == KeyCode::Escape && pressed {
            event_loop.exit();
        }
    }
    pub fn handle_mouse_button(
        &mut self,
        event_loop: &ActiveEventLoop,
        button: MouseButton,
        pressed: bool,
    ) {
    }
    pub fn handle_mouse_input(&mut self, pos: PhysicalPosition<f64>) {
        //
    }
    pub fn update(&mut self) {
        //
    }
    pub fn render(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    pub fn resize(&mut self, width: u32, height: u32) {}
}
