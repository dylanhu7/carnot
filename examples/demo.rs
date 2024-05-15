use carnot::{
    builtins::primitives::ImplicitFromPrimitive,
    graphics::{
        camera::CameraTransform,
        implicit::{Implicit, ImplicitSphere},
        ray::Ray,
    },
    prelude::*,
};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    App::new()
        .with_title("Carnot Demo")
        .with_default_systems()
        .with_system(Startup, initialize_player)
        .with_system(Startup, spawn_scene)
        .with_system(Startup, init_grid)
        .with_system(Startup, spawn_targets)
        .with_system(Update, check_hit)
        .run();
}

fn initialize_player(mut query: Query<(&mut CameraTransform, &ActiveCamera)>) {
    for (transform, _) in &mut query {
        transform.0.w_axis = Vec4::new(0.0, 1.8, 0.0, 0.0);
    }
}

fn spawn_scene(world: &mut World) {
    // Dimensions of box
    let (x, y, z) = (10.0, 4.0, 14.0);

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

struct Target;

fn spawn_targets(world: &mut World) {
    for _ in 0..3 {
        let sphere = world.new_entity();
        let mesh = Primitive::spawn(Primitive::SPHERE);
        let implicit = Primitive::spawn_implicit(Primitive::SPHERE);
        let grid_index = world
            .get_resource_mut::<TargetGrid>()
            .unwrap()
            .find_and_occupy()
            .unwrap();
        let location = world
            .get_resource::<TargetGrid>()
            .unwrap()
            .location_at(&grid_index);
        world.add_component_to_entity::<Mesh>(sphere, mesh);
        world.add_component_to_entity::<ImplicitSphere>(sphere, implicit);
        world.add_component_to_entity::<Transform>(
            sphere,
            Transform::from(Mat4::from_scale_rotation_translation(
                Vec3::splat(0.3),
                Quat::IDENTITY,
                location,
            )),
        );
        world.add_component_to_entity::<GridIndex>(sphere, grid_index);
        world.add_component_to_entity::<Target>(sphere, Target);
    }
}

fn check_hit(
    camera: Query<(&CameraTransform, &ActiveCamera)>,
    mut targets: Query<(&mut Transform, &ImplicitSphere, &mut GridIndex, &Target)>,
    mut target_grid: ResMut<TargetGrid>,
    input: ResMut<InputState>,
) {
    if !input.clicked {
        return;
    }
    let camera_mat = camera.into_iter().next().unwrap().0 .0;
    let camera_pos = camera_mat.w_axis.truncate();
    let camera_look = -camera_mat.z_axis.truncate();

    for (target_transform, target_implicit, grid_index, _) in &mut targets {
        let ray = Ray::new(camera_pos, camera_look);
        let hit = target_implicit.intersect_world(&ray, target_transform);
        if hit.is_some() {
            let (new_location, new_index) = target_grid.move_target(grid_index).unwrap();
            target_transform.0 = Mat4::from_scale_rotation_translation(
                Vec3::splat(0.3),
                Quat::IDENTITY,
                new_location,
            );
            *grid_index = new_index;
        }
    }
}

fn init_grid(world: &mut World) {
    let grid = TargetGrid::new(1.7, 1.8, 5, 5, 0.0, 2.0, -4.0);
    world.add_resource::<TargetGrid>(grid);
}

struct TargetGrid {
    locations: Vec<Vec3>,
    occupied: Vec<bool>,
}

struct GridIndex(usize);

impl TargetGrid {
    fn new(
        extent_x: f32,
        extent_y: f32,
        cells_x: usize,
        cells_y: usize,
        center_x: f32,
        center_y: f32,
        center_z: f32,
    ) -> Self {
        let mut grid = Vec::new();
        let step_x = extent_x / cells_x as f32;
        let step_y = extent_y / cells_y as f32;

        for i in 0..cells_x {
            let x = center_x + ((i as f32 - cells_x as f32 / 2.0) + 0.5) * step_x;
            for j in 0..cells_y {
                let y = center_y + ((j as f32 - cells_y as f32 / 2.0) + 0.5) * step_y;
                grid.push(Vec3::new(x, y, center_z));
            }
        }

        let num_locations = grid.len();

        Self {
            locations: grid,
            occupied: vec![false; num_locations],
        }
    }

    fn find_empty(&self) -> Option<GridIndex> {
        let mut indices: Vec<usize> = (0..self.locations.len()).collect();
        indices.shuffle(&mut thread_rng());

        for i in indices {
            if !self.occupied[i] {
                return Some(GridIndex(i));
            }
        }

        None
    }

    fn find_and_occupy(&mut self) -> Option<GridIndex> {
        let index = self.find_empty()?;
        self.occupied[index.0] = true;
        Some(index)
    }

    fn location_at(&self, index: &GridIndex) -> Vec3 {
        self.locations[index.0]
    }

    fn move_target(&mut self, hit: &GridIndex) -> Option<(Vec3, GridIndex)> {
        let new_index = self.find_empty()?;
        let new_location = self.location_at(&new_index);
        self.occupied[new_index.0] = true;
        self.occupied[hit.0] = false;
        Some((new_location, new_index))
    }
}
