# Adavanced Types
## Using the newtype pattern for type safety and abstraction
- newtypes can be used to abstract away some implementation details and internal implementation of the type
## Creating type synonyms with type alias
- type alias gives an exisiting type another name
- alias makes it easier to manage longer types
- also gives us consistent interface
```rust
  type Kilometers = i32;
  let x: i32 = 5;
  let y: Kilometers = 5;

  // for long types
  type Thunk = Box<dyn Fn() + Send + 'static>;

  // with generics
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
// repetetive Result<.. Error> can be aliased as
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```
## Never type that never returns
- rust has special type named `!`, which has no value
- it stands in the place of the return type when a function will never return 
```rust
fn bar() -> ! {}

        // match arms must return the same type
        // continue returns !
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```
## Dynamically sized types and the Sized trait
- Dynamically sized types or unsized types let us write code using values whose size we can only know only at runtime
- egs. `str` *not `&str`&*
  - we can't know how long the string is until runtime, meaning we can't create a variable of type `str`, nor can we take arguments of type `str`
```rust
    // this does not work
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
```
  - to fix this, use `&str` which stores the address of the str and its length, which we know the size of
- dynamically sized types must be put behind a pointer of some kind
- traits are also dynamically sized type, thus in order to use trait object, you have to put it behind a pointer

- rust provides `Sized` trait to determine whether or not a type's size is known at compile time, this trait is implemented automatically for types with known size
- by default generic functions will work only on types that have a known size at compile time
```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
// using ? before Sized trait allows you to use unsized type in generics
// then you have to use & or some other kind of pointer
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```
