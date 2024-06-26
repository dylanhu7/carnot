use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use super::component::ComponentVec;
use super::resource::{Res, ResMut};

#[derive(Default)]
pub struct World {
    pub num_entities: usize,
    component_vecs: HashMap<TypeId, Rc<RefCell<dyn ComponentVec>>>, // HashMap<TypeId, Rc<RefCell<Vec<Option<T>>>>>
    pub resources: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}

impl World {
    // ECS implementations
    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.num_entities;
        for component_vec in self.component_vecs.values_mut() {
            component_vec.borrow_mut().push_none();
        }
        self.num_entities += 1;
        entity_id
    }

    pub fn add_component_to_entity<T: Any + 'static>(&mut self, entity: usize, component: T) {
        let mut binding = self
            .component_vecs
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                Rc::new(RefCell::new(Vec::<Option<T>>::with_capacity(
                    self.num_entities,
                )))
            })
            .borrow_mut();
        let component_vec = binding
            .as_any_mut()
            .downcast_mut::<Vec<Option<T>>>()
            .expect("failed to downcast component vec to RefCell<Vec<Option<<T>>>");
        while component_vec.len() < self.num_entities {
            component_vec.push(None);
        }
        component_vec[entity] = Some(component);
    }

    pub fn borrow_component_vec<T: 'static>(&self) -> Option<Ref<Vec<Option<T>>>> {
        self.component_vecs
            .get(&TypeId::of::<T>())
            .map(|component_vec| {
                Ref::map(component_vec.borrow(), |component_vec| {
                    component_vec
                        .as_any()
                        .downcast_ref::<Vec<Option<T>>>()
                        .expect("failed to downcast component vec to Vec<Option<T>>")
                })
            })
    }

    pub fn borrow_component_vec_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        self.component_vecs
            .get(&TypeId::of::<T>())
            .map(|component_vec| {
                RefMut::map(component_vec.borrow_mut(), |component_vec| {
                    component_vec
                        .as_any_mut()
                        .downcast_mut::<Vec<Option<T>>>()
                        .expect("failed to downcast component vec to Vec<Option<T>>")
                })
            })
    }
}

// Resource implementations
impl World {
    pub fn add_resource<T: 'static>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(resource)));
    }

    pub fn get_resource<T: 'static>(&self) -> Option<Res<T>> {
        self.resources
            .get(&TypeId::of::<T>())
            .map(|resource| {
                Ref::map(resource.borrow(), |resource| {
                    resource
                        .downcast_ref::<T>()
                        .expect("failed to downcast resource to T")
                })
            })
            .map(|borrowed| Res { value: borrowed })
    }

    pub fn get_resource_mut<T: 'static>(&self) -> Option<ResMut<T>> {
        self.resources
            .get(&TypeId::of::<T>())
            .map(|resource| {
                RefMut::map(resource.borrow_mut(), |resource| {
                    resource
                        .downcast_mut::<T>()
                        .expect("failed to downcast resource to T")
                })
            })
            .map(|borrowed| ResMut { value: borrowed })
    }

    // pub fn get_resource<T: 'static>(&self) -> Option<&T> {
    //     self.resources
    //         .get(&TypeId::of::<T>())
    //         .and_then(|resource| resource.downcast_ref::<T>())
    // }

    // pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
    //     self.resources
    //         .get_mut(&TypeId::of::<T>())
    //         .and_then(|resource| resource.downcast_mut::<T>())
    // }

    // pub fn get_resource_or_insert<T: 'static>(&mut self, resource: T) -> &T {
    //     self.resources
    //         .entry(TypeId::of::<T>())
    //         .or_insert_with(|| Box::new(resource))
    //         .downcast_ref::<T>()
    //         .unwrap()
    // }

    // pub fn get_resource_or_insert_with<T: 'static, F: FnOnce() -> T>(&mut self, f: F) -> &T {
    //     self.resources
    //         .entry(TypeId::of::<T>())
    //         .or_insert_with(|| Box::new(f()))
    //         .downcast_ref::<T>()
    //         .unwrap()
    // }

    // pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
    //     self.resources
    //         .remove(&TypeId::of::<T>())
    //         .and_then(|resource| resource.downcast().ok().map(|resource| *resource))
    // }

    // pub fn contains_resource<T: 'static>(&self) -> bool {
    //     self.resources.contains_key(&TypeId::of::<T>())
    // }
}

// #[test]
// fn ecs_test() {
//     let mut world = World::new();
//     let entity1 = world.new_entity();
//     assert_eq!(entity1, 0);
//     assert!(world.borrow_component_vec_mut::<String>().is_none());
//     let entity2 = world.new_entity();
//     assert_eq!(entity2, 1);
//     assert!(world.borrow_component_vec_mut::<String>().is_none());

//     world.add_component_to_entity::<String>(entity1, "Hello, World!".to_string());
//     assert_eq!(world.borrow_component_vec_mut::<String>().unwrap().len(), 2);
//     world.add_component_to_entity(entity1, 42);
//     assert_eq!(world.borrow_component_vec_mut::<i32>().unwrap().len(), 2);
//     world.add_component_to_entity(entity2, 42);
//     assert_eq!(world.borrow_component_vec_mut::<String>().unwrap().len(), 2);

//     assert_eq!(
//         world.borrow_component_vec_mut::<String>().unwrap()[entity1]
//             .as_ref()
//             .unwrap(),
//         "Hello, World!"
//     );
//     assert_eq!(
//         world.borrow_component_vec_mut::<i32>().unwrap()[entity1]
//             .as_ref()
//             .unwrap(),
//         &42
//     );
//     assert_eq!(
//         world.borrow_component_vec_mut::<i32>().unwrap()[entity2]
//             .as_ref()
//             .unwrap(),
//         &42
//     );

//     assert_eq!(world.borrow_component_vec_mut::<i32>().unwrap().len(), 2);
// }
// #[test]
// fn resources_test() {
//     let mut world = World::new();
//     world.add_resource::<String>("Hello, World!".to_string());
//     world.add_resource::<u32>(42);
//     world.add_resource::<f32>(std::f32::consts::PI);

//     #[derive(Debug, PartialEq)]
//     struct TimeTest(f64);
//     world.add_resource(TimeTest(123.0));
//     assert_eq!(*world.get_resource::<TimeTest>().unwrap(), TimeTest(123.0));

//     assert_eq!(
//         *world.get_resource::<String>().unwrap(),
//         "Hello, World!".to_string()
//     );
//     assert_eq!(*world.get_resource::<u32>().unwrap(), 42);
//     assert_eq!(*world.get_resource::<f32>().unwrap(), std::f32::consts::PI);

//     world.get_resource_mut::<String>().unwrap().push('!');
//     assert_eq!(
//         *world.get_resource::<String>().unwrap(),
//         "Hello, World!!".to_string()
//     );

//     assert_eq!(
//         *world.get_resource_or_insert::<String>("Won't be inserted".to_string()),
//         "Hello, World!!".to_string()
//     );

//     assert_eq!(
//         *world.get_resource_or_insert_with::<String, _>(|| "Won't be inserted".to_string()),
//         "Hello, World!!".to_string()
//     );

//     assert!(world.contains_resource::<String>());
//     assert!(world.contains_resource::<u32>());
//     assert!(world.contains_resource::<f32>());

//     assert_eq!(
//         world.remove_resource::<String>().unwrap(),
//         "Hello, World!!".to_string()
//     );
//     assert!(world.remove_resource::<String>().is_none());
//     assert!(!world.contains_resource::<String>());
// }
