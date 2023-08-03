# rapidsync

## Have you ever yearned to do mutable threadsafe storage eloquently?
Rapidsync is a combination of data stores that offers (threadsafe) interior mutability without mutable references. It aims to be as safe, as performant, and as flexible as possible, in that order.

This library was built because we loved the flexibility of DashMap, but wanted a safer approach for super high-traffic systems.

**This library is in alpha. Help make it better! Open [an issue or pull request](https://github.com/edwardwc/rapidsync/pulls) if you have a performance suggestion, want a new feature, or experience issues.**

## Stores
high performance, threadsafe stores offering stress-free simple interfaces, copy-free reads, fast writes, and mutable references - without having mutable references to the store itself
### features
- locks cannot be poisoned unless you hold a mutable reference and try to read/write
  - on debug builds, rapidsync automatically notifies you if a deadlock is detected
- memory safety in all conditions because of fast performance without splitting trees
- no external dependencies
- atomically backed
- safe & fast locking - no std RwLock

### [RapidSnap - the cell](https://docs.rs/rapidsync/latest/rapidsync/struct.RapidSnap.html)

### RapidMap
the map - coming soon

## Notes, gotchas, advice
- By design, stores cannot be poisoned on reads or writes. the only way a store can be 'poisoned' is you own a mutable reference and try to read/write to the same object. can be solved by:
  - letting the mutable reference fall out of scope (for most users, this is good)
  - calling rust default function `drop`
