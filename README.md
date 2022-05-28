# rust-experiments

## Resources
- [TheDevMethod: Rust YT tutorials](https://www.youtube.com/c/TheDevMethod)


## Notes
### Arc (Atomically Reference Counted)
> For thread-safe referencing of a single variable. 
- When wanting to share a value between multiple threads, use the `Arc` library. `Arc<T>` provides shared ownership of a value of type `T`, allocated in the heap. `clone` produces a new `Arc` instance which points to the same allocation on the heap as the source `Arc`, while increasing a reference count. The only way to mutate through an `Arc` is to use `Mutex`, `RwLock` or one of the `Atomic` types. If you are not sharing reference-counted allocations between threads, consider using Rc<T> for lower overhead (less expensive).

### Mutex (Mutual Exclusion Primitive)
> For protecting shared data
- Blocks threads waiting for the lock to become available. Data can only be accessed through the RAII guards returned from `lock` and `try_lock`, which guarantees that the data is only ever accessed when the mutex is locked.

### Atomic (types)
> Primitive shared-memory communication between threads, and are the building blocks of other concurrent types.
- This module defines atomic versions of a select number of primitive types, including `AtomicBool`, `AtomicIsize`, `AtomicUsize`, `AtomicI8`, `AtomicU16`, etc. Atomic types present operations that, when used correctly, synchronize updates between threads.