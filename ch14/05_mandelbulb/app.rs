use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::state::State;

pub struct App {
    state: Option<State>,
    max_iter: f32,
    scale: f32,
    mouse_control: bool,
    title: &'static str,
}

impl App {
    pub fn new(max_iter: f32, scale: f32, mouse_control: bool, title: &'static str) -> Self {
        Self {
            state: None,
            max_iter,
            scale,
            mouse_control,
            title,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title(self.title);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.state = Some(pollster::block_on(async {
            State::new(
                event_loop.owned_display_handle(),
                window.into(),
                self.max_iter,
                self.scale,
                self.mouse_control,
            )
            .await
        }));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                let _ = state.render();
                state.update();
            }
            WindowEvent::CursorMoved { position, .. } => {
                state.handle_mouse_moved(position.x, position.y);
            }
            WindowEvent::MouseInput {
                state: btn_state,
                button,
                ..
            } => state.handle_mouse_button(button, btn_state.is_pressed()),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key_input(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window().request_redraw();
        }
    }
}
