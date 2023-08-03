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
    pub locked: Cell<Option<ThreadId>>,

    // to display notifications
    #[cfg(debug_assertions)]
    pub notified: Cell<bool>
}

impl Guard {
    pub fn new() -> Self {
        Self {
            guard: AtomicU8::new(UNLOCKED_BIT),
            #[cfg(debug_assertions)]
            locked: Cell::new(None),
            #[cfg(debug_assertions)]
            notified: Cell::new(false),
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
                if !self.notified.get() {
                    let id = thread::current().id();

                    if t == id {
                        deadlock_detected!("on {:?}", id);
                        self.notified.replace(true);
                    }
                }
            }
        }

        false
    }

    pub fn can_read(&self) -> bool {
        if self.guard.load(Ordering::Acquire) == UNLOCKED_BIT {
            return true
        }

        false
    }

    pub fn release_lock(&self) {
        self.guard.swap(UNLOCKED_BIT, Ordering::Release);
    }
}