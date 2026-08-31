use pollster::block_on;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, MouseButton, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::WindowAttributes,
};

use crate::state::State;

pub struct App {
    state: Option<State>,
}
impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}
impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut window_attributes = WindowAttributes::default();
        window_attributes.title = String::from("Zuku!");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        // Log the error and exit gracefully
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::MouseInput {
                state: mouse_button_state,
                button,
                ..
            } => match (button, mouse_button_state.is_pressed()) {
                (button, pressed) => {
                    state.handle_mouse_button(event_loop, button, pressed);
                }
            },
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                state.handle_mouse_input(position);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}
pub fn run() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
