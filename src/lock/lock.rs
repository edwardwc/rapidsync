use std::cell::UnsafeCell;
use std::sync::atomic::AtomicU64;

pub struct Lock<T> {
    guard: AtomicU64,
    state: UnsafeCell<T>,
}

impl Lock<T> {
    pub fn new<T>() {
        Lock {
            guard: Default::default(),
            state: UnsafeCell::new(()),
        }
    }
}