use glam::Mat4;
use winit::keyboard::Key;

use crate::{
    builtins::systems::ActiveCamera,
    ecs::{query::Query, resource::ResMut, World},
    graphics::{camera::CameraTransform, PerspectiveCamera, Transform},
    input::InputState,
};

pub fn init_camera_system(world: &mut World) {
    let camera = PerspectiveCamera::new(90.0, 800_f32 / 600_f32, 0.1, 100.0);
    let camera_transform = CameraTransform::from(&Transform::from(
        Mat4::look_to_rh(
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 0.0, -1.0),
            glam::Vec3::new(0.0, 1.0, 0.0),
        )
        .inverse(),
    ));

    let camera_entity = world.new_entity();
    world.add_component_to_entity::<PerspectiveCamera>(camera_entity, camera);
    world.add_component_to_entity::<CameraTransform>(camera_entity, camera_transform);
    world.add_component_to_entity::<ActiveCamera>(camera_entity, ActiveCamera);
}

pub fn update_camera_system(
    mut input_state: ResMut<InputState>,
    mut camera: Query<(&mut CameraTransform, &ActiveCamera)>,
) {
    let (transform, _) = (&mut camera)
        .into_iter()
        .next()
        .expect("No active camera found");

    const SPEED: f32 = 0.05;
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

    dir = dir.normalize_or_zero();

    transform.0.w_axis += dir * SPEED;

    const SENSITIVITY: f32 = 0.001;
    let (dx, dy) = (-input_state.mouse_delta.0, -input_state.mouse_delta.1);

    // first-person controls
    let (scale, rot, trans) = transform.0.to_scale_rotation_translation();
    let horizontal_rotation = glam::Quat::from_axis_angle(glam::Vec3::Y, dx as f32 * SENSITIVITY);
    let mut vertical_rotation =
        glam::Quat::from_axis_angle(transform.0.x_axis.truncate(), dy as f32 * SENSITIVITY);
    let new_z = vertical_rotation * transform.0.z_axis.truncate();

    if new_z.dot(transform.0.x_axis.truncate().cross(glam::Vec3::Y)) < 0.0 {
        let dir = if -new_z.y > 0.0 {
            glam::Vec3::Y
        } else {
            glam::Vec3::NEG_Y
        };
        vertical_rotation = glam::Quat::from_rotation_arc(-transform.0.z_axis.truncate(), dir)
    }

    let new_transform = glam::Mat4::from_scale_rotation_translation(
        scale,
        horizontal_rotation * vertical_rotation * rot,
        trans,
    );

    transform.0 = new_transform;

    // third-person controls
    // let horizontal_rotation = glam::Mat4::from_axis_angle(glam::Vec3::Y, dx as f32 * SENSITIVITY);
    // let vertical_rotation =
    //     glam::Mat4::from_axis_angle(transform.0.x_axis.truncate(), dy as f32 * SENSITIVITY);
    // transform.0 = horizontal_rotation * vertical_rotation * transform.0;

    input_state.mouse_delta = (0.0, 0.0);
}
