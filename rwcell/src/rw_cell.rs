use crate::lock::{Lock, ReadGuard, WriteGuard};
use crate::rw_cell_error::RwCellError;
use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct RwCell<T: ?Sized> {
    lock: Lock<UnsafeCell<T>>,
}

//unsafe impl<T: ?Sized + Send> Send for Lock<T> {}
//unsafe impl<T: ?Sized + Send + Sync> Sync for Lock<T> {}

//unsafe impl<T: ?Sized + Send> Send for SyncCell<T> {}
//unsafe impl<T: ?Sized> Sync for SyncCell<T> {}

impl<T> RwCell<T> {
    pub fn try_borrow(&self) -> Option<RwRef<'_, T>> {
        self.lock.try_read_shared().map(|lock| RwRef {
            cell: self,
            guard: lock,
        })
    }

    pub fn try_borrow_mut(&self) -> Option<RwRefMut<'_, T>> {
        self.lock.try_write_exclusive().map(|lock| RwRefMut {
            cell: self,
            guard: lock,
        })
    }

    pub fn borrow(&self) -> Result<RwRef<'_, T>, RwCellError> {
        match self.lock.read_shared() {
            Ok(lock) => Ok(RwRef {
                cell: self,
                guard: lock,
            }),
            Err(_) => Err(RwCellError {}),
        }
    }

    pub fn borrow_mut(&self) -> Result<RwRefMut<'_, T>, RwCellError> {
        match self.lock.write_exclusive() {
            Ok(lock) => Ok(RwRefMut {
                cell: self,
                guard: lock,
            }),
            Err(_) => Err(RwCellError {}),
        }
    }
}

#[derive(Debug)]
pub struct RwRef<'scope, T: ?Sized> {
    cell: &'scope RwCell<T>,
    guard: ReadGuard<'scope, UnsafeCell<T>>,
}

#[derive(Debug)]
pub struct RwRefMut<'scope, T: ?Sized> {
    cell: &'scope RwCell<T>,
    guard: WriteGuard<'scope, UnsafeCell<T>>,
}
