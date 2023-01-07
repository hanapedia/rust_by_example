# Fearless concurrency
- concurrent programming: different parts of program execute independently
- parallel programming: different parts of a program execute at the same time
- in the book, the term *concurrency* refers to concurrent and/or parallel

## Using thread to run code simultaneously
- threads run the independent parts of the programs simultaneously
- splitting computation to multiple threads to run multiple tasks at the same time improve performance, but it also adds complexity such as
  - Race conditions, where threads are accessing data or resources in an inconsistent order 
  - Deadlocks where two threads are waiting for each other, preventing both threads from continuing 
  - Bugs that happen only in certain situations and are hard to reproduce and fix reliably
- rust uses 1:1 model of thread implementation, whereby a program uses one operating system thread per one laguage thread
  - some libraries in rust uses other models
### spawning threads 
- call `thread::spawn` with a closure
```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
- once the main process completes all thread shutsdown
- so most likely the newly spawned thread in the code above will not complete
- by sleeping the threads, it gives the os to switch threads to execute
### waiting for all threads to finish using join handles
- `thread::spawn` returns `JoinHandle` which is an owned value that, when we call `join` method on it, it will wait for its thread to finish
  - in other words, join method blocks the main thread
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); 
    // blocks the execution of main thread if called here

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // wait for the spawned thread
}
```
### Move closures with threads
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // without moving captured value
    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v); // this produces compile error
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```
- rust infers how to capture values for closures and in this case it is borrowed
- however rust cannot tell how long the spawned thread will run, so it doen't know if the refernce will always be available
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
- so move the value inside the closure with `move`
