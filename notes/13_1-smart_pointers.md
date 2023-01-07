# Smart pointers
- smart pointers are data structures that act like a pointer but also have additional metadata and capabilities.
- smart pointers implements `Deref` and `Drop` traits
## Using Box<T> to point to data on the heap
- boxes allows you to store data on the heap rather than the stack. what remains on the stack is the pointer ot the heap data.
- it most often used in one of these situations:
1. When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
2. When you have a large amount of data and you want to transfer the ownership but unsure the data won't be copied when you do so.
3. When you want to own a value and you care only that it's a type tha implements a particular trait rather than being of a specific type. (Chapter 17)

- to store data using `Box<T>`
```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```
- the deallocation happens when the Box goes out of scope and the pointer and the data are cleared
- Boxes also does not pose any performance overhead

### Recursive types with Boxes
- a value of a recursive type include a value of the same type as part of itself
- this poses an issue because Rust needs to know how much space a type takes up at compile time
  - for enums, its size is determined by the biggest size of entries. If the enum itself is included as one of the entries, the compiler cannot determine the size of an instance 
  - so instead of including the type itself as one of the entries, you can include the pointet to the type as an entry. this ensures the size of an instance of the enum
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

## Treating smart pointers like regular references with the Deref Trait
- implementing the `Deref` trait allows you to customize the behavior of the *derefrence operator* `*`
- a regular reference is a type of pointer, but their dereferencing defers from that of smart pointers

### Defining our own smart pointer
- similar to standard library's `Box<T>`
```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // this won't compile since rust doesn't know how to dereference MyBox
}
```
#### Impelementing Deref
```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // associated type
    type Target = T;

    // borrows self and returns a reference
    fn deref(&self) -> &Self::Target {
        &self.0 // first value in a tuple struct
    }
}
```
- without `Deref` trait, the compiler can only dereference & references. The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a & reference that it knows how to dereference.
- essentially it is running 
```rust
*(y.deref())
```
### Implicit Deref coercion with functions and methods
- Deref coercion converts a refrence to a type that implements the `Deref` trait into a reference to another type. `&String` to `&str`
- it is performed on arguments to functions and methods

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); 
    hello(&(*m)[..]); // without deref coercion
}
```
#### with mutability
- `DerefMut` trait can be used to overwrite the `*` operator on mutable references
- Rust does deref coercion when it finds the types and trait implentations in three cases:
  - From `&T` to `&U` when `T: Deref<Target=U>`
  - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
  - From `&mut T` to `&U` when `T: Deref<TargetMut=U>`
- rust can coerce mutable reference to immutable but not vice-verca

## Running code on cleanup with the `Drop` Trait
- allows you to customize what happens when a value is about to go out of scope
- `Drop` trait requires you to implement method named `drop` which takes the mutable reference to self as an argument
- `Drop` trait is included in the prelude
```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```
- `drop` method cannot be called manually as rust will automatically call this method at the end of the scope, which will cause double *free error*
  - in case you want to drop the value early, use `drop` *function* provided by std
    - dropping values early can be useful when working with smartpointers that manage locks
