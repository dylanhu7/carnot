use std::marker::PhantomData;

use super::{
    system::{SystemParam, SystemParamFetch},
    World,
};

pub struct Query<'w, Q: QueryItem> {
    world: &'w World,
    marker: PhantomData<Q>,
}

impl<'w, Q: QueryItem> SystemParam for Query<'w, Q> {
    type Fetch = QueryFetch<'new, Q>;
}

pub struct QueryFetch<'w, Q: QueryItem> {
    marker: PhantomData<(&'w World, Q)>,
}

pub trait QueryItem: 'static {}

impl<'w, Q: QueryItem> SystemParamFetch<'w> for QueryFetch<'w, Q> {
    type Item = Query<'w, Q>;
    fn fetch(world: &'w World) -> Self::Item {
        Query {
            world,
            marker: PhantomData,
        }
    }
}

impl QueryItem for () {}
impl<'a, P1> QueryItem for (&'a P1,) where P1: QueryItem {}
