use crate::lock::{Lock, ReadGuard, WriteGuard};
use crate::rw_cell_error::RwCellError;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct RwCell<T: ?Sized> {
    lock: Lock<UnsafeCell<T>>,
}

unsafe impl<T: ?Sized + Send> Send for RwCell<T> {}
unsafe impl<T: ?Sized + Send> Sync for RwCell<T> {}
//unsafe impl<T: ?Sized + Send + Sync> Sync for RwCell<T> {} ?

impl<T> RwCell<T> {
    #[inline]
    pub fn new(val: T) -> RwCell<T> {
        RwCell {
            lock: Lock::new(UnsafeCell::new(val)),
        }
    }

    #[inline]
    pub fn try_borrow(&self) -> Option<RwRef<'_, T>> {
        self.lock
            .try_read_shared()
            .map(|lock| RwRef { guard: lock })
    }

    #[inline]
    pub fn try_borrow_mut(&self) -> Option<RwRefMut<'_, T>> {
        self.lock
            .try_write_exclusive()
            .map(|lock| RwRefMut { guard: lock })
    }

    #[inline]
    pub fn borrow(&self) -> Result<RwRef<'_, T>, RwCellError> {
        match self.lock.read_shared() {
            Ok(lock) => Ok(RwRef { guard: lock }),
            Err(_) => Err(RwCellError {}),
        }
    }

    #[inline]
    pub fn borrow_mut(&self) -> Result<RwRefMut<'_, T>, RwCellError> {
        match self.lock.write_exclusive() {
            Ok(lock) => Ok(RwRefMut { guard: lock }),
            Err(_) => Err(RwCellError {}),
        }
    }
}

#[derive(Debug)]
pub struct RwRef<'scope, T: ?Sized> {
    guard: ReadGuard<'scope, UnsafeCell<T>>,
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
    guard: WriteGuard<'scope, UnsafeCell<T>>,
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

impl<T> From<T> for RwCell<T> {
    #[inline]
    fn from(val: T) -> Self {
        RwCell::new(val)
    }
}

impl<T: ?Sized + Default> Default for RwCell<T> {
    #[inline]
    fn default() -> Self {
        RwCell::new(Default::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {}
}
