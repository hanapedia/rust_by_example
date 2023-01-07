# Error handling
## Unrecoverable errors
- when an unrecoverable errors occurs, rust panics
  - panic can be produced artificially using panic! macro
  - by default the program starts unwinding to clean up the data used by the program, this behavior can be expensive. thus you can choose to abort instead by configuring in `Cargo.toml`.
```rust
fn main() {
    panic!("crash and burn");
}
```
### Panic backtrace
- set RUST_BACKTRACE environment variable to anything other than 0 to enable backtracing

## Recoverable errors
- most errors aren't serious enough to require the program to stop entirely.
- use `Result` enum to handle such errors
```rust
enum Result<T, E> { // T is the type of value and E is the type of error
    Ok(T),
    Err(E),
}
```
### error handling using match
```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt"); // returns Result enum

    let greeting_file = match greeting_file_result { 
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```
### maching different errors
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt"); 

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() { // error is io::Error
            ErrorKind::NotFound => match File::create("hello.txt") { // matches not found 
                Ok(fc) => fc, // another match for create
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```
### use closures to reduce the number of match statements
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```
### shortcuts for panic on error: `unwrap` and `expect`
```rust
use std::fs::File;

fn main() {
// returns value if ok, else panic
    let greeting_file = File::open("hello.txt").unwrap();
// returns value if ok, else panic with me message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```
### Propagating errors
- instead of handling error inside the function itself, it can return the error to the caller so that caller can decide what to do with the error
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```
#### shortcut for propagating errors: ? operator
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // If the value is Err, the function returns here
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // same here
    Ok(username)
}

// chaining makes it even shorter
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
// reading a file into a string is so common that standard library provides the function 
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```
- the differnce between `match` and `?` operator is that `Err` value will go through `from` function for `?` operator, meaning the error type going into the operator will be converted to the error type of the function
- to use `?` operator, the function must have compatible return type: `Result`, `Option`, etc
  - to use `?` with `Option`, the function must return `Option` and function to apply the operator must also return `Option` just like the case with `Result`
#### Returning error from main function
main function allows you to return `Result<(), E>`, so this works
```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { // 
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```
- `Box<dyn Error>` is a trait object, which basically means "any kind of error"
- main function may return any types that implement `std::process::Termination` trait

