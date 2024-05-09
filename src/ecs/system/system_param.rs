use std::{any::TypeId, marker::PhantomData};

use crate::ecs::resource::{Res, ResMut};

use super::World;

pub trait SystemParam {
    type Item<'a>: SystemParam;

    fn fetch(world: &World) -> Self::Item<'_>;
}

pub type SystemParamItem<'w, P> = <P as SystemParam>::Item<'w>;

impl SystemParam for &World {
    type Item<'a> = &'a World;

    fn fetch(world: &World) -> Self::Item<'_> {
        world
    }
}

impl<'res, T: 'static> SystemParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn fetch(world: &World) -> Self::Item<'_> {
        Res {
            value: world.resources.get(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }
}

impl<'res, T: 'static> SystemParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn fetch(world: &World) -> Self::Item<'_> {
        ResMut {
            value: world
                .resources
                .get(&TypeId::of::<T>())
                .unwrap()
                .borrow_mut(),
            _marker: PhantomData,
        }
    }
}

impl SystemParam for () {
    type Item<'a> = ();

    fn fetch(_: &World) -> Self::Item<'_> {}
}

impl<P1: SystemParam> SystemParam for (P1,) {
    type Item<'a> = (P1::Item<'a>,);

    fn fetch(world: &World) -> Self::Item<'_> {
        (P1::fetch(world),)
    }
}

impl<P1: SystemParam, P2: SystemParam> SystemParam for (P1, P2) {
    type Item<'a> = (P1::Item<'a>, P2::Item<'a>);

    fn fetch(world: &World) -> Self::Item<'_> {
        (P1::fetch(world), P2::fetch(world))
    }
}

impl<P1: SystemParam, P2: SystemParam, P3: SystemParam> SystemParam for (P1, P2, P3) {
    type Item<'a> = (P1::Item<'a>, P2::Item<'a>, P3::Item<'a>);

    fn fetch(world: &World) -> Self::Item<'_> {
        (P1::fetch(world), P2::fetch(world), P3::fetch(world))
    }
}
