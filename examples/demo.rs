use carnot::{
    builtins::{
        primitive::Primitive,
        systems::{render_system, ActiveCamera},
    },
    graphics::{Mesh, PerspectiveCamera, Transform},
    App,
};
use glam::Mat4;

#[tokio::main]
async fn main() {
    let mut app = App::new(800, 600, "Carnot Demo").await;

    let cube = app.world.new_entity();
    app.world
        .add_component_to_entity::<Mesh>(cube, Primitive::spawn(Primitive::CUBE));
    app.world
        .add_component_to_entity::<Transform>(cube, Transform::default());

    let camera = PerspectiveCamera::new(800.0 / 600.0, 45.0, 0.1, 100.0);
    let camera_transform = Transform::from(
        Mat4::look_at_rh(
            glam::Vec3::new(-1.0, 4.0, 3.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
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

    app.add_system(Box::new(render_system));

    app.run();
}
