use std::borrow::Cow;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;
use crate::{RapidMap, RapidSnap};
use crate::tools::snap::SnapRef;

#[derive(Clone)]
pub enum RapidMapData<'a, K: Send + Sync + Clone + Eq + PartialEq + Hash, V: Send + Sync + Clone> {
    Owned(HashMap<K, Arc<V>>),
    Borrowed(Cow<'a, HashMap<K, Arc<V>>>)
}

impl<'a, K: Send + Sync + Clone + Eq + PartialEq + Hash, V: Send + Sync + Clone> RapidMap<'a, K, V> {
    pub fn new() -> RapidMap<'a, K, V> {
        RapidMap {
            map: RapidSnap::new(RapidMapData::Owned(HashMap::new())),
        }
    }

    pub fn get(&self, key: &K) -> Option<SnapRef<V>> {
        let read_ref = self.map.read();

        let data = match read_ref.data.deref() {
            RapidMapData::Owned(owned_data) => {
                owned_data.get(key)
            },
            RapidMapData::Borrowed(borrowed_data) => {
                borrowed_data.get(key)
            },
        };

        match data {
            Some(t) => {
                Some(SnapRef {
                    data: t.clone(),
                })
            }
            None => None
        }
    }

    pub fn insert(&self, key: K, value: V) {
        let mut mut_ref = self.map.get_mut();

        match unsafe { &mut *mut_ref } {
            RapidMapData::Owned(owned_data) => {
                owned_data.insert(key, Arc::new(value));
            },
            RapidMapData::Borrowed(borrowed_data) => {
                borrowed_data.insert(key, Arc::new(value));
            },
        }
    }

    pub fn remove(&self, key: &K) {
        let mut mut_ref = self.map.get_mut();

        match unsafe { &mut *mut_ref } {
            RapidMapData::Owned(owned_data) => {
                owned_data.remove(key);
            },
            RapidMapData::Borrowed(borrowed_data) => {
                borrowed_data.remove(key);
            },
        }
    }

    /// Swap the map. This is the best way to remove all entries. Returns old map that will
    pub fn swap(&self) -> RapidMap<K, V> {
        let swap = self.map.swap(RapidMapData::Owned(HashMap::new()));

        RapidMap {
            map: RapidSnap::new((swap.deref()))
        }
    }

/* iterating is going to take more work
    pub fn iter(&self) -> Iter<K, Arc<V>> {
        let iter = self.map.read().iter().clone();

        iter
    }

 */
}