use core::sync::atomic::*;
use core::cell::UnsafeCell;
use core::ops::*;

pub struct SpinlockGuard<'m, T> {
    lock: &'m Spinlock<T>
}

impl<'m, T> Drop for SpinlockGuard<'m, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'m, T> Deref for SpinlockGuard<'m, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'m, T> DerefMut for SpinlockGuard<'m, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}


pub struct Spinlock<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool
}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Spinlock<T> {
        Spinlock { locked: AtomicBool::new(false), data: UnsafeCell::new(data) }
    }

    pub fn try_lock(&self) -> Option<SpinlockGuard<T>> {
        let (current, new) = (false, true);
        if self.locked.compare_and_swap(current, new, Ordering::Acquire) == current {
            Some(SpinlockGuard { lock: self })
        } else {
            None
        }
    }

    pub fn lock(&self) -> SpinlockGuard<T> {
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
        }
    }
}

unsafe impl<T> Sync for Spinlock<T> {}

