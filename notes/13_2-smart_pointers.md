# Smart pointers provided by the standard library
## Rc<T>, the reference counted smart pointer
- There are cases where a single value might have multiple owners.
  - egs. in graph data structure, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node should not be cleaned up unless it doesn't have any edges pointing to it and so has no owners
- in such case, `Rc<T>` is used
  - it keeps track of the number of references to a value to determine whether or not the value is still in use.
  - it is used when we want to allocate some data on the heap for multiple parts of our program to read and we can't decide at compile time which part will finish using data last
    - if we knew which part finishes last, we can just make that part the owner
  - *Note that `Rc<T>` is only for use in single-threaded scenarios*

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); // this throws an error as a has been moved to b
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc; // bring rc into scope

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a)); // this works and now b and c shares ownership of a
}
```
- `Rc::clone` does not create the deep copy of the value
- `Rc::strong_count` returns reference count of the moment
- there is also `Rc::weak_count`

## RefCell<T> and the interior mutability pattern
- interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.
  - the pattern uses `unsafe` code inside to bend Rust's usual rules
- you can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime, even though the copiler can't guarantee that

### Enforcing borrowing rules at runtime with RefCell
- `RefCell<T>` enforces borrowing rules at runtime instead of compile time, if you violate the rules, the program will panic and exit
  - the advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, where they would've been disallowed by the compile time checks
  - *Note that `RefCell<T>` is only for use in single-threaded scenarios*

### Interior mutability: A mutable borrow to an immutable value
- without `RefCell` an immutable value cannot be borrowed mutably
- it allows you to mutate the inner values of an immutable type
  - egs. mutating a field of a immutable struct
#### use case for interior mutability: Mock Objects
- mock objects are used as a placeholder types during testing, it records what happens during a test so you can assert that correct actions took place
- `RefCell` keeps track of number of immutable and mutable references
- refer [15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) for example

### Having multiple owners of mutabale data by combining Rc and RefCell
```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    // value is passed as immutable reference
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```
