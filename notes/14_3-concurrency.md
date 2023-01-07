# Fearless concurrency
## Shared-state concurrency
- it communicates by sharing memory
### Using mutexes to allow access to data form one thread at a time
- *Mutex* is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at given tiem.
- to accesss the data in mutex, a thread must first signal that it wants access by asking to acquire the mutex's *lock*
- the lock is a data structure that is part of the mutex that keeps track of who currently has exclucsive access to the data.
- mutex is described as *guarding* the data it holds via the locking system

- mutex has reputation of being difficult to use because you have to remember two rules:
  - You must attempt to acquire the lock before using the data
  - When you're done with the data that the mutex guards, you must unlock the data so other thread can acquired the lock

### the api of `Mutex<T>`
- `Mutex<T>` is a smart pointer
- the call to `lock` returns a smart pointer called `MutexGuard`, wrapped in a `LockResult` that we handle with the call to `unwrap`
- `MutexGuard` smart pointer implements `Deref` to point to inner data
- and `Drop` that releases the lock automatically when a `MutexGuard` goes out of scope
  - thus we don't risk forgetting to release the lock
```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // acquire lock by lock method
        // must call lock to be able to use the value
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

### sharing a Mutex<T> Between Multiple threads
- `Rc` cannot be shared safely across threads, thus you cannot use it for shared ownership 
- so we need a type exactly like `Rc` but one that makes changes to the reference count in a thread-safe way.
### Atomic reference counting with Arc<T>
- `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.
- the *a* stands for *atomic*
- all the primitive types aren't atomic as ensuring atomicity comes with performance penalty
- `Arc` and `Rc` have the same API
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Mutex provides interior mutability
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
- if you are doing simple numerical operations, there are types simpler than `Mutex` types provided by the `std::sync::atomic` module
- these type provide safe, concurrent, atomic access to primitive types. 

### similarities between RefCell/Rc and Mutex/Arc
- `Mutex` provides internal mutability like `RefCell`, so you can mutate contents inside `Arc`
- similar to reference cycles, `Mutex` comes with the risk of creating *deadlocks* 
  - they occur  when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.

## Extensible concurrency with the Sync and Send traits
- rust has very few concurrency features. Almost all the features used in this chapter is part of the standard library
- two concurrency concepts are embedded in the language
  - `std::marker` traits `Sync` and `Send`
### Allowing transference of ownership between threads with Send
- the `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. 
  - almost every type in Rust is `Send`
  - `Rc` does not implement `Send`
  - Any type composed entirely of `Send` type is automatically marked as `Send`
  - Almost all primitive types are `Send`, except raw pointers
### Allowing access from multiple threads with Sync
- the `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. 
  - any type `T` is `Sync` if `&T`(an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread.
  - Any type composed entirely of `Sync` type is automatically marked as `Sync`
  - all primitive types are `Sync` 
  - `Rc` does not implement `Sync`
  - `RefCell` does not implement `Sync` and the family of related `Cell` types
### Implementing Send and Sync manually is unsafe
- because the types that are made up of `Send` and `Sync` traits are automatically `Send` and `Sync`, we don't have to implement those traits manually. 
- As marker traits, they don't even have any methods to implement. 
- they are just useful for enforcing invariants related to concurrency

## summary
- many concurrency solutions are implemented as external crates
