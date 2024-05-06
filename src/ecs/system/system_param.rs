use super::World;

pub trait SystemParam {
    type Item<'w>: SystemParam;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w>;
}

pub type SystemParamItem<'w, P> = <P as SystemParam>::Item<'w>;

impl SystemParam for () {
    type Item<'w> = ();

    fn fetch<'w>(_: &'w World) -> Self::Item<'w> {}
}

impl<P1: SystemParam> SystemParam for (P1,) {
    type Item<'w> = (P1::Item<'w>,);

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        (P1::fetch(world),)
    }
}

impl<P1: SystemParam, P2: SystemParam> SystemParam for (P1, P2) {
    type Item<'w> = (P1::Item<'w>, P2::Item<'w>);

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        (P1::fetch(world), P2::fetch(world))
    }
}
