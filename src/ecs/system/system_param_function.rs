use super::{SystemParam, SystemParamItem};

pub trait SystemParamFunction<M>: 'static {
    type Param: SystemParam;

    fn run(&mut self, param: SystemParamItem<Self::Param>);
}

impl<F: 'static> SystemParamFunction<()> for F
where
    for<'w> &'w mut F: FnMut(),
{
    type Param = ();

    fn run(&mut self, param: SystemParamItem<Self::Param>) {
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
    type Param = (P1,);

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
    type Param = (P1, P2);

    fn run(&mut self, param: SystemParamItem<(P1, P2)>) {
        fn call_inner<P1, P2>(mut f: impl FnMut(P1, P2), p1: P1, p2: P2) {
            f(p1, p2);
        }
        let (p1, p2) = param;
        call_inner(self, p1, p2)
    }
}

impl<F: 'static, P1: SystemParam, P2: SystemParam, P3: SystemParam>
    SystemParamFunction<(P1, P2, P3)> for F
where
    for<'w> &'w mut F:
        FnMut(P1, P2, P3) + FnMut(SystemParamItem<P1>, SystemParamItem<P2>, SystemParamItem<P3>),
{
    type Param = (P1, P2, P3);

    fn run(&mut self, param: SystemParamItem<(P1, P2, P3)>) {
        fn call_inner<P1, P2, P3>(mut f: impl FnMut(P1, P2, P3), p1: P1, p2: P2, p3: P3) {
            f(p1, p2, p3);
        }
        let (p1, p2, p3) = param;
        call_inner(self, p1, p2, p3)
    }
}
