use std::cell::Cell;
use std::sync::atomic::{AtomicU64, AtomicU8, fence, Ordering};
use std::thread;
use std::thread::ThreadId;
use crate::deadlock_detected;
use crate::vars::{LOCKED_BIT, READING_BIT, UNLOCKED_BIT, WRITER_WAITING};

#[derive(Debug)]
pub struct Guard {
    // our counter
    pub guard: AtomicU8,

    // current amount of readers
    pub readers: AtomicU64,

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
            readers: AtomicU64::new(0),
            #[cfg(debug_assertions)]
            locked: Cell::new(None),
            #[cfg(debug_assertions)]
            notified: Cell::new(false),
        }
    }

    pub fn try_acquire_lock(&self) -> bool {
        if (self.readers.load(Ordering::Acquire) == 0 && self.guard.compare_exchange(WRITER_WAITING, LOCKED_BIT, Ordering::Release, Ordering::Acquire).is_ok())
            || self.guard.compare_exchange(UNLOCKED_BIT, LOCKED_BIT, Ordering::Release, Ordering::Acquire).is_ok() {
            // we're okay to write
            #[cfg(debug_assertions)] {
                self.locked.replace(Some(thread::current().id()));
            }

            return true
        }

        self.guard.compare_exchange(READING_BIT, WRITER_WAITING, Ordering::Release, Ordering::Acquire);

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

    pub fn try_acquire_read(&self) -> bool {
        if self.guard.load(Ordering::Acquire) != WRITER_WAITING {
            if self.guard.compare_exchange(UNLOCKED_BIT, LOCKED_BIT, Ordering::Release, Ordering::Acquire).is_ok() {
                // self.readers.fetch_add(1, Ordering::AcqRel);

                return true
            }
        }

        false
    }

    pub fn release_read_lock(&self) {
        if self.guard.fetch_sub(1, Ordering::AcqRel) == 1 { // we're now at zero
            self.guard.store(UNLOCKED_BIT, Ordering::Release);
        }
    }

    pub fn release_write_lock(&self) {
        self.guard.store(UNLOCKED_BIT, Ordering::Release);
    }
}

