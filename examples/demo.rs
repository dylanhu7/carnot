use carnot::{
    builtins::{
        primitive::Primitive,
        systems::{camera_system, render_system, ActiveCamera},
    },
    graphics::{Mesh, PerspectiveCamera, Transform},
    App,
};
use glam::{Mat4, Vec3};

fn main() {
    let mut app = App::new("Carnot Demo");

    let cube = app.world.new_entity();
    app.world
        .add_component_to_entity::<Mesh>(cube, Primitive::spawn(Primitive::CUBE));
    app.world.add_component_to_entity::<Transform>(
        cube,
        Transform::from(Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0))),
    );

    let ground = app.world.new_entity();
    app.world
        .add_component_to_entity::<Mesh>(ground, Primitive::spawn(Primitive::PLANE));
    app.world.add_component_to_entity::<Transform>(
        ground,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(10.0, 1.0, 10.0),
            glam::Quat::IDENTITY,
            Vec3::new(0.0, -1.0, 0.0),
        )),
    );

    // let size = app.window.as_ref().unwrap().inner_size();
    // let (width, height) = (size.width, size.height);

    let camera = PerspectiveCamera::new(800 as f32 / 600 as f32, 103.0, 0.1, 100.0);
    let camera_transform = Transform::from(
        Mat4::look_to_rh(
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 0.0, -1.0),
            glam::Vec3::new(0.0, 1.0, 0.0),
        )
        .inverse(),
    );

    let camera_entity = app.world.new_entity();
    app.world
        .add_component_to_entity::<PerspectiveCamera>(camera_entity, camera);
    app.world
        .add_component_to_entity::<Transform>(camera_entity, camera_transform);
    app.world
        .add_component_to_entity::<ActiveCamera>(camera_entity, ActiveCamera);

    app.add_system(render_system);
    app.add_system(camera_system);

    app.run();
}
