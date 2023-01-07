# Ownership
## stack and heap
stack:
- stack is memory that stores values in last-in first-out manner.
- all the data stored on stack must have known fixed size at compile time.
  - data with unknown or unfixed size must be stored in heap

heap:
- when adding data to the heap, you are requesting a certain amount of space in memory. In return memory allocator returns pointer to the memory that was allocated

comparison:
- pushing data to stack is faster than storing data in stack as it does not look for an empty spot in memory and it just stores the data at the top of the stack.
- reading data from stack is faster than reading from the heap as you don't have to follow the pointer.

usage:
- when a function is called, the arguments and variables inside of the functions are moved to the stack. and after the execution is complete the data is popped off the stack.
- the main purpose of Ownership in rust is to manage heap data

## Ownership rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
  - this ensures a single memory deallocation for a single allocation.

### moving the data on the heap
```rust
    let s1 = String::from("hello");
    let s2 = s1;
```
- this only copies the reference to the data on the heap and not the data itself
  - however when both of the variables goes out of scope, the memory will be cleared twice, causing double free error.
  - to prevent this error, after re-assigning the reference to the data on the heap, rust invalidates the first reference
  - for this, it is not called shallow copy but *move*
```rust
let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // this throws an error
```
- rust never automatically creates deep copy of the data on heap
  - if you want a deep copy, run `.clone()` 

### functions
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

```
- a variable becomes unavailable for use after passed to a function, as it moves out of scope and into that of the function. in other words, the ownership of the data changes
- a variable returned by a function also changes ownership from the function and the caller
- assigning a value to another variable moves it. the value is dropped when the variable goes out of scope unless the ownership of the data has moved
- moving and moving back the ownership of a variable works but becomes tedious when you want to return multiple variables, etc
  - for this rust has the concept of references and borrowing
