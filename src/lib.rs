mod cell;
mod vars;
mod snap;
mod lock;
mod tools;

use std::cell::UnsafeCell;

use std::sync::{Arc};


pub use crate::snap::snap::SnapMut;
use crate::tools::guard::Guard;

/// super high performance, threadsafe cell offering stress-free simple yet flexible interface with interior mutability
/// - atomically backed
/// - safe & fast locking (no RWLock)
/// - reads receive "snapshots" of values (whatever the value is on readtime), making reads lock-free and copy-free always
/// example:
/// ```
///     let cell = rapidsync::RapidSnap::new("Hey there");
///
///     let mut new_cell = cell.get_mut();
///
///     *new_cell = "new value";
///
///     println!("{}", cell.read());
/// ```
///
pub struct RapidSnap<T> {
    guard: Guard,
    data: UnsafeCell<Arc<T>>,
}


pub struct RapidCell {

}

pub struct RapidMap {

}
