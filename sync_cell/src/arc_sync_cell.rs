use crate::sync_cell::SyncCell;
use std::ops::Deref;
use std::sync::{Arc, Weak};

pub struct ArcSyncCell<T: ?Sized> {
    arc: Arc<SyncCell<T>>,
}

unsafe impl<T> Send for ArcSyncCell<T> where SyncCell<T>: Send {}
unsafe impl<T> Sync for ArcSyncCell<T> where SyncCell<T>: Sync {}

pub struct WeakSyncCell<T: ?Sized> {
    weak: Weak<SyncCell<T>>,
}

unsafe impl<T> Send for WeakSyncCell<T> where SyncCell<T>: Send {}
unsafe impl<T> Sync for WeakSyncCell<T> where SyncCell<T>: Sync {}

impl<T> ArcSyncCell<T> {
    #[inline]
    pub fn new(val: T) -> ArcSyncCell<T> {
        ArcSyncCell {
            arc: Arc::new(SyncCell::new(val)),
        }
    }

    // private associated function
    #[inline]
    fn internal_new(val: Arc<SyncCell<T>>) -> ArcSyncCell<T> {
        ArcSyncCell { arc: val }
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        ArcSyncCell::strong_count_(&self.arc)
    }

    // private associated function
    #[inline]
    fn strong_count_(this: &Arc<SyncCell<T>>) -> usize {
        Arc::<SyncCell<T>>::strong_count(this)
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        ArcSyncCell::weak_count_(&self.arc)
    }

    // private associated function
    #[inline]
    fn weak_count_(this: &Arc<SyncCell<T>>) -> usize {
        Arc::<SyncCell<T>>::weak_count(this)
    }

    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        ArcSyncCell::ptr_eq_(&self.arc, &other.arc)
    }

    // private associated function
    #[inline]
    fn ptr_eq_(this: &Arc<SyncCell<T>>, other: &Arc<SyncCell<T>>) -> bool {
        Arc::<SyncCell<T>>::ptr_eq(this, other)
    }

    #[must_use = "this returns a new `WeakSyncCell` pointer, \
                  without modifying the original `ArcSyncCell`"]
    pub fn create_weak(&self) -> WeakSyncCell<T> {
        ArcSyncCell::downgrade(&self.arc)
    }

    // private associated function
    fn downgrade(this: &Arc<SyncCell<T>>) -> WeakSyncCell<T> {
        WeakSyncCell::internal_new(Arc::<SyncCell<T>>::downgrade(this))
    }
}

impl<T: ?Sized> Deref for ArcSyncCell<T> {
    type Target = SyncCell<T>;

    fn deref(&self) -> &Self::Target {
        self.arc.deref()
    }
}

impl<T> Clone for ArcSyncCell<T> {
    #[inline]
    fn clone(&self) -> Self {
        ArcSyncCell::internal_new(self.arc.clone())
    }
}

impl<T> WeakSyncCell<T> {
    // private associated function
    #[inline]
    fn internal_new(val: Weak<SyncCell<T>>) -> WeakSyncCell<T> {
        WeakSyncCell { weak: val }
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        self.weak.strong_count()
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        self.weak.weak_count()
    }

    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.weak.ptr_eq(&other.weak)
    }

    #[must_use = "this returns a new `ArcSyncCell`, \
                  without modifying the original weak pointer"]
    pub fn try_create_strong(&self) -> Option<ArcSyncCell<T>> {
        self.upgrade()
    }

    // private method
    fn upgrade(&self) -> Option<ArcSyncCell<T>> {
        self.weak
            .upgrade()
            .map(|upgraded| ArcSyncCell { arc: upgraded })
    }
}

impl<T> Clone for WeakSyncCell<T> {
    #[inline]
    fn clone(&self) -> Self {
        WeakSyncCell::internal_new(self.weak.clone())
    }
}
