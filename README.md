# rapidsync

Rapidsync is a combination of data stores that offers (threadsafe) interior mutability without mutable references. It aims to be as safe and performant as possible, in that order.

This library was built because we loved the flexibility of DashMap, but wanted a safer lock-based (for writes) alternative for super high-traffic systems.

Please open [an issue or pull request](https://github.com/edwardwc/rapidsync/pulls) if you have a performance suggestion, want a new feature, or experience issues.

## Stores
high performance, threadsafe stores offering stress-free simple interfaces, copy-free reads, fast writes, and mutable references - without having mutable references to the store itself
### RapidMap
the map
### RapidSnap
the cell
- atomically backed
- safe & fast locking (no std RWLock)
- reads receive "snapshots" of values (whatever the value is on read is persistent), making reads lock-free and copy-free always

## Notes, gotchas, advice
- By design, stores cannot be poisoned on reads or writes. the only way a store can be 'poisoned' is you own a mutable reference and try to write to the same object. can be solved by:
  - letting the mutable reference fall out of scope (for most users, this is good)
  - manually calling `std::mem::drop`
