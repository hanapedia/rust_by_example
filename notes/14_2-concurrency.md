# Fearless concurrency
## Using message passing to transfer data between threads
- One increasingly popular approach to ensuring safe concurrency is *message passing*, whre thread or actors comminicate by sending each other messages containg data.
  - "Do not communicate by sharing memory; instead, share memory by communicating"
- rust uses channels
  - channels consist of two halves: a transmitter and receiver, transmitter sends message to receiver
  - a channel is said to be closed if either the transmitter or receiver half is dropped
### creating channel
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // thread needs to own transmitter to send messages
        tx.send(val).unwrap();
        // send returns Result<T, E>
        // send returns Err when the receiver has already been dropped
    });
    
    let received = rx.recv().unwrap();
    // recv returns Result<T, R>
    // Err when transmitter closes
    println!("Got: {}", received);
}
```
- `mpsc` stands for *multiple producer, single consumer*
- `mpsc::channel` returns tuple containing transmitter(`tx`) and receiver(`rx`)
- `recv` blocks the execution of the thread where it is called 
- `try_recv` does not block the thread and returns `Result` immediately
  - `Ok` if a message is available
  - `Err` if there aren't any messages 
### channels and ownership transference
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val); // produces compile error
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
- `send` functions takes ownership of its parameter, and when the value is moved, the receiver takes the ownership of it
### sending multiple values and seeing the receiver waiting
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receiver can be used as an iterator
    // which iterates each time it receives a message
    for received in rx {
        println!("Got: {}", received);
    }
}
```
### Creating multiple producers by cloning the transmitter
```rust
    // --snip
    let (tx, rx) = mpsc::channel();

    // clone transmitter
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    // --snip
```
