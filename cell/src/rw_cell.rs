use crate::errors::{ERR_MSG, CellError};
use rwlock_rs::Lock;
use crate::refs::{RwRef, RwRefMut};
use std::cell::UnsafeCell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct RwCell<T: ?Sized> {
    lock: Lock<UnsafeCell<T>>,
}

unsafe impl<T: ?Sized + Send> Send for RwCell<T> {}
//unsafe impl<T: ?Sized + Send> Sync for RwCell<T> {} ?
unsafe impl<T: ?Sized + Send + Sync> Sync for RwCell<T> {}

impl<T> RwCell<T> {
    #[inline]
    pub fn new(val: T) -> RwCell<T> {
        RwCell {
            lock: Lock::new(UnsafeCell::new(val)),
        }
    }

    //noinspection ALL
    #[inline]
    pub fn try_borrow(&self) -> Option<RwRef<'_, T>> {
        self.lock
            .try_read_shared()
            .map(|lock| RwRef { guard: lock })
    }

    //noinspection ALL
    #[inline]
    pub fn try_borrow_mut(&self) -> Option<RwRefMut<'_, T>> {
        self.lock
            .try_write_exclusive()
            .map(|lock| RwRefMut { guard: lock })
    }

    #[inline]
    pub fn borrow(&self) -> Result<RwRef<'_, T>, CellError> {
        match self.lock.read_shared() {
            Ok(lock) => Ok(RwRef { guard: lock }),
            Err(_) => Err(CellError {}),
        }
    }

    #[inline]
    pub fn borrow_mut(&self) -> Result<RwRefMut<'_, T>, CellError> {
        match self.lock.write_exclusive() {
            Ok(lock) => Ok(RwRefMut { guard: lock }),
            Err(_) => Err(CellError {}),
        }
    }

    #[inline]
    pub fn borrow_panic(&self) -> RwRef<'_, T> {
        self.borrow().expect(ERR_MSG)
    }

    #[inline]
    pub fn borrow_mut_panic(&self) -> RwRefMut<'_, T> {
        self.borrow_mut().expect(ERR_MSG)
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
    use super::*;

    //noinspection ALL
    #[test]
    fn test_try_borrow_mut() {
        let mut s1: &mut String = &mut String::from("not initialized");
        println!("{}", s1);

        let cell;
        {
            cell = RwCell::new(String::from("hello world"));
            let mut sync_ref_mut = cell.try_borrow_mut().unwrap();
            let ref mut s2 = *sync_ref_mut;
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
