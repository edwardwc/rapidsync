use std::cell::Cell;
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::thread::ThreadId;
use crate::deadlock_detected;
use crate::vars::{LOCKED_BIT, UNLOCKED_BIT};

pub struct Guard {
    // our counter
    pub guard: AtomicU8,
    // for debug assertions only, to detect deadlocks
    #[cfg(debug_assertions)]
    pub locked: Cell<Option<ThreadId>>
}

impl Guard {
    pub fn new() -> Self {
        Self {
            guard: AtomicU8::new(UNLOCKED_BIT),
            locked: Cell::new(None),
        }
    }

    pub fn try_acquire_lock(&self) -> bool {
        if self.guard.compare_exchange(UNLOCKED_BIT, LOCKED_BIT, Ordering::Release, Ordering::Acquire).is_ok() {
            #[cfg(debug_assertions)] {
                self.locked.replace(Some(thread::current().id()));
            }
            return true
        }

        #[cfg(debug_assertions)] {
            if let Some(t) = self.locked.get() {
                if t == thread::current().id() {
                    deadlock_detected!("on thread {:?}", thread::current())
                }
            }
        }

        false
    }

    pub fn release_lock(&self) {
        self.guard.swap(UNLOCKED_BIT, Ordering::Release);
    }
}