use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

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
pub struct Query<'a, D: QueryData> {
    refs: D::ItemVecRefs<'a>,
}

impl<'a, D: QueryData> Query<'a, D> {
    pub fn new(world: &'a World) -> Self {
        let refs = D::fetch(world);
        Self { refs }
    }
}

pub struct QueryIter<'a, 'q, D: QueryData> {
    query: &'a Query<'q, D>,
    index: usize,
}

impl<'a, 'q, D: QueryData> Iterator for QueryIter<'a, 'q, D> {
    type Item = D::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        D::next(&self.query.refs, self.index).map(|component| {
            self.index += 1;
            component
        })
    }
}

impl<'q, D: QueryData> SystemParam for Query<'q, D> {
    type Item<'a> = Query<'a, D>;

    fn fetch(world: &World) -> Self::Item<'_> {
        Query::<D>::new(world)
    }
}

// implementation of IntoIterator for Query
impl<'a, 'q, D: QueryData> IntoIterator for &'a Query<'q, D> {
    type Item = D::Item<'a>;
    type IntoIter = QueryIter<'a, 'q, D>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter {
            query: self,
            index: 0,
        }
    }
}

pub trait QueryData: Sized {
    type Item<'a>: 'a;
    type ItemVecRefs<'a>;

    fn fetch(world: &World) -> Self::ItemVecRefs<'_>;

    fn next<'a>(vec_refs: &'a Self::ItemVecRefs<'_>, index: usize) -> Option<Self::Item<'a>>;
}

// implement QueryData for a reference to a component type
impl<T: 'static> QueryData for &T {
    type Item<'a> = &'a T;
    type ItemVecRefs<'a> = Ref<'a, Vec<Option<T>>>;

    fn fetch(world: &World) -> Self::ItemVecRefs<'_> {
        world.borrow_component_vec::<T>().unwrap()
    }

    fn next<'a>(vec_refs: &'a Self::ItemVecRefs<'_>, index: usize) -> Option<Self::Item<'a>> {
        vec_refs.get(index).and_then(|component| component.as_ref())
    }
}

impl<D: QueryData> QueryData for (D,) {
    type Item<'a> = (D::Item<'a>,);
    type ItemVecRefs<'a> = (D::ItemVecRefs<'a>,);

    fn fetch(world: &World) -> Self::ItemVecRefs<'_> {
        (D::fetch(world),)
    }

    fn next<'a>(vec_refs: &'a Self::ItemVecRefs<'_>, index: usize) -> Option<Self::Item<'a>> {
        D::next(&vec_refs.0, index).map(|item| (item,))
    }
}

impl<D1: QueryData, D2: QueryData> QueryData for (D1, D2) {
    type Item<'a> = (D1::Item<'a>, D2::Item<'a>);
    type ItemVecRefs<'a> = (D1::ItemVecRefs<'a>, D2::ItemVecRefs<'a>);

    fn fetch(world: &World) -> Self::ItemVecRefs<'_> {
        (D1::fetch(world), D2::fetch(world))
    }

    fn next<'a>(vec_refs: &'a Self::ItemVecRefs<'_>, index: usize) -> Option<Self::Item<'a>> {
        Some((D1::next(&vec_refs.0, index)?, D2::next(&vec_refs.1, index)?))
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
        if i == 0 {
            assert_eq!(*component, 42);
        } else {
            assert_eq!(*component, 100);
        }
    }

    let query = Query::<(&i32, &String)>::fetch(&world);
    for (int_component, string_component) in &query {
        assert_eq!(*int_component, 100);
        assert_eq!(*string_component, "hello");
    }
}
