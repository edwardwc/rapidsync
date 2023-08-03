mod cell;
mod vars;
mod snap;
mod lock;

use std::cell::UnsafeCell;

use std::sync::{Arc};
use std::sync::atomic::{AtomicU8};

pub use crate::snap::snap::SnapMut;

/// super high performance, threadsafe cell offering stress-free simple yet flexible interface with interior mutability
/// - atomically backed
/// - safe & fast locking (no RWLock)
/// - reads receive "snapshots" of values (whatever the value is on readtime), making reads lock-free and copy-free always
pub struct RapidSnap<T> {
    guard: AtomicU8,
    data: UnsafeCell<Arc<T>>,
}


pub struct RapidCell {

}

pub struct RapidMap {

}
