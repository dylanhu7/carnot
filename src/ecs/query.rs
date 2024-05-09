use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    cell::{Ref, RefCell},
    marker::PhantomData,
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

    fn fetch<'w>(world: &'w World) -> Self::Item<'_> {
        Query::new(world)
    }
}

// implementation of IntoIterator for Query
impl<'a, D: QueryData> IntoIterator for &'a Query<'a, D> {
    type Item = &'a D::IterItem<'a>;
    type IntoIter = std::iter::Map<
        std::slice::Iter<'a, D::Item<'a>>,
        fn(&'a D::Item<'a>) -> &'a D::IterItem<'a>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.refs
            .iter()
            .map(|component| D::item_to_iter_item(component))
        // todo!()
    }
}

pub trait QueryData: Sized {
    type Item<'a>;
    type IterItem<'a>;

    fn fetch<'w>(world: &'w World) -> Vec<Self::Item<'w>>;

    fn item_to_iter_item<'a>(item: &'a Self::Item<'a>) -> &'a Self::IterItem<'a>;
}

// implement QueryData for a reference to a component type
impl<T: 'static> QueryData for &T {
    type Item<'a> = Rc<T>;
    type IterItem<'a> = T;

    fn fetch<'w>(world: &'w World) -> Vec<Self::Item<'w>> {
        world
            .borrow_component_vec::<T>()
            .unwrap()
            .iter() // filter out the None values
            .filter_map(|component| component.as_ref())
            .cloned()
            .collect()
    }

    fn item_to_iter_item<'a>(item: &'a Self::Item<'_>) -> &'a Self::IterItem<'a> {
        let brw = item.as_ref();
        brw
        // item.borrow().clone()
    }
}

#[test]
fn test_query() {
    let mut world = World::new();
    let entity = world.new_entity();
    world.add_component_to_entity(entity, 42i32);
    let query = Query::<&i32>::fetch(&world);
    for &component in &query {
        assert_eq!(component, 42);
    }
}

// struct QueryIter<'q, D: QueryData> {
//     query: &'q Query<'q, D>,
//     index: usize,
// }

// impl<'q, D: QueryData> QueryIter<'q, D> {
//     fn new(query: &'q Query<'q, D>) -> Self {
//         // for sparse_ref in &query.sparse_refs {

//         // }
//         Self { query, index: 0 }
//     }
// }

// impl<'d, D: QueryData> Iterator for QueryIter<'d, D> {
//     type Item = D::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         D::next(self)
//     }
// }

// trait SparseQueryData {
//     type Item<'a>: QueryIter<'a>;

//     fn to_dense(&self) ->
// }

// impl<T: 'static> SparseQueryData for Ref<'_, Vec<Option<T>>> {
//     type Item<'a> = Ref<'a, Vec<Option<T>>>;

// }
