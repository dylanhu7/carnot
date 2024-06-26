# Carnot Development Log

## 2024-04-17

### Input handling and camera movement

#### Reading input events

I have implemented a simple input handling system that reads events from `winit` and updates an `InputState` struct to be read by systems. The `InputState` struct looks like:

```rust
pub struct InputState {
    pub keys: HashSet<Key>,
    pub mouse_position: PhysicalPosition<f64>,
    pub last_mouse_position: Option<PhysicalPosition<f64>>,
    pub mouse_delta: (f64, f64),
    pub mouse_wheel_delta: MouseScrollDelta,
}
```

Parsing events from `winit` is a bit tedious though:

```rust
match event {
    Event::DeviceEvent { event, .. } => match event {
        winit::event::DeviceEvent::MouseMotion { delta } => {
            self.input_state.mouse_delta = delta;
        }
        winit::event::DeviceEvent::MouseWheel { delta } => {
            self.input_state.mouse_wheel_delta = delta;
        }
        _ => {}
    },
    Event::WindowEvent { event, .. } => match event {
        WindowEvent::KeyboardInput { event, .. } => {
            if event.state == winit::event::ElementState::Pressed {
                self.input_state.keys.insert(event.logical_key);
            } else {
                self.input_state.keys.remove(&event.logical_key);
            }
        }
        WindowEvent::CursorMoved { position, .. } => {
            self.input_state.last_mouse_position =
                Some(self.input_state.mouse_position);
            self.input_state.mouse_position = position;
        }
    }
}
```

#### Camera movement

For now, I've decided to approach camera movement a bit differently than I normally would have. Since the camera is represented as an entity in our ECS system, it has a `Transform` component (a 4x4 matrix) that represents its position and orientation in the world. Consequently, the `PerspectiveCamera` struct only represents the camera's intrinsics (field of view, aspect ratio, near and far planes) and can produce its projection matrix.

##### Anatomy of a transform

At least in introductory computer graphics (to my knowledge), the camera is usually represented by storing its position in world space (eye), look direction or target point, and up vector. These vectors are modified as the camera moves and turns, and a view matrix is produced from them (for example, using `glm`'s `lookAt` function or `glam::look_at_rh`). The view matrix brings the world into the camera's view (it's a camera-to-world transformation), so the inverse of the camera's `Transform` matrix (which brings the camera into world space) is the view matrix. In fact, we generate the initial `Transform` for the camera by taking the inverse of a view matrix, as it's convenient to generate a view matrix using a position, look direction, and up vector using our linear algebra library `glam`.

What's interesting is that when using the `Transform` as our representation instead of eye/look/up, we can perform translations and rotations in a fairly intuitive way, relying on the structure of the matrix directly.

You might be familiar with 4x4 transformation matrices that look like this:

$$
\begin{bmatrix}
r_{00} & r_{01} & r_{02} & t_x \\
r_{10} & r_{11} & r_{12} & t_y \\
r_{20} & r_{21} & r_{22} & t_z \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

Here, $r$ is the rotation part of the matrix and $t$ is the translation part. The rotation part is an orthonormal 3x3 matrix that represents the rotation of the object, and the translation part is a vector that represents the position of the object.

The first three columns are the basis vectors of the object's local space, and the fourth column is the position of the object in world space. If the camera is in its canonical position (at the world space origin) and orientation (looking down the negative world space z-axis, the up direction the world space y-axis, and the right direction the world space x-axis), then the matrix would simply be the identity matrix:

$$
\begin{bmatrix}
1 & 0 & 0 & 0 \\
0 & 1 & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

If we translated the camera 3 units forward in the negative-z direction and then rotated it 90 degrees around the y-axis (so that it now looks down the positive x-axis), the matrix would look like this:

$$
\begin{bmatrix}
0 & 0 & -1 & 0 \\
0 & 1 & 0 & 0 \\
1 & 0 & 0 & -3 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

We can see that now the camera's local x-axis (its right vector) has changed from being aligned with the global x-axis to being aligned with the global z-axis, and the other local axes have also changed accordingly. The translation part of the matrix has moved the camera 3 units in the negative z-direction.

Note that the order we chose — translation first, then rotation — is important. If we had rotated the camera first, then the translation would have been applied in the rotated space, and the camera would have moved in the direction of the rotated z-axis.

##### Implementing camera movement

Thankfully, our linear algebra library provides direct access to the columns of the matrix. We can perform translations and rotations by modifying these columns directly.

To handle WASD input for camera movement, we can derive the cumulative translation vector by composing the columns and adding it to the translation part of the matrix.

```rust
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
```

Handling camera rotation based on mouse movement is a little more complicated, but still highly interpretable when using this representation!

Let's take a look at how first-person camera controls might be implemented:

```rust
const SENSITIVITY: f32 = 0.01;
let (dx, dy) = (-input_state.mouse_delta.0, -input_state.mouse_delta.1);

// first-person controls
let (scale, rot, trans) = transform.0.to_scale_rotation_translation();
let horizontal_rotation = glam::Quat::from_axis_angle(glam::Vec3::Y, dx as f32 * SENSITIVITY);
let mut vertical_rotation =
    glam::Quat::from_axis_angle(transform.0.x_axis.truncate(), dy as f32 * SENSITIVITY);
```

We construct the horizontal rotation quaternion to be around the global y-axis instead of the camera's local y-axis. You can imagine that if the player were looking straight down, the local y-axis would be parallel to the ground, and rotating about it would cause the camera to look up, which is not what we want.

The vertical rotation quaternion is constructed around the camera's local x-axis. This is because the local x-axis rotates along with the camera, so this will always produce the correct rotation regardless of which way the camera is currently facing.

However, there's one problem: the vertical rotation can cause the camera to flip upside down. Almost always in first-person games, if the player keeps moving their mouse up or down, the camera is not allowed to rotate past looking straight up or straight down.

To fix this, we can leverage that the matrix representation affords us direct access to the local coordinate frame of the camera. We can apply the intended vertical rotation to the camera's local negative z-axis (the direction the camera is looking) and check if that new local z-axis is in front of or behind the plane created by the local x-axis and the global y-axis. This check can be done by computing the dot product of the new local z-axis with the normal to that plane which is given by the cross product of the vectors which form that plane. If the dot product is negative, then the new local z-axis is behind the plane, and we should clamp the rotation.

```rust
let new_z = vertical_rotation * transform.0.z_axis.truncate();

if new_z.dot(transform.0.x_axis.truncate().cross(glam::Vec3::Y)) < 0.0 {
    let dir = if -new_z.y > 0.0 {
        glam::Vec3::Y
    } else {
        glam::Vec3::NEG_Y
    };
    vertical_rotation = glam::Quat::from_rotation_arc(-transform.0.z_axis.truncate(), dir)
}
```

Now, we can update the transform matrix.

```rust
let new_transform = glam::Mat4::from_scale_rotation_translation(
    scale,
    horizontal_rotation * vertical_rotation * rot,
    trans,
);

transform.0 = new_transform;
```

And now this camera can move and look around in a first-person perspective!

https://github.com/dylanhu7/carnot/assets/45575415/a4483cf1-4fc3-46f0-ac4d-74ef745d09f6

## 2024-04-16

### Simple systems

In order to actually use the ECS system, we need to define a way for users to define systems that operate on entities with certain components.

We add a field to the `App` struct to store systems:

```rust
type System = Box<dyn FnMut(&mut World, &mut Renderer)>;

pub struct App {
    ...
    systems: Vec<System>,
}
```

In most ECS implementations, systems are considered part of the "world", but if the `World` owned the systems and we wanted to pass `&mut World` to the systems, we would run into issues. For now, we will store the systems in the `App` struct and see if there are better ways to organize ownership and borrowing in the future.

For now, we have the `Renderer` as a parameter to the system functions, but in the future this should be removed once we have a better way to expose the renderer to the systems.

We also add a method to add systems to the `App`:

```rust
impl App {
    ...
    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }
    ...
}
```

We can now define systems and add them to the `App`:

```rust
fn render_system(world: &mut World, renderer: &mut Renderer) {
    let camera = world.get_resource::<PerspectiveCamera>().unwrap();
    let meshes = world.borrow_component_vec::<Mesh>().unwrap();
    let transforms = world.borrow_component_vec::<Transform>().unwrap();
    let models = meshes
        .iter()
        .zip(transforms.iter())
        .filter_map(|(mesh, transform)| Some((mesh.as_ref()?, transform.as_ref()?)));
    ...
}

fn main() {
    let mut app = App::new();
    app.add_system(Box::new(render_system));
    ...
}
```

And the systems are executed in the main loop:

```rust

WindowEvent::RedrawRequested => {
    self.world.update();
    for system in self.systems.iter_mut() {
        system(&mut self.world, &mut self.renderer);
    }
    self.window.window.request_redraw();
}
```

### Getting something on the screen

The builting render system [`render_system.rs`](src/builtins/systems/render_system.rs) is hacked together for now, and it renders a cube!

![Cube](https://github.com/dylanhu7/carnot/assets/45575415/a6743071-d07a-4fd4-a5f0-999efa797722)

However, this current `render_system` does not abstract away any of the `wgpu` boilerplate, and so we do not yet have an extensible and ergonomic rendering API. This will be a focus in the near future.

A valuable resource for researching how to do this is [nannou](https://github.com/nannou-org/nannou), which provides an abstraction layer over `wgpu`.

## 2024-04-03

### A naive ECS implementation

I have added the [`World`](src/ecs/world.rs) struct, implemented a resource system, and added a basic entity and component system.

#### Singleton-like resources

```rust
pub struct World {
    ...
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
  ...
  pub fn add_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_ref::<T>())
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_mut::<T>())
    }

    pub fn get_resource_or_insert_with<T: 'static, F: FnOnce() -> T>(&mut self, f: F) -> &T {
        self.resources
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(f()))
            .downcast_ref::<T>()
            .unwrap()
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.resources
            .remove(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast().ok().map(|resource| *resource))
    }

    pub fn contains_resource<T: 'static>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }
    ...
}
```

Here, we leverage Rust's `TypeId` to be able to set and get resources by their type directly. This is super neat, as it allows us to avoid the need for a string identifier or enum variant to identify resources. Internally, `TypeId` is a unique identifier for a type generated at compile time (though it is not guaranteed to be the same across different runs of the program).

By using a `HashMap<TypeId, Box<dyn Any>>`, we can store a single resource of any type in the world. For example, we might want to store the elapsed time in our game loop:

```rust
struct Time(f64);
let mut world = World::new();
world.add_resource::<Time>(Time(0.0));
```

We can then retrieve the resource later:

```rust
loop {
    let time = world.get_resource::<Time>().unwrap();
    println!("Current time: {}", time.0);
}
```

#### Entities and components

```rust
pub struct World {
    ...
    pub num_entities: usize,
    component_vecs: HashMap<TypeId, Box<dyn ComponentVec>>, // HashMap<TypeId, Box<dyn RefCell<Vec<Option<Box<dyn Any>>>>>>
}

// ECS implementations
impl World {
    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.num_entities;
        for component_vec in self.component_vecs.values_mut() {
            component_vec.push_none();
        }
        self.num_entities += 1;
        entity_id
    }

    pub fn add_component_to_entity<T: Any + 'static>(&mut self, entity: usize, component: T) {
        let component_vec = self
            .component_vecs
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                Box::new(RefCell::new(Vec::<Option<T>>::with_capacity(
                    self.num_entities,
                )))
            })
            .as_any_mut()
            .downcast_mut::<RefCell<Vec<Option<T>>>>()
            .expect("failed to downcast component vec to RefCell<Vec<Option<T>>>")
            .get_mut();
        while component_vec.len() < self.num_entities {
            component_vec.push(None);
        }
        component_vec[entity] = Some(component);
    }

    pub fn borrow_component_vec<T: 'static>(&self) -> Option<Ref<Vec<Option<T>>>> {
        self.component_vecs
            .get(&TypeId::of::<T>())
            .and_then(|component_vec| {
                component_vec
                    .as_any()
                    .downcast_ref::<RefCell<Vec<Option<T>>>>()
                    .map(|component_vec| component_vec.borrow())
            })
    }

    pub fn borrow_component_vec_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        self.component_vecs
            .get(&TypeId::of::<T>())
            .and_then(|component_vec| {
                component_vec
                    .as_any()
                    .downcast_ref::<RefCell<Vec<Option<T>>>>()
                    .map(|component_vec| component_vec.borrow_mut())
            })
    }
}
```

In this naive implementation, we represent entities as a simple `usize` identifier. Each new entity increments the `num_entities` counter, and we use this counter to ensure that all component vectors are the same length. A component vector is a `Vec<Option<T>>` where `T` is the component type. We use `Option<T>` to allow for sparse component storage, as not all entities will have all components. Of course, it's not actually sparse storage, as each `Option<T>` will take up the same amount of memory in the vector regardless of whether it is `Some<T>` or `None`.

We can think of this implementation as representing the entities and their components as a 2D array or matrix, where the rows represent entities and the columns represent components. Each entity is a column in the matrix, all instances of a particular component are a row, and so a particular component for a particular entity is the intersection of a row and column.

##### Querying

Querying for bundles of components that belong to the same entity can be done by iterating over the component vectors in parallel. For example, to get all entities with a `Transform` and `Mesh` component:

```rust
let transform_vec = world.borrow_component_vec::<Transform>().unwrap();
let mesh_vec = world.borrow_component_vec::<Mesh>().unwrap();

for (transform, mesh) in transform_vec.iter().zip(mesh_vec.iter()) {
    if let (Some(transform), Some(mesh)) = (transform, mesh) {
        // Do something with transform and mesh
    }
}
```

A bit more idiomatic and concise way to do this is to directly produce an iterator over the entities with both components:

```rust
let (meshes, transforms) = mesh_vec
        .iter()
        .zip(transforms_vec.iter())
        .filter_map(|(mesh, transform)| Some((mesh.as_ref()?, transform.as_ref()?)));
```

In the future, I'd like to provide querying functionality automatically, so that users can define queries and iterate over entities with those components without having to manually zip and filter the component vectors.

A sort of "endgame" ECS implementation in Rust would be to define traits and implementations on all functions that query the ECS system. This way, user can define systems as functions that have a query as a parameter, and the ECS system can automatically iterate over the entities that match the query.

Here's what that looks like in Bevy:

```rust
fn my_system(query: Query<(&Transform, &Mesh)>) {
    for (transform, mesh) in query.iter() {
        // Do something with transform and mesh
    }
}
```

It's pretty magical.

Unfortunately, it requires a ton of code and advanced type system features to implement this in Rust, including using `macro_rules!` generate code that can handle arbitrary queries and component bundles.

An implementation that is perhaps more readable is [kecs](https://github.com/kettle11/koi/tree/main/crates/kecs) by Ian Kettlewell.

## 2024-04-02

Taking a break from the render pipeline research, I have been looking into ECS implementations in Rust. It seems that ECS in Rust is actually quite a popular topic, with countless libraries and a solid amount of resources available. Some existing ECS libraries include:

- [bevy-ecs](https://crates.io/crates/bevy-ecs) (used in the [Bevy](https://bevyengine.org/) game engine)
- [hecs](https://crates.io/crates/hecs)
- [legion](https://crates.io/crates/legion) (used in the archived Amethyst game engine)
- [specs](https://crates.io/crates/specs)
- [gecs](https://crates.io/crates/gecs) (compile-time generated queries)

I have also found Brooks Builds' [YouTube series](https://www.youtube.com/playlist?list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs) very helpful in not only how to implement a basic ECS system in Rust, but also as a fantastic crash course in fundamental Rust language features and patterns.

In my initial implementation, I will follow [this tutorial](https://ianjk.com/ecs-in-rust/) from [Ian Kettlewell](https://ianjk.com/about/) but also bring in additional functionality such as resources and built-in iterators.

In the future, I hope to implement a much more advanced ECS system that can handle complex queries and define systems ergonoimcally. Bevy's ECS system is extremely ergonomic, using Rust functions as systems and function parameters to define queries. Ian Kettlewell's [kecs] crate might be a good reference for that, as it's not as feature-rich as Bevy's ECS but still provides a nice API for an archetypal ECS system.

## 2024-04-01

In researching modern render pipelines, I have decided to play around with Vulkan as there are more resources available covering best practices and common optimizations. I have started following the [Vulkan Tutorial](https://docs.vulkan.org/tutorial/latest/index.html) in C++. I am particularly interested in the parts of the tutorial that cover synchronization, as understanding how synchronization is naively implemented in Vulkan with semaphores and fences will help me understand how a render graph system can abstract away these details.

## 2024-03-27

I found the blog post from Godot very accessible, as it explains concepts without too much technical jargon so that someone who simply uses the engine, for example, might understand it. A few major takeaways include:

- The render graph is a directed acyclic graph (DAG) that represents the rendering pipeline.
- Command buffers are used to record commands for the GPU to execute.
- These commands are executed after submitting the command buffers to a queue.
- Operations in the queue are executed asynchronously, so synchronization is necessary to ensure that operations are executed in the correct order.

  > Operations that are submitted to queues are executed asynchronously. Therefore we have to use synchronization objects like semaphores to ensure a correct order of execution. Execution of the draw command buffer must be set up to wait on image acquisition to finish, otherwise it may occur that we start rendering to an image that is still being read for presentation on the screen. The vkQueuePresentKHR call in turn needs to wait for rendering to be finished, for which we’ll use a second semaphore that is signaled after rendering completes.
  >
  > \- [Khronos Vulkan Tutorial](https://docs.vulkan.org/tutorial/latest/01_Overview.html#_step_7_command_pools_and_command_buffers)

- A render pass is an object that holds references to textures that will be used as render targets. Only certain operations can be performed within a render pass.
- Since the execution of the commands is not guaranteed to match the order in which they were submitted, synchronization barriers must be manually inserted to ensure that results from previous commands are available as necessary before dependent commands are executed.

## 2024-03-26

### Outline

In this first entry, I will outline my existing knowledge, as well as my goals and motivations for this project.

I am coming in with a very introductory understanding of realtime graphics. In the past, I have implemented projects in OpenGL using vertex buffers and attributes, shaders, framebuffers, and textures. In these projects, the rendering pipeline simply consisted of looping through shapes, binding their vertex buffers and uniforms, and drawing them to the screen. It is my hope that this project will allow me to explore more advanced rendering techniques and architectures.

Currently, I am focusing on researching modern approaches to rendering and game engine architecture. In particular, I aim to research and implement:

- A render graph system
- A physically based rendering (PBR) pipeline
- An entity-component-system (ECS) architecture

I am also using this project as an opportunity to gain experience reading and writing idiomatic Rust.

### Render graph

At this point, I am still researching render graph systems to gain a high-level understanding of their use cases, components, and implementation details. Some resources I am using include:

- [FrameGraph: Extensible Rendering Architecture in Frostbite](https://www.gdcvault.com/play/1024612/FrameGraph-Extensible-Rendering-Architecture-in)
- [rend3](https://github.com/BVE-Reborn/rend3)
- [Ponies and Light: Rendergraphs and how to implement one](https://poniesandlight.co.uk/reflect/island_rendergraph_1/)
- [Ponies and Light: Vulkan Render-Queues and how they Sync](https://poniesandlight.co.uk/reflect/island_rendergraph_2/)
- [GPU synchronization in Godot 4.3 is getting a major upgrade](https://godotengine.org/article/rendering-acyclic-graph/)
