# Building a multithreaded web server
- TCP is the lower-level protocol that describes the details of how inforamtion gets from one server to another but doesn't specify what that information is.
- HTTP builds on top of TCP by defining the contents of the requests and responses

## closer look at an http request 
- request takes this format
```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```
```
"GET / HTTP/1.1",
Remaning lines are headers
GET request have no body
```
- first line is the *request line* which holds information about what the client is requesting

## writing response
```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```
example with no header and no body:
```
HTTP/1.1 200 OK\r\n\r\n
```

## improving throughput with thread pool
- a thread pool is a group of spawned threads that are waiting and ready to handle a task
- having limit on the number of threads that can spawn, it becomes more resilient to DoS
- the pool will maintain a queue of request, from which threads can take to process
- other options for improving the throughput are
  - fork/join model
  - single-threaded async I/O model
  - multithreaded async I/O model

- after spawning thread with `thread::spawn`, it expects to get some code that the thread can execute
  - but with our case, we want the thread to wait for code, that will be sent when the server receives a request
  - which is not implemented in the standard library
- to implement this behavior, we have to have a data structure that manages: `Worker` 
  - Worker picks up code that needs to be run and runs the code in the Worker's thread
  - to send the code for the workers to execute, use `channels`, where each worker takes receiver
    - however `mpsc` cannot have multiple receiver 
    - we also want to send the message once so that it is executed only once
    - use `Arc<Mutex<T>>` for the receiver to share its ownership across threads, and ensure that only one thread gets to mutate it

## Graceful shutdown
- when shutting down the server, we want the threads to finish executing before dropping
- and we also don't want to take any additional requests when the server is shutting down
- we achieve this by implementing `Drop` trait on `ThreadPool`
  - drop the `sender` first so that it stops sending messages
  - then wait for each thread to complete with `join`

## Notes
- `take_while` method on iterator that takes closure, where it yields the element while the closure returns true

- when trying to design code, write the client interface first can help guide your design

- when there aren't enough system resource to create new thread, `thread::spawn` panics
  - use `thread::Builder` to get `Result` instead

- calling `lock` on `Mutex` mutex can fail when if the mutex is in *poisoned state* where the other threads paniced when they had the lock

- using `while let` for each thread results in undesired threading behavior, where slow request will still cause other requests wait to be processed
  - `let job = receiver.lock(),,,` works because any temporary values used in the expression on the right hand side is immediately dropped
  - on the other hand `while let` does not drop the temporary values until the end of the association block
    - thus the lifetime of `MutexGuard` does not carry over
  - it is extracting the message passed through channel to another variable so that lock can be returned 
```rust
        // with while let
        let thread = thread::spawn(move || {
            // lock is obtained here
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");

                job();
            }
            // lock is not dropped until the job finished

        // with loop
        let thread = thread::spawn(move || loop {
            // lock obtained 
            let job = receiver.lock().unwrap().recv().unwrap();
            // lock dropped so other threads can obtain lock

            println!("Worker {id} got a job; executing.");

            job();
        });
```

- `take` method on `Iterator` trait limits the iteration to the first n items at most
