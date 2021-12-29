use parking_lot::lock_api::{ReentrantMutex, ReentrantMutexGuard};
use parking_lot::{RawMutex, RawThreadId};
use std::cell::{Cell, UnsafeCell};
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

type RMutex<T> = ReentrantMutex<RawMutex, RawThreadId, T>;
type RMutexGuard<'scope, T> = ReentrantMutexGuard<'scope, RawMutex, RawThreadId, T>;

pub trait ISyncCell<T: ?Sized> {
    fn try_borrow(&self) -> Option<SyncRef<'_, T>>;

    fn try_borrow_mut(&self) -> Option<SyncRefMut<'_, T>>;

    fn borrow(&self) -> SyncRef<'_, T>;

    fn borrow_mut(&self) -> SyncRefMut<'_, T>;

    fn into_inner(self) -> T
    where
        T: Default;

    fn replace(&self, val: T) -> T;

    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T;

    fn swap(&self, other: &Self);

    fn take(&self) -> T
    where
        T: Default;
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
    pub fn new(value: T) -> Self {
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

    fn into_inner(self) -> T
    where
        T: Default,
    {
        std::mem::take(&mut *self.borrow_mut())
    }

    fn replace(&self, val: T) -> T {
        std::mem::replace(&mut *self.borrow_mut(), val)
    }

    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        let mut_borrow = &mut *self.borrow_mut();
        let replacement = f(mut_borrow);
        std::mem::replace(mut_borrow, replacement)
    }

    fn swap(&self, other: &Self) {
        std::mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut())
    }

    fn take(&self) -> T
    where
        T: Default,
    {
        self.replace(Default::default())
    }
}

impl<T> From<T> for SyncCell<T> {
    #[inline]
    fn from(val: T) -> Self {
        SyncCell::new(val)
    }
}

impl<T: ?Sized + Default> Default for SyncCell<T> {
    #[inline]
    fn default() -> Self {
        SyncCell::new(Default::default())
    }
}

impl<T: ?Sized + Clone> Clone for SyncCell<T> {
    fn clone(&self) -> Self {
        SyncCell::new(self.borrow().clone())
    }

    fn clone_from(&mut self, other: &Self) {
        let dst = &mut *self.borrow_mut();
        let source = &&*other.borrow();
        dst.clone_from(source);
    }
}

//impl<T> AsRef<T> for SyncCell<T> {
//    fn as_ref(&self) -> &T {
//        unsafe {&*self.borrow().guard.get()}
//    }
//}

impl<T: PartialEq> PartialEq for SyncCell<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

impl<T: Eq> Eq for SyncCell<T> {}

impl<T: PartialOrd> PartialOrd for SyncCell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.borrow().partial_cmp(&*other.borrow())
    }

    fn lt(&self, other: &Self) -> bool {
        *self.borrow() < *other.borrow()
    }

    fn le(&self, other: &Self) -> bool {
        *self.borrow() <= *other.borrow()
    }

    fn gt(&self, other: &Self) -> bool {
        *self.borrow() > *other.borrow()
    }

    fn ge(&self, other: &Self) -> bool {
        *self.borrow() >= *other.borrow()
    }
}

impl<T: Ord> Ord for SyncCell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.borrow().cmp(&*other.borrow())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clone_from() {
        let hello = "hello";
        let world = "world";
        let mut hello_cell = SyncCell::new(String::from(hello));
        let world_cell = &SyncCell::new(String::from(world));

        hello_cell.clone_from(world_cell);
        let sh = &*hello_cell.borrow();
        let sw = &*world_cell.borrow();
        println!("after clone_from in dest: {}", sh);
        assert_eq!(sh, world);
        println!("after clone_from in source: {}", sw);
        assert_eq!(sw, world);
    }

    #[test]
    fn test_swap() {
        let hello = "hello";
        let world = "world";
        let hello_cell = SyncCell::new(String::from(hello));
        let world_cell = SyncCell::new(String::from(world));
        hello_cell.swap(&world_cell);
        let sh = &*hello_cell.borrow();
        let sw = &*world_cell.borrow();
        assert_eq!(sh, world);
        assert_eq!(sw, hello);
    }

    #[test]
    fn test_replace_with() {
        let f = |s: &mut String| {
            s.push_str(" world");
            s.clone()
        };
        let hello = "hello";
        let cell = SyncCell::new(String::from(hello));
        let res = cell.replace_with(f);
        println!("result is : {}", res);
        assert_eq!(res, "hello world");
    }

    #[test]
    fn test_replace() {
        let hello = "hello";
        let cell = SyncCell::new(String::from(hello));
        let orig = cell.replace(String::from("world world"));
        println!("orig was : {}", orig);
        assert_eq!(orig, hello);
        let new = &*cell.borrow();
        println!("new is : {}", new);
        assert_eq!(new, "world world");
    }

    #[test]
    fn test_take() {
        let hello = "hello";
        let cell = SyncCell::new(String::from(hello));
        let value = cell.take();
        let after_take = &*cell.borrow();
        println!("value is : {}", value);
        println!("value after take is : {}", after_take);
        assert_eq!(after_take, "");
        assert_eq!(value, hello);
    }

    #[test]
    fn test_into_inner() {
        let cell = SyncCell::new(String::from("into inner"));
        let s = cell.into_inner();
        println!("got the string : {}", s);
    }

    #[test]
    fn test_try_borrow_mut() {
        let mut s1: &mut String = &mut String::from("not initialized");
        println!("{}", s1);

        let cell;
        {
            cell = SyncCell::new(String::from("hello world"));
            let mut sync_ref_mut = cell.try_borrow_mut().unwrap();
            let s2: &mut String = &mut *sync_ref_mut;
            println!("got the string : {}", s2);
            s2.clear();
            s2.push_str("A changed string!");
            s1 = s2;
            println!("the string is now : {}", s1);
        }

        // error[E0597]: 'borrowed value does not live long enough'
        //s.push_str(" - this shouldn't not work!");
        let s3 = &*cell.try_borrow_mut().unwrap();
        assert_eq!(s3, "A changed string!");
        println!("the string is still : {}", s3);
    }
}
