use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::{RapidSnap, SnapMut, style_panic};
use crate::vars::UNLOCKED_BIT;

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
        self.data.guard.release_lock()
    }
}