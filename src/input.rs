use std::collections::HashSet;

use winit::{dpi::PhysicalPosition, event::MouseScrollDelta, keyboard::Key};

pub struct InputState {
    pub keys: HashSet<Key>,
    pub mouse_position: PhysicalPosition<f64>,
    pub last_mouse_position: Option<PhysicalPosition<f64>>,
    pub mouse_delta: (f64, f64),
    pub mouse_wheel_delta: MouseScrollDelta,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            mouse_position: PhysicalPosition::new(0.0, 0.0),
            last_mouse_position: None,
            mouse_delta: (0.0, 0.0),
            mouse_wheel_delta: MouseScrollDelta::LineDelta(0.0, 0.0),
        }
    }
}
