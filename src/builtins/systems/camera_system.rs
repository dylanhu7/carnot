use winit::keyboard::Key;

use crate::{
    builtins::systems::ActiveCamera, ecs::World, graphics::Transform, input::InputState,
    render::Renderer,
};

pub fn camera_system(world: &mut World, _: &mut Renderer, input_state: &mut InputState) {
    const SPEED: f32 = 0.05;
    let mut transform_vec = world.borrow_component_vec_mut::<Transform>().unwrap();
    let mut active_camera_vec = world.borrow_component_vec_mut::<ActiveCamera>().unwrap();
    let transform = transform_vec
        .iter_mut()
        .zip(active_camera_vec.iter_mut())
        .filter(|(_, active)| active.is_some())
        .filter_map(|(transform, _)| transform.as_mut())
        .next()
        .expect("No active camera found");

    let mut dir = glam::Vec4::ZERO;

    if input_state.keys.contains(&Key::Character("w".into())) {
        dir += -transform.0.z_axis;
    }
    if input_state.keys.contains(&Key::Character("s".into())) {
        dir += transform.0.z_axis;
    }
    if input_state.keys.contains(&Key::Character("a".into())) {
        dir += -transform.0.x_axis;
    }
    if input_state.keys.contains(&Key::Character("d".into())) {
        dir += transform.0.x_axis;
    }

    transform.0.w_axis += dir * SPEED;

    const SENSITIVITY: f32 = 0.01;

    let (dx, dy) = (-input_state.mouse_delta.0, -input_state.mouse_delta.1);

    // first-person controls
    let horizontal_rotation = glam::Quat::from_axis_angle(glam::Vec3::Y, dx as f32 * SENSITIVITY);
    let vertical_rotation =
        glam::Quat::from_axis_angle(transform.0.x_axis.truncate(), dy as f32 * SENSITIVITY);
    let (scale, rot, trans) = transform.0.to_scale_rotation_translation();
    transform.0 = glam::Mat4::from_scale_rotation_translation(
        scale,
        horizontal_rotation * vertical_rotation * rot,
        trans,
    );

    // third-person controls
    // let horizontal_rotation = glam::Mat4::from_axis_angle(glam::Vec3::Y, dx as f32 * SENSITIVITY);
    // let vertical_rotation =
    //     glam::Mat4::from_axis_angle(transform.0.x_axis.truncate(), dy as f32 * SENSITIVITY);
    // transform.0 = horizontal_rotation * vertical_rotation * transform.0;

    input_state.mouse_delta = (0.0, 0.0);
}
