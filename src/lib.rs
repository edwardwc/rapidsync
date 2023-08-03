mod cell;
mod vars;
mod snap;
mod lock;

use std::cell::UnsafeCell;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, AtomicU8};

/// RapidSnap is a snapshot-based atomically-backed cell with memory-free reads
pub struct RapidSnap<T> {
    guard: AtomicU8,
    data: UnsafeCell<Arc<T>>,
}


pub struct RapidCell {

}

pub struct RapidMap {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
