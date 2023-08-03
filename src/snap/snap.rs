use std::cell::UnsafeCell;
use std::hint;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc};
use std::sync::atomic::{AtomicU8, Ordering};
use crate::{RapidSnap, style_panic};
use crate::tools::guard::Guard;
use crate::vars::{LOCKED_BIT, UNLOCKED_BIT};

unsafe impl<T> Sync for RapidSnap<T> {}
unsafe impl<T> Send for RapidSnap<T> {}

impl<T> RapidSnap<T> {
    /// Create a new RapidSnap
    pub fn new(value: T) -> Self {
        RapidSnap {
            guard: Guard::new(),
            data: UnsafeCell::new(Arc::new(value))
        }
    }

    /// Read a value (this call is lockless)
    pub fn read(&self) -> Arc<T> {
        return unsafe { (*self).data.get().read().clone() }
    }

    /// Swap the data in the cell
    pub fn swap(&self, new_value: T) -> Arc<T> {
        loop {
            if self.guard.try_acquire_lock() {
                let val = unsafe { (*self).data.get().replace(Arc::new(new_value)) };

                self.guard.release_lock();

                return val
            }

            hint::spin_loop()
        }
    }

    /// Get a mutable reference to the object. This call blocks other mutable references or swaps until the mutable reference is dropped.
    pub fn get_mut(&self) -> SnapMut<T> {
        loop {
            if self.guard.try_acquire_lock() {
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
    pub(crate) data: &'a RapidSnap<T>,
}