use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct World {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn update(&mut self) {}
}

#[test]
fn resources_test() {
    let mut world = World::new();
    world.add_resource::<String>("Hello, World!".to_string());
    world.add_resource::<u32>(42);
    world.add_resource::<f32>(std::f32::consts::PI);

    assert_eq!(
        *world.get_resource::<String>().unwrap(),
        "Hello, World!".to_string()
    );
    assert_eq!(*world.get_resource::<u32>().unwrap(), 42);
    assert_eq!(*world.get_resource::<f32>().unwrap(), std::f32::consts::PI);

    world.get_resource_mut::<String>().unwrap().push('!');
    assert_eq!(
        *world.get_resource::<String>().unwrap(),
        "Hello, World!!".to_string()
    );

    assert_eq!(
        *world.get_resource_or_insert_with::<String, _>(|| "Won't be inserted".to_string()),
        "Hello, World!!".to_string()
    );

    assert!(world.contains_resource::<String>());
    assert!(world.contains_resource::<u32>());
    assert!(world.contains_resource::<f32>());

    assert_eq!(
        world.remove_resource::<String>().unwrap(),
        "Hello, World!!".to_string()
    );
    assert!(world.remove_resource::<String>().is_none());
    assert!(!world.contains_resource::<String>());
}
