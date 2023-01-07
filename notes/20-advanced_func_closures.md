# Advanced functions and closures
## Function pointers
- functions coerce to type `fn`, which is called a *function pointer*
- passing functions with function pointers will allow you to use functions as arguments to other functions
```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

// takes function pointer to any functions that takes i32 and returns i32
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```
- function pointers implement all three of the closure traits(`Fn`,`FnMut`,`FnOnce`)
  - meaning you can always pass a function pointer to functions that take closures
```rust
    let list_of_numbers = vec![1, 2, 3];
    // with closures
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    // with function pointers
    let list_of_strings: Vec<String> =
        // need to use fully qualified name as there are many to_string available
        list_of_numbers.iter().map(ToString::to_string).collect();
```
- name of each enum variant becomes an initializer function, which can also be uses as function pointers
  - thus they can also be used in place of closures
```rust
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

## Returning closures
- closures are represented by traits, which means you can't return closures directly
- In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function. 
- this does not work with closures because they don't have a concrete type that is returnable
  - you're not allowed to use the function pointer `fn` as a return type
```rust
// this won' compile, as the compiler does not know how much size the closure needs
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
// instead, use trait object to put it behind a pointer
// this can return any closure with Fn(i32) -> i32
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```
