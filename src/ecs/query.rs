use std::cell::{Ref, RefMut};

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

impl<'q, D: QueryData> SystemParam for Query<'q, D> {
    type Item<'a> = Query<'a, D>;

    fn fetch(world: &World) -> Self::Item<'_> {
        Query {
            refs: D::fetch(world),
        }
    }
}

// implementation of IntoIterator for Query
impl<'a, 'q, D: QueryData> IntoIterator for &'a Query<'q, D> {
    type Item = D::Item<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        D::refs_to_sparse_iter(&self.refs)
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'a, 'q, D: QueryData> IntoIterator for &'a mut Query<'q, D> {
    type Item = D::Item<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        D::refs_to_sparse_iter_mut(&mut self.refs)
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

pub trait QueryData: Sized {
    type Item<'a>;
    type ItemVecRefs<'a>;

    fn fetch<'a, 'w: 'a>(world: &'w World) -> Self::ItemVecRefs<'a>;

    fn refs_to_sparse_iter<'a>(
        _refs: &'a Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        Vec::new().into_iter()
    }

    fn refs_to_sparse_iter_mut<'a>(
        refs: &'a mut Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        Self::refs_to_sparse_iter(refs)
    }
}

impl<T: 'static> QueryData for &T {
    type Item<'a> = &'a T;
    type ItemVecRefs<'a> = Ref<'a, Vec<Option<T>>>;

    fn fetch<'a, 'w: 'a>(world: &'w World) -> Self::ItemVecRefs<'a> {
        world.borrow_component_vec::<T>().unwrap()
    }

    fn refs_to_sparse_iter<'a>(
        refs: &'a Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        refs.iter().map(|component| component.as_ref())
    }
}

impl<T: 'static> QueryData for &mut T {
    type Item<'a> = &'a mut T;
    type ItemVecRefs<'a> = RefMut<'a, Vec<Option<T>>>;

    fn fetch<'a, 'w: 'a>(world: &'w World) -> Self::ItemVecRefs<'a> {
        world.borrow_component_vec_mut::<T>().unwrap()
    }

    fn refs_to_sparse_iter_mut<'a>(
        refs: &'a mut Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        refs.iter_mut().map(|component| component.as_mut())
    }
}

impl<D: QueryData> QueryData for (D,) {
    type Item<'a> = (D::Item<'a>,);
    type ItemVecRefs<'a> = (D::ItemVecRefs<'a>,);

    fn fetch<'a, 'w: 'a>(world: &'w World) -> Self::ItemVecRefs<'a> {
        (D::fetch(world),)
    }

    fn refs_to_sparse_iter<'a>(
        refs: &'a Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        let iter = D::refs_to_sparse_iter(&refs.0);
        iter.map(|item| item.map(|item| (item,)))
    }

    fn refs_to_sparse_iter_mut<'a>(
        refs: &'a mut Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        let iter = D::refs_to_sparse_iter_mut(&mut refs.0);
        iter.map(|item| item.map(|item| (item,)))
    }
}

impl<D1: QueryData, D2: QueryData> QueryData for (D1, D2) {
    type Item<'a> = (D1::Item<'a>, D2::Item<'a>);
    type ItemVecRefs<'a> = (D1::ItemVecRefs<'a>, D2::ItemVecRefs<'a>);

    fn fetch<'a, 'w: 'a>(world: &'w World) -> Self::ItemVecRefs<'a> {
        (D1::fetch(world), D2::fetch(world))
    }

    fn refs_to_sparse_iter<'a>(
        refs: &'a Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        let iter1 = D1::refs_to_sparse_iter(&refs.0);
        let iter2 = D2::refs_to_sparse_iter(&refs.1);
        iter1
            .zip(iter2)
            .map(|(item1, item2)| Some((item1?, item2?)))
    }

    fn refs_to_sparse_iter_mut<'a>(
        refs: &'a mut Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        let iter1 = D1::refs_to_sparse_iter_mut(&mut refs.0);
        let iter2 = D2::refs_to_sparse_iter_mut(&mut refs.1);
        iter1
            .zip(iter2)
            .map(|(item1, item2)| Some((item1?, item2?)))
    }
}

impl<D1: QueryData, D2: QueryData, D3: QueryData> QueryData for (D1, D2, D3) {
    type Item<'a> = (D1::Item<'a>, D2::Item<'a>, D3::Item<'a>);
    type ItemVecRefs<'a> = (
        D1::ItemVecRefs<'a>,
        D2::ItemVecRefs<'a>,
        D3::ItemVecRefs<'a>,
    );

    fn fetch<'a, 'w: 'a>(world: &'w World) -> Self::ItemVecRefs<'a> {
        (D1::fetch(world), D2::fetch(world), D3::fetch(world))
    }

    fn refs_to_sparse_iter<'a>(
        refs: &'a Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        let iter1 = D1::refs_to_sparse_iter(&refs.0);
        let iter2 = D2::refs_to_sparse_iter(&refs.1);
        let iter3 = D3::refs_to_sparse_iter(&refs.2);
        iter1
            .zip(iter2)
            .zip(iter3)
            .map(|((item1, item2), item3)| Some((item1?, item2?, item3?)))
    }

    fn refs_to_sparse_iter_mut<'a>(
        refs: &'a mut Self::ItemVecRefs<'_>,
    ) -> impl Iterator<Item = Option<Self::Item<'a>>> {
        let iter1 = D1::refs_to_sparse_iter_mut(&mut refs.0);
        let iter2 = D2::refs_to_sparse_iter_mut(&mut refs.1);
        let iter3 = D3::refs_to_sparse_iter_mut(&mut refs.2);
        iter1
            .zip(iter2)
            .zip(iter3)
            .map(|((item1, item2), item3)| Some((item1?, item2?, item3?)))
    }
}
