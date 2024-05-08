use std::marker::PhantomData;

pub use system_param::{SystemParam, SystemParamItem};

use super::World;

use self::system_param_function::SystemParamFunction;

mod system_param;
mod system_param_function;

pub trait System {
    fn run(&mut self, world: &mut World);
}

pub type BoxedSystem = Box<dyn System>;

pub struct FunctionSystem<F, M>
where
    F: SystemParamFunction<M>,
    M: 'static,
{
    func: F,
    marker: PhantomData<M>,
}

impl<F, M> System for FunctionSystem<F, M>
where
    F: SystemParamFunction<M>,
    M: 'static,
{
    fn run(&mut self, world: &mut World) {
        let param = F::Param::fetch(world);
        self.func.run(param);
    }
}

pub trait IntoSystem<Params> {
    type System: System + 'static;

    fn into_system(self) -> Self::System;
}

impl<F, M> IntoSystem<M> for F
where
    M: 'static,
    F: SystemParamFunction<M>,
{
    type System = FunctionSystem<F, M>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            func: self,
            marker: PhantomData,
        }
    }
}

#[test]
fn test_0() {
    fn test() {}
    let mut system = IntoSystem::<()>::into_system(test);
    let mut world = World::default();
    system.run(&mut world);
}
