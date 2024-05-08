use std::{
    any::Any,
    cell::{Ref, RefCell},
    marker::PhantomData,
    ops::Deref,
    path::Iter,
};

use super::{component, system::SystemParam, World};

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
///     for mesh, transform in &mut query {
///         // mesh is a &Mesh
///         // transform is a &mut Transform
///         // do something with mesh and transform
///     }
/// }
pub struct Query<'w, D: QueryData> {
    world: &'w World,
    // dense:
    marker: PhantomData<D>,
}

impl<'w, D: QueryData> Query<'w, D> {
    pub fn new(world: &'w World) -> Self {
        let sparse = D::fetch_sparse(world);
        Self {
            world,
            marker: PhantomData,
        }
    }
}

impl<D: QueryData + 'static> SystemParam for Query<'_, D> {
    type Item<'a> = Query<'a, D>;

    fn fetch(world: &World) -> Self::Item<'_> {
        Query::new(world)
    }
}

pub trait QueryData {
    type Item;
    // type SparseItem<'b>;
    type Iter<'b>: Iterator<Item = Option<&'b Self::Item>>;

    fn fetch_sparse(world: &World) -> Self::Iter<'_>;
}

// implement QueryData for a reference to a component type
impl<T: 'static> QueryData for &T {
    type Item = T;
    // type SparseItem<'a> = Option<&'a Self::Item>;
    type Iter<'c> = QueryIter<'c, Self>;

    fn fetch_sparse(world: &World) -> Self::Iter<'_> {
        let sparse = world.borrow_component_vec::<T>().unwrap();
        QueryIter::new(sparse)
    }
}

struct QueryIter<'a, D: QueryData> {
    sparse: Ref<'a, Vec<Option<D::Item>>>,
    index: usize,
}

impl<'a, D: QueryData> QueryIter<'a, D> {
    fn new(sparse: Ref<'a, Vec<Option<D::Item>>>) -> Self {
        Self { sparse, index: 0 }
    }
}

impl<'a, D: QueryData> Iterator for QueryIter<'a, D> {
    type Item = Option<&'a D::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.sparse.get(self.index)?;
        self.index += 1;
        let item = item.as_ref();
        Some(item)
    }
}

// trait SparseQueryData {
//     type Item<'a>: QueryIter<'a>;

//     fn to_dense(&self) ->
// }

// impl<T: 'static> SparseQueryData for Ref<'_, Vec<Option<T>>> {
//     type Item<'a> = Ref<'a, Vec<Option<T>>>;

// }
