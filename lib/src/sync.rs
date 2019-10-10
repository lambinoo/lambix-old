use core::sync::atomic::*;
use core::cell::UnsafeCell;
use core::ops::*;

pub struct MutexGuard<'m, T> {
    mutex: &'m Mutex<T>
}

impl<'m, T> Drop for MutexGuard<'m, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<'m, T> Deref for MutexGuard<'m, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'m, T> DerefMut for MutexGuard<'m, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}


pub struct Mutex<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool
}

impl<T> Mutex<T> {
    #[deprecated(note = "very very naive implementation")]
    pub const fn new(data: T) -> Mutex<T> {
        Mutex { locked: AtomicBool::new(false), data: UnsafeCell::new(data) }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        let (current, new) = (false, true);
        if self.locked.compare_and_swap(current, new, Ordering::Acquire) == current {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
        }
    }
}

unsafe impl<T> Sync for Mutex<T> {}
