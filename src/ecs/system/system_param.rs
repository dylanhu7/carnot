use crate::ecs::resource::{Res, ResMut};

use super::World;

pub trait SystemOrWorldParam {}

impl SystemOrWorldParam for &mut World {}
impl<SP: SystemParam> SystemOrWorldParam for SP {}

pub trait SystemParam {
    type Item<'a>: SystemParam;

    fn fetch(world: &World) -> Self::Item<'_>;
}

pub type SystemParamItem<'w, P> = <P as SystemParam>::Item<'w>;

impl<'res, T: 'static> SystemParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn fetch(world: &World) -> Self::Item<'_> {
        world.get_resource::<T>().unwrap()
    }
}

impl<'res, T: 'static> SystemParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn fetch(world: &World) -> Self::Item<'_> {
        world.get_resource_mut::<T>().unwrap()
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

impl<P1: SystemParam, P2: SystemParam, P3: SystemParam, P4: SystemParam> SystemParam
    for (P1, P2, P3, P4)
{
    type Item<'a> = (P1::Item<'a>, P2::Item<'a>, P3::Item<'a>, P4::Item<'a>);

    fn fetch(world: &World) -> Self::Item<'_> {
        (
            P1::fetch(world),
            P2::fetch(world),
            P3::fetch(world),
            P4::fetch(world),
        )
    }
}
