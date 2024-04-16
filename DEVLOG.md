# Carnot Development Log

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

```

```

```

```

```

```
