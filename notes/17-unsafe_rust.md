# Unsafe rust
- unsafe rust exists because static analysis is conservative by nature
- another reason is that the underlying computer hardware is inherently unsafe
  - if rust didn't let you do unsafe operations, you couldn't do certain tasks

## Unsafe Superpowers
- unsafe rust includes the ability to:
  - dereference a raw pointer
  - call an unsafe function or method
  - Access or modify a mutable static variable
  - implement an unsafe trait
  - access fields of `union`s
- unsafe does not turn off borrow checker or disable any other of rust safety checks
- limiting the use of these five features only within `unsafe` blocks, allows you to know that any memory related errors must have come from the block 
### Dereferencing a Raw Pointer
- written as `*const T` and `*mut T`, can be immutable or mutable
- different from references and smart pointers in ways that raw ponters:
  - are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
  - aren't guaranteed to point to valid memory
  - are allowed to be null
  - don't implement any automatic cleanup
```rust
    // you can define raw pointer within safe code 
    // but cannot dereference it
    let mut num = 5;

    // allows immutable and mutable at the same time
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1); // dereference
        println!("r2 is: {}", *r2);
    }
```
- the use cases of raw pointers are 
  - when you want to interact with C  
  - when building up safe abstractions that borrow checker doesn't understand
### Calling Unsafe Function or Method
- unsafe functions and methods have `unsafe` keyword, 
  - which indicates that the function has requirements that we need to uphold whewe call this function, because rust can't guaranteed we've met these requirements
  - by calling an unsafe function within an `unsafe` block, we're saying that we've read this funcion's documentaion and take responsibility for upholdin the function's contracts
- bodies of unsafe functions are effectively `unsafe` block
#### Creating a safe abstraction over unsafe code
- just because a function contains unsafe code, doesn't mean we need to mark the entire function as unsafe. 
  - egs. `split_at_mut` method, which splits a slice at given index
```rust
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
```
Implementation example of `split_at_mut` 
```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    // though it is fundamentally okay to borrow two sections of slices that doesn't overlap,
    // borrow checker cannot tell that it doesn't overlap
    // disallowing you to create two mutable references to a slice
    (&mut values[..mid], &mut values[mid..])

    // with unsafe
    // from_raw_parts_mut and add are unsafe as they require raw pointers
    let ptr = values.as_mut_ptr(); // returns raw pointer
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            // add gives new pointer at mid
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```
#### Using extern functions to call external code
- `extern` keyword facilitates the creation an use of *Foreign Function Interface*(FFI), 
  - which allows programming languages to define functions and enable a different programming languages to call those functions
- calling C code
```rust
// "C" part defines which application binary interface to use
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
- defining Rust for C
```rust
// no_mangle tells the compiler not to mangle the name of the function
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

### Accessing or Modifying a mutale static variable
- in rust, global variables are called *static* vairable, which can be problematic with rust;s ownership rules
- difference between constants and static variables
  - value in static variable have a fixed address in memory, using the value will always access the same data
  - constans are allowed to duplicate their data whenever they are used
  - static variables can be mutable, thus accessing and modifying mutable static variables is unsafe
- mutable global variable can cause data races, thus rust considers them as `unsafe`
```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

### Implementing an unsafe trait
- a tra is unsafe when at least one of its methods has some invariant that compiler can't verify
```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```
### Accessing Fields of a union
- a union is similar to struct, but only one declared field is used in a particular instance at one time.
- primary used to unterface with unitions in C code

## When to use unsafe
- Using `unsafe` to take one of the five actions isn't wrong or even frowned upon
