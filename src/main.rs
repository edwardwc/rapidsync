#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;
use rapidsync::{RapidMap, RapidSnap, SnapMut};

lazy_static!(
    static ref TESTER: RapidSnap<String> = RapidSnap::new("hey there cool beans".to_string());
);

fn main() {
    let map = RapidMap::new();

    map.insert("hey there", "wow hello!");

    println!("{:?}", map.get(&"hey there"));
/*
    for n in 0..100 {
        for i in 0..10 {
            std::thread::spawn(move || {
                let v = i.to_string();
                TESTER.swap(v.clone());
                let mut h = TESTER.get_mut();
                *h = "hey jude".to_string();
                let _ = TESTER.read();
            });
        }
        println!("{:?}", TESTER.read().data);
        // println!("{}", n);
    }

    sleep(Duration::from_secs(1));


 */
    println!("done");
}