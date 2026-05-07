use std::sync::Arc;
use std::time;
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
    color_scale: f32,
    title: &'static str,
    render_start_time: Option<time::Instant>,
}

impl App {
    pub fn new(
        color_scale: f32,
        title: &'static str,
        render_start_time: Option<time::Instant>,
    ) -> Self {
        Self {
            state: None,
            color_scale,
            title,
            render_start_time,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title(self.title);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.state = Some(pollster::block_on(async {
            State::new(
                window.into(),
                self.color_scale,
            )
            .await
        }));

        self.render_start_time = Some(time::Instant::now());
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
                // Emits a new redraw requested event.
                state.window().request_redraw();
                let now = std::time::Instant::now();
                let dt = now - self.render_start_time.unwrap_or(now);
                state.update(dt);
            }
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
