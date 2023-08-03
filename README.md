# rapidsync

Rapidsync is a combination of data stores that offers (threadsafe) interior mutability without mutable references. It aims to be as safe and performant as possible, in that order.

This library was built because we loved the flexibility of DashMap, but wanted a safer alternative for super high-traffic systems. We also wanted TTL for entries built-in.

Our philosophy? Locking *is* performant enough, when done right.

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
