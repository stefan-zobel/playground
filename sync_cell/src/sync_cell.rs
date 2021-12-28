use parking_lot::lock_api::{ReentrantMutex, ReentrantMutexGuard};
use parking_lot::{RawMutex, RawThreadId};
use std::cell::{Cell, UnsafeCell};
use std::ops::{Deref, DerefMut};

type RMutex<T> = ReentrantMutex<RawMutex, RawThreadId, T>;
type RMutexGuard<'scope, T> = ReentrantMutexGuard<'scope, RawMutex, RawThreadId, T>;

pub trait ISyncCell<T: ?Sized> {
    fn try_borrow(&self) -> Option<SyncRef<'_, T>>;

    fn try_borrow_mut(&self) -> Option<SyncRefMut<'_, T>>;

    fn borrow(&self) -> SyncRef<'_, T>;

    fn borrow_mut(&self) -> SyncRefMut<'_, T>;
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum CellState {
    Unshared,
    Shared(usize),
    Exclusive,
}

#[derive(Debug)]
pub struct SyncRef<'scope, T: ?Sized> {
    cell: &'scope SyncCell<T>,
    guard: RMutexGuard<'scope, UnsafeCell<T>>,
}

impl<T> Deref for SyncRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        if let CellState::Shared(_) = self.cell.state.get() {
            unsafe { &*self.guard.get() }
        } else {
            unreachable!()
        }
    }
}

impl<T: ?Sized> Drop for SyncRef<'_, T> {
    fn drop(&mut self) {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        match self.cell.state.get() {
            CellState::Exclusive | CellState::Unshared => unreachable!(),
            CellState::Shared(1) => {
                self.cell.state.set(CellState::Unshared);
            }
            CellState::Shared(n) => {
                self.cell.state.set(CellState::Shared(n - 1));
            }
        }
        // guard gets dropped
    }
}

#[derive(Debug)]
pub struct SyncRefMut<'scope, T: ?Sized> {
    cell: &'scope SyncCell<T>,
    guard: RMutexGuard<'scope, UnsafeCell<T>>,
}

impl<T> Deref for SyncRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        if let CellState::Exclusive = self.cell.state.get() {
            unsafe { &*self.guard.get() }
        } else {
            unreachable!();
        }
    }
}

impl<T: ?Sized> Drop for SyncRefMut<'_, T> {
    fn drop(&mut self) {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        match self.cell.state.get() {
            CellState::Shared(_) | CellState::Unshared => unreachable!(),
            CellState::Exclusive => {
                self.cell.state.set(CellState::Unshared);
            }
        }
        // guard gets dropped
    }
}

impl<T> DerefMut for SyncRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        if let CellState::Exclusive = self.cell.state.get() {
            unsafe { &mut *self.guard.get() }
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug)]
pub struct SyncCell<T: ?Sized> {
    state: Cell<CellState>,
    mutex: RMutex<UnsafeCell<T>>,
}

unsafe impl<T: ?Sized + Send> Send for SyncCell<T> {}
unsafe impl<T: ?Sized> Sync for SyncCell<T> {}

impl<T> SyncCell<T> {
    #![allow(dead_code)]
    #[inline]
    fn new(value: T) -> Self {
        SyncCell {
            mutex: RMutex::new(UnsafeCell::new(value)),
            state: Cell::new(CellState::Unshared),
        }
    }
}

impl<T> ISyncCell<T> for SyncCell<T> {
    fn try_borrow(&self) -> Option<SyncRef<'_, T>> {
        let lock: RMutexGuard<UnsafeCell<T>> = self.mutex.lock();
        match self.state.get() {
            CellState::Unshared => {
                self.state.set(CellState::Shared(1));
                Some(SyncRef {
                    cell: self,
                    guard: lock,
                })
                // lock gets moved
            }
            CellState::Shared(n) => {
                self.state.set(CellState::Shared(n + 1));
                Some(SyncRef {
                    cell: self,
                    guard: lock,
                })
                // lock gets moved
            }
            CellState::Exclusive => None,
            // lock gets dropped
        }
    }

    fn try_borrow_mut(&self) -> Option<SyncRefMut<'_, T>> {
        let lock: RMutexGuard<UnsafeCell<T>> = self.mutex.lock();
        if let CellState::Unshared = self.state.get() {
            self.state.set(CellState::Exclusive);
            Some(SyncRefMut {
                cell: self,
                guard: lock,
            })
            // lock gets moved
        } else {
            None
            // lock gets dropped
        }
    }

    fn borrow(&self) -> SyncRef<'_, T> {
        loop {
            if let Some(syn_ref) = self.try_borrow() {
                return syn_ref;
            }
        }
    }

    fn borrow_mut(&self) -> SyncRefMut<'_, T> {
        loop {
            if let Some(syn_ref) = self.try_borrow_mut() {
                return syn_ref;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_borrow_mut() {
        let mut s1: &mut String = &mut String::from("not initialized");
        println!("{}", s1);

        let cell;
        {
            cell = SyncCell::new(String::from("hello world"));
            let mut sync_ref_mut = cell.try_borrow_mut().unwrap();
            let s2: &mut String = sync_ref_mut.deref_mut();
            println!("got the string : {}", s2);
            s2.clear();
            s2.push_str("A changed string!");
            s1 = s2;
            println!("the string is now : {}", s1);
        }

        // error[E0597]: 'borrowed value does not live long enough'
        //s.push_str(" - this shouldn't not work!");
        let s3 = cell.try_borrow_mut().unwrap().to_string();
        assert_eq!(s3, "A changed string!");
        println!("the string is still : {}", s3);
    }
}
