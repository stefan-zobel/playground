use rwlock_rs::{ReadGuard, WriteGuard};
use std::cell::{Ref, RefMut, UnsafeCell};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct RwRef<'scope, T> {
    pub(crate) guard: ReadGuard<'scope, UnsafeCell<T>>,
}

//noinspection ALL
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

//noinspection ALL
impl<T> Deref for RwRefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.guard.deref().get() }
    }
}

//noinspection ALL
impl<T> DerefMut for RwRefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.guard.deref().get() }
    }
}

#[derive(Debug)]
pub struct CellRef<'a, T> {
    pub(crate) ref_: EitherRef<'a, T>,
}

impl<T> Deref for CellRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.ref_.deref()
    }
}

#[derive(Debug)]
pub struct CellRefMut<'a, T> {
    pub(crate) ref_: EitherRefMut<'a, T>,
}

impl<T> Deref for CellRefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.ref_.deref()
    }
}

impl<T> DerefMut for CellRefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_.deref_mut()
    }
}

#[derive(Debug)]
pub(crate) enum EitherRef<'scope, T> {
    Std(Ref<'scope, T>),
    Locked(RwRef<'scope, T>),
}

#[derive(Debug)]
pub(crate) enum EitherRefMut<'scope, T> {
    Std(RefMut<'scope, T>),
    Locked(RwRefMut<'scope, T>),
}

impl<T> Deref for EitherRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            EitherRef::Std(std) => std.deref(),
            EitherRef::Locked(lck) => unsafe { &*lck.guard.deref().get() },
        }
    }
}

impl<T> Deref for EitherRefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            EitherRefMut::Std(std) => std.deref(),
            EitherRefMut::Locked(lck) => unsafe { &*lck.guard.deref().get() },
        }
    }
}

impl<T> DerefMut for EitherRefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            EitherRefMut::Std(std) => std.deref_mut(),
            EitherRefMut::Locked(lck) => unsafe { &mut *lck.guard.deref().get() },
        }
    }
}

#[cfg(test)]
mod refs_tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_size() {
        let size1 = size_of::<CellRef<'_, ()>>();
        let size2 = size_of::<CellRefMut<'_, ()>>();
        let size3 = size_of::<Ref<'_, ()>>();
        let size4 = size_of::<RefMut<'_, ()>>();
        let size5 = size_of::<RwRef<'_, ()>>();
        let size6 = size_of::<RwRefMut<'_, ()>>();
        println!("Ref wrapper size: {}", size1);
        println!("MutRef wrapper size: {}", size2);
        println!("Std Ref size: {}", size3);
        println!("Std RefMut size: {}", size4);
        println!("Rw Ref size: {}", size5);
        println!("Rw RefMut size: {}", size6);
    }
}
