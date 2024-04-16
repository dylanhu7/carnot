use carnot::{
    builtins::{primitive::Primitive, systems::render_system},
    graphics::{Mesh, PerspectiveCamera, Transform},
    App,
};

#[tokio::main]
async fn main() {
    let mut app = App::new(800, 600, "Carnot Demo").await;

    let cube = app.world.new_entity();
    app.world
        .add_component_to_entity::<Mesh>(cube, Primitive::spawn(Primitive::CUBE));
    app.world
        .add_component_to_entity::<Transform>(cube, Transform::default());
    let camera = PerspectiveCamera::new(
        glam::Vec3::new(1.0, 2.0, 3.0),
        glam::Vec3::new(0.0, 0.0, 0.0),
        glam::Vec3::new(0.0, 1.0, 0.0),
        800.0 / 600.0,
        45.0,
        0.1,
        100.0,
    );
    app.world.add_resource::<PerspectiveCamera>(camera);

    app.add_system(Box::new(render_system));

    app.run();
}
