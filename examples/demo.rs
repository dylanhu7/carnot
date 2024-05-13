use carnot::prelude::*;

fn main() {
    App::new()
        .with_title("Carnot Demo")
        .with_default_systems()
        .with_system(Startup, initialize_player)
        .with_system(Startup, spawn_scene)
        .with_system(Startup, spawn_cube)
        .with_system(Startup, spawn_sphere)
        .run();
}

fn initialize_player(mut query: Query<(&mut Transform, &ActiveCamera)>) {
    for (transform, _) in &mut query {
        transform.0.w_axis = Vec4::new(0.0, 2.0, 0.0, 1.0);
    }
}

fn spawn_scene(world: &mut World) {
    // Dimensions of box
    let (x, y, z) = (16.0, 8.0, 12.0);

    let floor = world.new_entity();
    world.add_component_to_entity::<Mesh>(floor, Primitive::spawn(Primitive::PLANE));
    world.add_component_to_entity::<Transform>(
        floor,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(x, 1.0, z),
            glam::Quat::IDENTITY,
            Vec3::new(0.0, 0.0, 0.0),
        )),
    );

    let ceiling = world.new_entity();
    world.add_component_to_entity::<Mesh>(ceiling, Primitive::spawn(Primitive::PLANE));
    world.add_component_to_entity::<Transform>(
        ceiling,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(x, 1.0, z),
            glam::Quat::from_rotation_x(std::f32::consts::PI),
            Vec3::new(0.0, y, 0.0),
        )),
    );

    let left_wall = world.new_entity();
    world.add_component_to_entity::<Mesh>(left_wall, Primitive::spawn(Primitive::PLANE));
    world.add_component_to_entity::<Transform>(
        left_wall,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(y, 1.0, z),
            glam::Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
            Vec3::new(-x / 2.0, y / 2.0, 0.0),
        )),
    );

    let right_wall = world.new_entity();
    world.add_component_to_entity::<Mesh>(right_wall, Primitive::spawn(Primitive::PLANE));
    world.add_component_to_entity::<Transform>(
        right_wall,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(y, 1.0, z),
            glam::Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            Vec3::new(x / 2.0, y / 2.0, 0.0),
        )),
    );

    let back_wall = world.new_entity();
    world.add_component_to_entity::<Mesh>(back_wall, Primitive::spawn(Primitive::PLANE));
    world.add_component_to_entity::<Transform>(
        back_wall,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(x, 1.0, y),
            glam::Quat::from_rotation_x(-std::f32::consts::PI / 2.0),
            Vec3::new(0.0, y / 2.0, z / 2.0),
        )),
    );

    let front_wall = world.new_entity();
    world.add_component_to_entity::<Mesh>(front_wall, Primitive::spawn(Primitive::PLANE));
    world.add_component_to_entity::<Transform>(
        front_wall,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::new(x, 1.0, y),
            glam::Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            Vec3::new(0.0, y / 2.0, -z / 2.0),
        )),
    );
}

fn spawn_cube(world: &mut World) {
    let cube = world.new_entity();
    world.add_component_to_entity::<Mesh>(cube, Primitive::spawn(Primitive::CUBE));
    world.add_component_to_entity::<Transform>(
        cube,
        Transform::from(Mat4::from_translation(Vec3::new(0.0, 2.0, -2.0))),
    );
}

fn spawn_sphere(world: &mut World) {
    let sphere = world.new_entity();
    world.add_component_to_entity::<Mesh>(sphere, Primitive::spawn(Primitive::SPHERE));
    world.add_component_to_entity::<Transform>(
        sphere,
        Transform::from(Mat4::from_scale_rotation_translation(
            Vec3::splat(0.5),
            Quat::IDENTITY,
            Vec3::new(0.0, 2.0, 2.0),
        )),
    );
}
