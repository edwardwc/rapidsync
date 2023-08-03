use std::cell::UnsafeCell;
use std::hint;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc};
use std::sync::atomic::{AtomicU8, Ordering};
use crate::{RapidSnap, style_panic};
use crate::vars::{LOCKED_BIT, UNLOCKED_BIT};

unsafe impl<T> Sync for RapidSnap<T> {}
unsafe impl<T> Send for RapidSnap<T> {}

impl<T> RapidSnap<T> {
    /// Create a new RapidSnap
    pub fn new(value: T) -> Self {
        RapidSnap {
            guard: AtomicU8::new(UNLOCKED_BIT),
            data: UnsafeCell::new(Arc::new(value))
        }
    }

    //
    pub fn read(&self) -> Arc<T> {
        return unsafe { (*self).data.get().read().clone() }
    }

    pub fn swap(&self, new_value: T) -> Arc<T> {
        loop {
            if self.guard.compare_exchange(UNLOCKED_BIT, LOCKED_BIT, Ordering::Release, Ordering::Acquire).is_ok() {
                let val = unsafe { (*self).data.get().replace(Arc::new(new_value)) };

                self.guard.store(UNLOCKED_BIT,Ordering::Release);

                return val
            }

            hint::spin_loop()
        }
    }

    pub fn get_mut<'a>(&'a self) -> SnapMut<'a, T> {
        loop {
            if self.guard.compare_exchange(UNLOCKED_BIT, LOCKED_BIT, Ordering::Release, Ordering::Acquire).is_ok() {
                return SnapMut {
                    data: &self,
                }
            }

            hint::spin_loop()
        }
    }
}

/// A (Rapidsnap) guard for mutable references
pub struct SnapMut<'a, T> {
    data: &'a RapidSnap<T>,
}

impl<'a, T> Deref for SnapMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data.data.get() }
    }
}

impl<'a, T> DerefMut for SnapMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match Arc::get_mut(unsafe { &mut *self.data.data.get() }) {
            Some(t) => t,
            None => style_panic!("Failed to dereference to mutable reference")
        }
    }
}

impl<'a, T> Drop for SnapMut<'a, T> {
    fn drop(&mut self) {
        self.data.guard.store(UNLOCKED_BIT, Ordering::Release)
    }
}

/*
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Snap<T> {
        value: Arc<T>
    }

    impl Snap<T> {
        pub fn new(value: T) -> Snap<T> {
            Snap {
                value: Arc::new(value),
            }
        }
    }
 */