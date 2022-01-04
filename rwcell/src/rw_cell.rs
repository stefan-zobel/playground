use crate::lock::{Lock, ReadGuard, WriteGuard};
use std::cell::UnsafeCell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
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

pub struct RwCellError {}

impl Error for RwCellError {}

impl Debug for RwCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt("RwCellError", f)
    }
}

impl Display for RwCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("already mutably borrowed by current thread", f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_borrow_mut() {
        let mut s1: &mut String = &mut String::from("not initialized");
        println!("{}", s1);

        let cell;
        {
            cell = RwCell::new(String::from("hello world"));
            let mut sync_ref_mut = cell.try_borrow_mut().unwrap();
            let s2: &mut String = &mut *sync_ref_mut;
            println!("got the string : {}", s2);
            s2.clear();
            s2.push_str("A changed string!");
            s1 = s2;
            println!("the string is now : {}", s1);
        }

        // error[E0597]: 'borrowed value does not live long enough'
        //s1.push_str(" - this shouldn't not work!");
        let s3 = &*cell.try_borrow_mut().unwrap();
        assert_eq!(s3, "A changed string!");
        println!("the string is still : {}", s3);
    }
}
