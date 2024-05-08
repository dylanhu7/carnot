use super::World;

pub trait SystemParam {
    type Item<'a>: SystemParam;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w>;
}

pub type SystemParamItem<'w, P> = <P as SystemParam>::Item<'w>;

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
