mod vars;
mod snap;
mod tools;
mod map;

use std::cell::UnsafeCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::Hash;

use std::sync::{Arc};
use crate::map::map::RapidMapData;


pub use crate::snap::snap::SnapMut;
use crate::tools::guard::Guard;

#[derive(Debug)]
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

pub struct RapidMap<'a, K: Send + Sync + Clone + Eq + PartialEq + Hash + Sized, V: Send + Sync + Clone + Sized> {
    map: RapidSnap<RapidMapData<'a, K, V>>
}
