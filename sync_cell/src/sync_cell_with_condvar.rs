use crate::cond_var::RCondVar;
use parking_lot::lock_api::{ReentrantMutex, ReentrantMutexGuard};
use parking_lot::{RawMutex, RawThreadId};
use std::cell::{Cell, UnsafeCell};
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

type RMutex<T> = ReentrantMutex<RawMutex, RawThreadId, T>;
type RMutexGuard<'scope, T> = ReentrantMutexGuard<'scope, RawMutex, RawThreadId, T>;

pub trait ISyncCell2<T: ?Sized> {
    fn try_borrow(&self) -> Option<SyncRef2<'_, T>>;

    fn try_borrow_mut(&self) -> Option<SyncRefMut2<'_, T>>;

    fn borrow(&self) -> SyncRef2<'_, T>;

    fn borrow_mut(&self) -> SyncRefMut2<'_, T>;

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
enum CellState2 {
    Unshared,
    Shared(usize),
    Exclusive,
}

#[derive(Debug)]
pub struct SyncRef2<'scope, T: ?Sized> {
    cell: &'scope SyncCell2<T>,
    guard: RMutexGuard<'scope, UnsafeCell<T>>,
}

impl<T> Deref for SyncRef2<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        if let CellState2::Shared(_) = self.cell.state.get() {
            unsafe { &*self.guard.get() }
        } else {
            unreachable!()
        }
    }
}

impl<T: ?Sized> Drop for SyncRef2<'_, T> {
    fn drop(&mut self) {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        match self.cell.state.get() {
            CellState2::Exclusive | CellState2::Unshared => unreachable!(),
            CellState2::Shared(1) => {
                self.cell.state.set(CellState2::Unshared);
                self.cell.cond_unshared.notify_all();
            }
            CellState2::Shared(n) => {
                self.cell.state.set(CellState2::Shared(n - 1));
            }
        }
        // guard gets dropped
    }
}

#[derive(Debug)]
pub struct SyncRefMut2<'scope, T: ?Sized> {
    cell: &'scope SyncCell2<T>,
    guard: RMutexGuard<'scope, UnsafeCell<T>>,
}

impl<T> Deref for SyncRefMut2<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        if let CellState2::Exclusive = self.cell.state.get() {
            unsafe { &*self.guard.get() }
        } else {
            unreachable!();
        }
    }
}

impl<T: ?Sized> Drop for SyncRefMut2<'_, T> {
    fn drop(&mut self) {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        match self.cell.state.get() {
            CellState2::Shared(_) | CellState2::Unshared => unreachable!(),
            CellState2::Exclusive => {
                self.cell.state.set(CellState2::Unshared);
                self.cell.cond_not_exclusive.notify_all();
                self.cell.cond_unshared.notify_all();
            }
        }
        // guard gets dropped
    }
}

impl<T> DerefMut for SyncRefMut2<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(self.cell.mutex.is_locked());
        debug_assert!(self.cell.mutex.is_owned_by_current_thread());
        if let CellState2::Exclusive = self.cell.state.get() {
            unsafe { &mut *self.guard.get() }
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug)]
pub struct SyncCell2<T: ?Sized> {
    state: Cell<CellState2>,
    cond_unshared: RCondVar,
    cond_not_exclusive: RCondVar,
    mutex: RMutex<UnsafeCell<T>>,
}

unsafe impl<T: ?Sized + Send> Send for SyncCell2<T> {}
unsafe impl<T: ?Sized> Sync for SyncCell2<T> {}

impl<T> SyncCell2<T> {
    #![allow(dead_code)]
    #[inline]
    pub fn new(value: T) -> Self {
        SyncCell2 {
            mutex: RMutex::new(UnsafeCell::new(value)),
            state: Cell::new(CellState2::Unshared),
            cond_unshared: RCondVar::new(),
            cond_not_exclusive: RCondVar::new(),
        }
    }

    fn internal_try_borrow(&self, wait: bool) -> Option<SyncRef2<'_, T>> {
        'outer: loop {
            let mut lock: RMutexGuard<UnsafeCell<T>> = self.mutex.lock();
            match self.state.get() {
                CellState2::Unshared => {
                    self.state.set(CellState2::Shared(1));
                    return Some(SyncRef2 {
                        cell: self,
                        guard: lock,
                    });
                    // lock gets moved
                }
                CellState2::Shared(n) => {
                    self.state.set(CellState2::Shared(n + 1));
                    return Some(SyncRef2 {
                        cell: self,
                        guard: lock,
                    });
                    // lock gets moved
                }
                CellState2::Exclusive => {
                    if wait {
                        let guard = &mut lock;
                        self.cond_not_exclusive.wait(guard);
                        continue 'outer;
                    } else {
                        return None;
                        // lock gets dropped
                    }
                }
            }
        }
    }

    fn internal_try_borrow_mut(&self, wait: bool) -> Option<SyncRefMut2<'_, T>> {
        'outer: loop {
            let mut lock: RMutexGuard<UnsafeCell<T>> = self.mutex.lock();
            if let CellState2::Unshared = self.state.get() {
                self.state.set(CellState2::Exclusive);
                return Some(SyncRefMut2 {
                    cell: self,
                    guard: lock,
                });
                // lock gets moved
            } else if wait {
                let guard = &mut lock;
                self.cond_unshared.wait(guard);
                continue 'outer;
            } else {
                return None;
                // lock gets dropped
            }
        }
    }
}

impl<T> ISyncCell2<T> for SyncCell2<T> {
    fn try_borrow(&self) -> Option<SyncRef2<'_, T>> {
        self.internal_try_borrow(false)
    }

    fn try_borrow_mut(&self) -> Option<SyncRefMut2<'_, T>> {
        self.internal_try_borrow_mut(false)
    }

    fn borrow(&self) -> SyncRef2<'_, T> {
        loop {
            if let Some(syn_ref) = self.internal_try_borrow(true) {
                return syn_ref;
            }
        }
    }

    fn borrow_mut(&self) -> SyncRefMut2<'_, T> {
        loop {
            if let Some(syn_ref) = self.internal_try_borrow_mut(true) {
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
    //noinspection ALL
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

impl<T> From<T> for SyncCell2<T> {
    #[inline]
    fn from(val: T) -> Self {
        SyncCell2::new(val)
    }
}

impl<T: ?Sized + Default> Default for SyncCell2<T> {
    #[inline]
    fn default() -> Self {
        SyncCell2::new(Default::default())
    }
}
//noinspection ALL
impl<T: ?Sized + Clone> Clone for SyncCell2<T> {
    fn clone(&self) -> Self {
        SyncCell2::new(self.borrow().clone())
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

impl<T: PartialEq> PartialEq for SyncCell2<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

impl<T: Eq> Eq for SyncCell2<T> {}

impl<T: PartialOrd> PartialOrd for SyncCell2<T> {
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

impl<T: Ord> Ord for SyncCell2<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.borrow().cmp(&*other.borrow())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_wait_notify() {
        let hello = "hello";
        let m1 = Arc::new(SyncCell2::new(String::from(hello)));
        {
            let m2 = m1.clone();
            let sync_ref : SyncRef2<String> = m1.try_borrow().unwrap();
            let sm1 = &*sync_ref;
            let _t = thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                // m1 should be locked now, we must wait until
                // the lock from m1 gets dropped
                println!("b) waiting for m1 to drop its lock");
                let sm2 = &mut *m2.borrow_mut();
                assert_eq!(sm2, hello);
                println!("c) 2nd thread : {}", sm2);
                sm2.push_str(" - abc");
                println!("d) dropping lock on m2");
            });
            assert_eq!(sm1, hello);
            println!("a) main thread : {}", sm1);
            thread::sleep(Duration::from_secs(1));
            // lock from m1 gets destroyed here
        }
        thread::sleep(Duration::from_secs(1));
        let sm1 = &*m1.borrow();
        assert_eq!(sm1, "hello - abc");
        println!("e) main thread : {}", sm1);
    }

    #[test]
    fn test_clone_from() {
        let hello = "hello";
        let world = "world";
        let mut hello_cell = SyncCell2::new(String::from(hello));
        let world_cell = &SyncCell2::new(String::from(world));

        hello_cell.clone_from(world_cell);
        let sh = &*hello_cell.borrow();
        let sw = &*world_cell.borrow();
        println!("after clone_from in dest: {}", sh);
        assert_eq!(sh, world);
        println!("after clone_from in source: {}", sw);
        assert_eq!(sw, world);
    }

    #[test]
    fn test_clone() {
        let hello = "hello";
        let hello_cell = SyncCell2::new(String::from(hello));
        let clone = hello_cell.clone();
        {
            let sc = &mut *clone.borrow_mut();
            let sh = &*hello_cell.borrow();
            assert_eq!(sc, hello);
            assert_eq!(sh, hello);
            sc.push_str(" - abc");
        }
        let sc = &*clone.borrow();
        let sh = &mut *hello_cell.borrow_mut();
        assert_eq!(sc, "hello - abc");
        assert_eq!(sh, hello);
    }

    #[test]
    fn test_swap() {
        let hello = "hello";
        let world = "world";
        let hello_cell = SyncCell2::new(String::from(hello));
        let world_cell = SyncCell2::new(String::from(world));
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
        let cell = SyncCell2::new(String::from(hello));
        let res = cell.replace_with(f);
        println!("result is : {}", res);
        assert_eq!(res, "hello world");
    }

    #[test]
    fn test_replace() {
        let hello = "hello";
        let cell = SyncCell2::new(String::from(hello));
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
        let cell = SyncCell2::new(String::from(hello));
        let value = cell.take();
        let after_take = &*cell.borrow();
        println!("value is : {}", value);
        println!("value after take is : {}", after_take);
        assert_eq!(after_take, "");
        assert_eq!(value, hello);
    }

    #[test]
    fn test_into_inner() {
        let cell = SyncCell2::new(String::from("into inner"));
        let s = cell.into_inner();
        println!("got the string : {}", s);
    }

    #[test]
    fn test_try_borrow_mut() {
        let mut s1: &mut String = &mut String::from("not initialized");
        println!("{}", s1);

        let cell;
        {
            cell = SyncCell2::new(String::from("hello world"));
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
