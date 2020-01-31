use core::sync::atomic::*;
use core::cell::UnsafeCell;
use core::ops::*;

pub struct StaticSpinlockGuard<'m, T> {
    lock: &'m StaticSpinlock<T>
}

impl<'m, T> Drop for StaticSpinlockGuard<'m, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'m, T> Deref for StaticSpinlockGuard<'m, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'m, T> DerefMut for StaticSpinlockGuard<'m, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}


pub struct StaticSpinlock<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool
}

impl<T> StaticSpinlock<T> {
    pub const fn new(data: T) -> StaticSpinlock<T> {
        StaticSpinlock { locked: AtomicBool::new(false), data: UnsafeCell::new(data) }
    }

    pub fn try_lock(&self) -> Option<StaticSpinlockGuard<T>> {
        let (current, new) = (false, true);
        if self.locked.compare_and_swap(current, new, Ordering::Acquire) == current {
            Some(StaticSpinlockGuard { lock: self })
        } else {
            None
        }
    }

    pub fn lock(&self) -> StaticSpinlockGuard<T> {
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
        }
    }
}

unsafe impl<T> Sync for StaticSpinlock<T> {}

