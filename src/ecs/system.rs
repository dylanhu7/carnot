use crate::{input::InputState, render::Renderer};

use super::World;

pub type System = Box<dyn Fn(&mut World, &mut Renderer, &mut InputState)>;
