use parking_lot::lock_api::{GetThreadId, RawMutex, ReentrantMutexGuard};
use parking_lot::{Condvar, Mutex};

pub type RCondVar = CondVarForRMutexGuard;

#[derive(Debug)]
pub struct CondVarForRMutexGuard {
    c: Condvar,
    m: Mutex<()>,
}

impl CondVarForRMutexGuard {
    #[inline]
    pub fn new() -> CondVarForRMutexGuard {
        CondVarForRMutexGuard {
            c: Condvar::new(),
            m: Mutex::new(()),
        }
    }

    // see https://github.com/Amanieu/parking_lot/issues/165
    #[inline]
    pub fn wait<R: RawMutex, G: GetThreadId, T: ?Sized>(
        &self,
        g: &mut ReentrantMutexGuard<'_, R, G, T>,
    ) {
        let new_guard = self.m.lock();
        ReentrantMutexGuard::unlocked(g, || {
            // Move the new guard in so it gets unlocked
            // before we re-lock the ReentrantMutexGuard
            let mut guard = new_guard;
            self.c.wait(&mut guard);
        });
    }

    #[inline]
    pub fn notify_all(&self) {
        self.c.notify_all();
    }
}

impl Default for CondVarForRMutexGuard {
    fn default() -> Self {
        Self::new()
    }
}
