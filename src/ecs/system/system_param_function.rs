use super::{SystemParam, SystemParamItem};

pub trait SystemParamFunction<Param: SystemParam>: 'static {
    fn run(&mut self, param: SystemParamItem<Param>);
}

impl<F: 'static> SystemParamFunction<()> for F
where
    for<'w> &'w mut F: FnMut(),
{
    fn run(&mut self, param: SystemParamItem<()>) {
        fn call_inner(mut f: impl FnMut()) {
            f();
        }
        let () = param;
        call_inner(self)
    }
}

impl<F: 'static, P1: SystemParam> SystemParamFunction<(P1,)> for F
where
    for<'w> &'w mut F: FnMut(P1) + FnMut(SystemParamItem<P1>),
{
    fn run(&mut self, param: SystemParamItem<(P1,)>) {
        fn call_inner<P1>(mut f: impl FnMut(P1), p1: P1) {
            f(p1);
        }
        let (p1,) = param;
        call_inner(self, p1)
    }
}

impl<F: 'static, P1: SystemParam, P2: SystemParam> SystemParamFunction<(P1, P2)> for F
where
    for<'w> &'w mut F: FnMut(P1, P2) + FnMut(SystemParamItem<P1>, SystemParamItem<P2>),
{
    fn run(&mut self, param: SystemParamItem<(P1, P2)>) {
        fn call_inner<P1, P2>(mut f: impl FnMut(P1, P2), p1: P1, p2: P2) {
            f(p1, p2);
        }
        let (p1, p2) = param;
        call_inner(self, p1, p2)
    }
}
