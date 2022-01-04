use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::{RawRwLock, RwLock};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub(crate) struct Lock<T: ?Sized> {
    owner: Owner,
    rw_lock: RwLock<T>,
}

unsafe impl<T: ?Sized + Send> Send for Lock<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for Lock<T> {}

impl<T> Lock<T> {
    #[inline]
    pub(crate) fn new(val: T) -> Lock<T> {
        Lock {
            rw_lock: RwLock::new(val),
            owner: Default::default(),
        }
    }

    #[inline]
    pub(crate) fn is_locked(&self) -> bool {
        self.rw_lock.is_locked()
    }

    #[inline]
    pub(crate) fn is_owned_by_current_thread(&self) -> bool {
        self.is_locked() && self.owner.is_current_thread()
    }

    #[inline]
    pub(crate) fn try_read_shared(&self) -> Option<ReadGuard<'_, T>> {
        if let Some(lock) = self.rw_lock.try_read_recursive() {
            assert!(!self.owner.is_owned());
            return Some(ReadGuard { guard: lock });
        }
        None
    }

    #[inline]
    pub(crate) fn read_shared(&self) -> Result<ReadGuard<'_, T>, LockError> {
        if self.is_owned_by_current_thread() {
            return Err(LockError {});
        }
        let lock = self.rw_lock.read_recursive();
        assert!(!self.owner.is_owned());
        Ok(ReadGuard { guard: lock })
    }

    #[inline]
    pub(crate) fn try_write_exclusive(&self) -> Option<WriteGuard<'_, T>> {
        if let Some(lock) = self.rw_lock.try_write() {
            self.owner.take_ownership();
            return Some(WriteGuard {
                lock: self,
                guard: lock,
            });
        }
        None
    }

    #[inline]
    pub(crate) fn write_exclusive(&self) -> Result<WriteGuard<'_, T>, LockError> {
        if self.is_owned_by_current_thread() {
            return Err(LockError {});
        }
        let lock = self.rw_lock.write();
        self.owner.take_ownership();
        Ok(WriteGuard {
            lock: self,
            guard: lock,
        })
    }
}

#[derive(Debug)]
pub(crate) struct WriteGuard<'a, T: ?Sized> {
    lock: &'a Lock<T>,
    guard: RwLockWriteGuard<'a, RawRwLock, T>,
}

impl<T: ?Sized> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.owner.release_ownership();
    }
}

#[derive(Debug)]
pub(crate) struct ReadGuard<'a, T: ?Sized> {
    guard: RwLockReadGuard<'a, RawRwLock, T>,
}

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

#[derive(Debug)]
struct Owner(AtomicU64);

impl Owner {
    #[inline]
    fn is_current_thread(&self) -> bool {
        let id = self.0.load(Ordering::Relaxed);
        if id != 0 && id == Owner::current_thread_id() {
            return true;
        }
        false
    }

    #[inline]
    fn take_ownership(&self) {
        assert!(!self.is_owned());
        self.0.store(Owner::current_thread_id(), Ordering::Relaxed);
    }

    #[inline]
    fn release_ownership(&self) {
        assert!(self.is_current_thread());
        self.0.store(0, Ordering::Relaxed);
    }

    #[inline]
    fn is_owned(&self) -> bool {
        self.0.load(Ordering::Relaxed) != 0
    }

    #[inline]
    fn new() -> Owner {
        Owner(AtomicU64::new(0))
    }

    #[inline]
    fn current_thread_id() -> u64 {
        std::thread::current().id().as_u64().get()
    }
}

impl Default for Owner {
    #[inline]
    fn default() -> Self {
        Owner::new()
    }
}

pub(crate) struct LockError {}

impl Error for LockError {}

impl Debug for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt("LockError", f)
    }
}

impl Display for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("lock already exclusively owned by current thread", f)
    }
}
