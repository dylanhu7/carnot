pub mod app;
pub mod builtins;
pub mod ecs;
pub mod graphics;
pub mod input;
pub mod render;

pub mod prelude {
    pub use crate::app::{App, SystemStage::*};
    pub use crate::builtins::primitives::Primitive;
    pub use crate::builtins::systems::ActiveCamera;
    pub use crate::ecs::{
        query::Query,
        resource::{Res, ResMut},
        World,
    };
    pub use crate::graphics::{Mesh, PerspectiveCamera, Transform};
    pub use crate::input::InputState;
    pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
}
