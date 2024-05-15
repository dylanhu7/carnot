use std::collections::HashSet;

use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta},
    keyboard::Key,
};

pub struct InputState {
    pub keys: HashSet<Key>,
    pub mouse_position: PhysicalPosition<f64>,
    pub last_mouse_position: Option<PhysicalPosition<f64>>,
    pub mouse_delta: (f64, f64),
    pub mouse_scroll_delta: MouseScrollDelta,
    pub mouse_buttons: HashSet<MouseButton>,
    pub clicked: bool,
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
            mouse_scroll_delta: MouseScrollDelta::LineDelta(0.0, 0.0),
            mouse_buttons: HashSet::new(),
            clicked: false,
        }
    }
}
