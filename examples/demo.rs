use carnot::prelude::*;

fn main() {
    App::new()
        .with_title("Carnot Demo")
        .with_default_systems()
        .with_system(Startup, spawn_ground)
        .with_system(Startup, spawn_cube)
        .run();
}

fn spawn_ground(world: &mut World) {
    let ground = world.new_entity();
    world.add_component_to_entity::<Mesh>(ground, Primitive::spawn(Primitive::PLANE));
    world.add_component_to_entity::<Transform>(
        ground,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(10.0, 1.0, 10.0),
            glam::Quat::IDENTITY,
            Vec3::new(0.0, -1.0, 0.0),
        )),
    );
}

fn spawn_cube(world: &mut World) {
    let cube = world.new_entity();
    world.add_component_to_entity::<Mesh>(cube, Primitive::spawn(Primitive::CUBE));
    world.add_component_to_entity::<Transform>(
        cube,
        Transform::from(Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0))),
    );
}
