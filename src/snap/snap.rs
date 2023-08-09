use std::cell::UnsafeCell;
use std::{hint, mem};
use std::mem::forget;
use std::ops::Deref;

use std::sync::{Arc};

use crate::{RapidSnap, style_panic};
use crate::tools::guard::Guard;
use crate::tools::snap::SnapRef;

unsafe impl<T: Send + Sync + Clone> Sync for RapidSnap<T> {}
unsafe impl<T: Send + Sync + Clone> Send for RapidSnap<T> {}

impl<T: Send + Sync + Clone> RapidSnap<T> {
    /// Create a new RapidSnap
    pub fn new(mut value: T) -> Self {
        RapidSnap {
            guard: Guard::new(),
            data: UnsafeCell::new(Arc::new(value))
        }
    }

    /// Read a value (this call is lockless)
    pub fn read(&self) -> SnapRef<T> {
        return SnapRef {
            data: (unsafe { &*(*self).data.get() }).clone(),
        }
/*
        loop {
            if self.guard.try_acquire_read() {
                let val = (unsafe { &*(*self).data.get() }).deref();

                self.guard.release_read_lock();

                return val
            }

            hint::spin_loop()
        }

 */
    }

    /// Swap the data in the cell
    pub fn swap(&self, mut new_value: T) -> SnapRef<T> {
        loop {
            if self.guard.try_acquire_lock() {
                let mut foo = match Arc::get_mut(unsafe { &mut *self.data.get() }) {
                    Some(t) => t,
                    None => style_panic!("Failed to dereference to write value")
                };

                let old_ref = SnapRef {
                    data: Arc::new((*foo).clone()),
                };

                *foo = new_value;

                self.guard.release_write_lock();

                return old_ref
            }

            hint::spin_loop()
        }
    }

    /// Get a mutable reference to the object. This call blocks other mutable references or swaps until the mutable reference is dropped.
    pub fn get_mut(&self) -> SnapMut<T> {
        loop {
            if self.guard.try_acquire_lock() {
                return SnapMut {
                    data: self,
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