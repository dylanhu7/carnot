use std::{
    cell::{Ref, RefMut},
    ops::{Deref, DerefMut},
};

pub struct Res<'a, T: 'static> {
    pub value: Ref<'a, T>,
}

impl<T: 'static> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.deref()
    }
}

pub struct ResMut<'a, T: 'static> {
    pub value: RefMut<'a, T>,
}

impl<T: 'static> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.deref()
    }
}

impl<T: 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.deref_mut()
    }
}
