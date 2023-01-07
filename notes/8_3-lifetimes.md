# Validating references with lifetimes
- lifetimes ensures that the reference is valid as long as we need it to be
- lifetime annotations are required to specify the lifetimes that cannot be inferred

## Borrow checker
```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}        
```
- borrow checker compares two lifetimes and see that `r` has lifetime of `'a` but that it refers to memory with a lifetime of `'b`, thereby rejecting to compile
```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}  
```
- this works because `'b` is longer than `'a`

## Generic Lifetime in Functions
```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// longest takes two references and returns one of them
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- this does not compile because the compiler cannot tell whether the return value has the lifetime of first reference or second reference
  - the borrow checker does not know how the lifetime of `x` and `y` relates to that of the return value
  - this is fixed by using *generic lifetime parameters*
### Lifetime Annotation Syntax
- lifetime annotation *does not* change the lifetime of data 
- they describe the relationshiops of the lifetimes of multiple references to each other without affecting the lifetimes
```rust
let i: &i32        // a reference
let i: &'a i32     // a reference with an explicit lifetime
let i: &'a mut i32 // a mutable reference with an explicit lifetime
```
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // using result here works
    }
    // result has the same lifetime as string2
    println!("The longest string is {}", result); // using result here does not copile
}
```
- this tells the compiler that the lifetime of the return value is same as the shortest lifetime of the values refered to by the arguments
- a function cannot return a reference unless it has the lifetime parameter that matches the lifetime parameter of the arguments.  
  - a function cannot return a reference unless it recieves a reference from the caller

## Lifetime Annotations in Struct Definitions
- for structs to hold references in their field, lifetime annotation must be used
- it ensures that the struct does not  live longer than the value that the reference in one of its field points to.
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // ImportantExcerpt will have the same lifetime as first_sentence 
}
```

## Lifetime Elision rules
- if your code fits these cases, you don't have to write the lifetimes explicitly
- the compiler will check if these rules are satisfied when given no lifetime annotations
1. The compiler assigns a lifetime parameter to each parameter of the references
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters

## Lifetime Annotations in Method Definitions
```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
// return value has same lifetime as self
    fn announce_and_return_part(&self, announcement: &str) -> &str { 
        println!("Attention please: {}", announcement);
        self.part
    }
}

```

## Static lifetimes
- static lifetimes denotes that the affected reference can live for the entire duration of the program
## Generic type parameters, trait bounds, and lifetimes together
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
