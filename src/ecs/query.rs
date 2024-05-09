use std::{cell::RefCell, rc::Rc};

use super::{system::SystemParam, World};

/// Want to write systems as functions with parameters like this:
/// ```rust
/// fn system(query: Query<&Transform>) {
///     for transform in &query {
///        // transform is a &Transform
///        // do something with transform
///     }
/// }
/// ```
/// or, if you need mutable access to the components:
/// ```rust
/// fn system(query: Query<(&Mesh, &mut Transform)>) {
///     for (mesh, transform) in &mut query {
///         // mesh is a &Mesh
///         // transform is a &mut Transform
///         // do something with mesh and transform
///     }
/// }
pub struct Query<'q, D: QueryData> {
    refs: Vec<D::Item<'q>>,
}

impl<'q, D: QueryData> Query<'q, D> {
    pub fn new(world: &'q World) -> Self {
        let refs = D::fetch(world);
        Self { refs }
    }
}

impl<'q, D: QueryData> SystemParam for Query<'q, D> {
    type Item<'a> = Query<'a, D>;

    fn fetch(world: &World) -> Self::Item<'_> {
        Query::new(world)
    }
}

// implementation of IntoIterator for Query
impl<'a, D: QueryData> IntoIterator for Query<'a, D> {
    type Item = D::Item<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.refs.into_iter()
    }
}

pub trait QueryData: Sized {
    type Item<'a>;

    fn fetch(world: &World) -> Vec<Self::Item<'_>>;
}

// implement QueryData for a reference to a component type
impl<T: 'static> QueryData for &T {
    type Item<'a> = Rc<RefCell<T>>;

    fn fetch(world: &World) -> Vec<Self::Item<'_>> {
        world
            .borrow_component_vec::<T>()
            .unwrap()
            .iter() // filter out the None values
            .filter_map(|component| component.as_ref())
            .cloned()
            .collect()
    }
}

impl<T: 'static> QueryData for &mut T {
    type Item<'a> = Rc<RefCell<T>>;

    fn fetch(world: &World) -> Vec<Self::Item<'_>> {
        world
            .borrow_component_vec::<T>()
            .unwrap()
            .iter() // filter out the None values
            .filter_map(|component| component.as_ref())
            .cloned()
            .collect()
    }
}

impl<T: 'static> QueryData for (&T,) {
    type Item<'a> = (Rc<RefCell<T>>,);

    fn fetch(world: &World) -> Vec<Self::Item<'_>> {
        world
            .borrow_component_vec::<T>()
            .unwrap()
            .iter() // filter out the None values
            .filter_map(|component| component.as_ref())
            .cloned()
            .map(|component| (component,))
            .collect()
    }
}

impl<T: 'static, U: 'static> QueryData for (&T, &U) {
    type Item<'a> = (Rc<RefCell<T>>, Rc<RefCell<U>>);

    fn fetch(world: &World) -> Vec<Self::Item<'_>> {
        let component_vec1 = world.borrow_component_vec::<T>().unwrap();
        let component_vec2 = world.borrow_component_vec::<U>().unwrap();
        let mut components = Vec::new();
        for (component1, component2) in component_vec1.iter().zip(component_vec2.iter()) {
            if let (Some(component1), Some(component2)) = (component1.as_ref(), component2.as_ref())
            {
                components.push((component1.clone(), component2.clone()));
            }
        }
        components
    }
}

impl<T: 'static, U: 'static, V: 'static> QueryData for (&T, &U, &V) {
    type Item<'a> = (Rc<RefCell<T>>, Rc<RefCell<U>>, Rc<RefCell<V>>);

    fn fetch(world: &World) -> Vec<Self::Item<'_>> {
        let component_vec1 = world.borrow_component_vec::<T>().unwrap();
        let component_vec2 = world.borrow_component_vec::<U>().unwrap();
        let component_vec3 = world.borrow_component_vec::<V>().unwrap();
        let mut components = Vec::new();
        for ((component1, component2), component3) in component_vec1
            .iter()
            .zip(component_vec2.iter())
            .zip(component_vec3.iter())
        {
            if let (Some(component1), Some(component2), Some(component3)) = (
                component1.as_ref(),
                component2.as_ref(),
                component3.as_ref(),
            ) {
                components.push((component1.clone(), component2.clone(), component3.clone()));
            }
        }
        components
    }
}

impl<T: 'static, U: 'static, V: 'static, W: 'static> QueryData for (&T, &U, &V, &W) {
    type Item<'a> = (
        Rc<RefCell<T>>,
        Rc<RefCell<U>>,
        Rc<RefCell<V>>,
        Rc<RefCell<W>>,
    );

    fn fetch(world: &World) -> Vec<Self::Item<'_>> {
        let component_vec1 = world.borrow_component_vec::<T>().unwrap();
        let component_vec2 = world.borrow_component_vec::<U>().unwrap();
        let component_vec3 = world.borrow_component_vec::<V>().unwrap();
        let component_vec4 = world.borrow_component_vec::<W>().unwrap();
        let mut components = Vec::new();
        for (((component1, component2), component3), component4) in component_vec1
            .iter()
            .zip(component_vec2.iter())
            .zip(component_vec3.iter())
            .zip(component_vec4.iter())
        {
            if let (Some(component1), Some(component2), Some(component3), Some(component4)) = (
                component1.as_ref(),
                component2.as_ref(),
                component3.as_ref(),
                component4.as_ref(),
            ) {
                components.push((
                    component1.clone(),
                    component2.clone(),
                    component3.clone(),
                    component4.clone(),
                ));
            }
        }
        components
    }
}

#[test]
fn test_query() {
    let mut world = World::new();

    let entity = world.new_entity();
    world.add_component_to_entity(entity, 42i32);

    let entity2 = world.new_entity();
    world.add_component_to_entity(entity2, 100i32);
    world.add_component_to_entity(entity2, "hello".to_string());

    let query = Query::<&i32>::fetch(&world);
    for (i, component) in query.into_iter().enumerate() {
        let component = (*component).borrow();
        if i == 0 {
            assert_eq!(*component, 42);
        } else {
            assert_eq!(*component, 100);
        }
    }

    let query = Query::<(&i32, &String)>::fetch(&world);
    for (int_component, string_component) in query {
        let int_component = (*int_component).borrow();
        let string_component = (*string_component).borrow();
        assert_eq!(*int_component, 100);
        assert_eq!(*string_component, "hello");
    }
}
