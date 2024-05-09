use std::{
    any::{Any, TypeId},
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
    sparse_refs: Vec<Ref<'q, Vec<Option<Rc<RefCell<dyn Any>>>>>>,
    type_ids: Vec<TypeId>,
    marker: PhantomData<D>,
}

impl<'q, D: QueryData> Query<'q, D> {
    pub fn new(world: &'q World) -> Self {
        let (sparse_refs, type_ids) = D::get_sparse_refs(world);
        Self {
            sparse_refs,
            type_ids,
            marker: PhantomData,
        }
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
    type Item = D::Item<'a>;
    type IntoIter = QueryIter<'a, D>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter::new(self)
    }
}

pub trait QueryData: Sized {
    type Item<'a>
    where
        Self: 'a;

    fn get_sparse_refs(world: &World)
        -> (Vec<Ref<Vec<Option<Rc<RefCell<dyn Any>>>>>>, Vec<TypeId>);

    fn next<'q>(query: &mut QueryIter<'q, Self>) -> Option<Self::Item<'q>>;
}

// implement QueryData for a reference to a component type
impl<T: 'static> QueryData for &T {
    type Item<'a> = Ref<'a, T> where Self: 'a;

    fn get_sparse_refs(
        world: &World,
    ) -> (Vec<Ref<Vec<Option<Rc<RefCell<dyn Any>>>>>>, Vec<TypeId>) {
        let sparse_refs = world.borrow_component_vec_as_any::<T>().unwrap();
        let type_id = TypeId::of::<T>();
        (vec![sparse_refs], vec![type_id])
    }

    fn next<'q>(iter: &mut QueryIter<'q, Self>) -> Option<Self::Item<'q>> {
        let sparse_refs = iter.query.sparse_refs;
        let type_ids = iter.query.type_ids;
        let index = iter.index;

        if index >= sparse_refs.first().unwrap().len() {
            return None;
        }

        let mut sparse_refs_iter = sparse_refs.iter();
        let mut type_ids_iter = type_ids.iter();

        let sparse_ref = sparse_refs_iter.next().unwrap();
        let type_id = type_ids_iter.next().unwrap();

        let component = sparse_ref[index]
            .unwrap()
            .as
            .unwrap()
            .borrow();

        iter.index += 1;

        Some(component)
    }
}

struct QueryIter<'q, D: QueryData> {
    query: &'q Query<'q, D>,
    index: usize,
}

impl<'q, D: QueryData> QueryIter<'q, D> {
    fn new(query: &'q Query<'q, D>) -> Self {
        // for sparse_ref in &query.sparse_refs {

        // }
        Self { query, index: 0 }
    }
}

impl<'d, D: QueryData> Iterator for QueryIter<'d, D> {
    type Item = D::Item;

    fn next(&mut self) -> Option<Self::Item> {
        D::next(self)
    }
}

// trait SparseQueryData {
//     type Item<'a>: QueryIter<'a>;

//     fn to_dense(&self) ->
// }

// impl<T: 'static> SparseQueryData for Ref<'_, Vec<Option<T>>> {
//     type Item<'a> = Ref<'a, Vec<Option<T>>>;

// }
