# Reference and borrowing

- a reference is like a pointer, an address to a value stored on the heap; that is owned by some other variable
  - this means that the ownership of the value does not move
  - this is called borrowing
- unlike pointer, reference is guaranteed to point to a value

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // passes down the reference to s1

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");

    change(&s); // this throws an error as references are immutable by default just like variables
}

fn calculate_length(s: &String) -> usize { // takes reference to a String
    s.len() // the value tha s points does not get dropped as s does not own it
}
fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
## mutable reference
- references are immutable by default just like any other variables in rust
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);// this does not throw error
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
- mutable referece has one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
  - it does not matter if the second reference is mutable or not
  - *there are no restrictions on immutable references*
- this prevents data races at compile time. data race is similar to a race condition and happens when theses three behaviors occur:
  - two or more pointers access the same data at the same time.
  - At least one of the pointers is being used to write the data.
  - There's no meachanism being used to synchronize access to the data.

This code throws an error:
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // this throws an error

    println!("{}, {}", r1, r2);
}
```
but this does not 
```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```
compiler can tell when a variable goes out of scope without reaching the end of a code block
```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    // so the compiler treats r1 and r2 as out of scope, they are dropped
    let r3 = &mut s; // no problem
    println!("{}", r3);
```

## dangling references
- rust compiler makes sure that there are no dangling references by ensuring that a data will not go out of scope before its reference does.
