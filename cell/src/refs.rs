use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use crate::lock::{ReadGuard, WriteGuard};

#[derive(Debug)]
pub struct RwRef<'scope, T: ?Sized> {
    pub(crate) guard: ReadGuard<'scope, UnsafeCell<T>>,
}

impl<T> Deref for RwRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.guard.deref().get() }
    }
}

#[derive(Debug)]
pub struct RwRefMut<'scope, T: ?Sized> {
    pub(crate) guard: WriteGuard<'scope, UnsafeCell<T>>,
}

impl<T> Deref for RwRefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.guard.deref().get() }
    }
}

impl<T> DerefMut for RwRefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.guard.deref().get() }
    }
}
