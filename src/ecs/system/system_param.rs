use super::World;

pub trait SystemParam: Sized {
    type Fetch: for<'w> SystemParamFetch<'w>;
}

pub trait SystemParamFetch<'w> {
    type Item: SystemParam<Fetch = Self>;
    fn fetch(world: &'w World) -> Self::Item;
}

pub type SystemParamItem<'w, P> = <<P as SystemParam>::Fetch as SystemParamFetch<'w>>::Item;

impl SystemParam for () {
    type Fetch = ();
}
impl<'w> SystemParamFetch<'w> for () {
    type Item = ();
    fn fetch(_: &'w World) -> Self::Item {}
}

impl<P1> SystemParam for (P1,)
where
    P1: SystemParam,
{
    type Fetch = (P1::Fetch,);
}
impl<'w, P1> SystemParamFetch<'w> for (P1,)
where
    P1: SystemParamFetch<'w>,
{
    type Item = (P1::Item,);
    fn fetch(world: &'w World) -> Self::Item {
        (P1::fetch(world),)
    }
}

impl<P1, P2> SystemParam for (P1, P2)
where
    P1: SystemParam,
    P2: SystemParam,
{
    type Fetch = (P1::Fetch, P2::Fetch);
}
impl<'w, P1, P2> SystemParamFetch<'w> for (P1, P2)
where
    P1: SystemParamFetch<'w>,
    P2: SystemParamFetch<'w>,
{
    type Item = (P1::Item, P2::Item);
    fn fetch(world: &'w World) -> Self::Item {
        (P1::fetch(world), P2::fetch(world))
    }
}
