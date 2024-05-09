use std::{
    any::Any,
    cell::RefCell,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

pub trait ComponentVec {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + 'static> ComponentVec for RefCell<Vec<Option<Rc<T>>>> {
    fn push_none(&mut self) {
        self.get_mut().push(None);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for Box<dyn ComponentVec> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComponentVec").finish()
    }
}

pub trait ComponentCell {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + 'static> ComponentCell for Rc<RefCell<T>> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
