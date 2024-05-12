use std::marker::PhantomData;

pub use system_param::{SystemOrWorldParam, SystemParam, SystemParamItem};

use super::World;

use self::system_param_function::SystemParamFunction;

mod system_param;
mod system_param_function;

pub trait System {
    fn run(&mut self, world: &mut World);
}

pub type BoxedSystem = Box<dyn System>;

pub struct SystemParamFunctionHolder<F, M>
where
    F: SystemParamFunction<M>,
    M: 'static,
{
    func: F,
    marker: PhantomData<M>,
}

impl<F, M> System for SystemParamFunctionHolder<F, M>
where
    F: SystemParamFunction<M>,
    M: 'static,
{
    fn run(&mut self, world: &mut World) {
        let param = F::Param::fetch(world);
        self.func.run(param);
    }
}

pub trait IntoSystem<M> {
    type System: System + 'static;

    fn into_system(self) -> Self::System;
}

impl<F, M: SystemParam + 'static> IntoSystem<M> for F
where
    F: SystemParamFunction<M>,
{
    type System = SystemParamFunctionHolder<F, M>;
    fn into_system(self) -> Self::System {
        SystemParamFunctionHolder {
            func: self,
            marker: PhantomData,
        }
    }
}

pub trait WorldParamFunction {
    fn run(&mut self, world: &mut World);
}

impl<F> WorldParamFunction for F
where
    F: for<'w> FnMut(&'w mut World),
{
    fn run(&mut self, world: &mut World) {
        self(world)
    }
}

impl<F> IntoSystem<&mut World> for F
where
    F: WorldParamFunction + 'static,
{
    type System = WorldParamFunctionHolder<F>;
    fn into_system(self) -> Self::System {
        WorldParamFunctionHolder { func: self }
    }
}

pub struct WorldParamFunctionHolder<F>
where
    F: WorldParamFunction,
{
    func: F,
}

impl<F> System for WorldParamFunctionHolder<F>
where
    F: WorldParamFunction,
{
    fn run(&mut self, world: &mut World) {
        self.func.run(world);
    }
}

#[test]
fn test_0() {
    fn test() {}
    let mut system = IntoSystem::into_system(test);
    let mut world = World::default();
    system.run(&mut world);
}
