use std::collections::HashSet;

use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputHandler {
    pressed_keys: HashSet<KeyCode>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn submit_winit_event(&mut self, event: &winit::event::WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {  event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else {
                    return;
                };

                match event.state {
                    ElementState::Pressed => {
                        self.pressed_keys.insert(code);
                    },
                    ElementState::Released => {
                        self.pressed_keys.remove(&code);
                    },
                }
            }
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
}
