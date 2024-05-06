use std::marker::PhantomData;

use super::World;

mod system_param;
mod system_param_function;

pub use system_param::{SystemParam, SystemParamFetch, SystemParamItem};
pub use system_param_function::SystemParamFunction;

pub trait System: 'static {
    fn run(&mut self, world: &mut World);
}

pub type BoxedSystem = Box<dyn System>;

pub struct FunctionSystem<F, Param>
where
    F: SystemParamFunction<Param>,
    Param: SystemParam,
{
    func: F,
    marker: PhantomData<Param>,
}

impl<F, Param> System for FunctionSystem<F, Param>
where
    F: SystemParamFunction<Param> + 'static,
    Param: SystemParam + 'static,
{
    fn run(&mut self, world: &mut World) {
        let params = <Param as SystemParam>::Fetch::fetch(world);
        self.func.run(params);
    }
}

pub trait IntoSystem<Params> {
    type System: System;
    fn into_system(self) -> Self::System;
}

impl<F, Params> IntoSystem<Params> for F
where
    F: SystemParamFunction<Params> + 'static,
    Params: SystemParam + 'static,
{
    type System = FunctionSystem<F, Params>;
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
