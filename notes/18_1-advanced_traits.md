# Advanced Traits
## Specifying placeholder types in trait definitions with associated types
- Associated types connect a type placeholder with a trait such that trait method definitions can use these types in their signatures
- the implementer of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation.
- this allows you to define a trait that uses some types without needing to know exactly what those types are until trait is implemented
```rust
pub trait Iterator {
    // associated type 
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    // associated type given a concrete type in implementation
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {}
```
- associated types seems similar to generics
```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```
- with generics parameter for a trait, it can be implemented for a type multiple times, changing the concrete types of the generic type parameter each time 
- with associated type, we cannot implement a trait on a type multiple times as there can only be one `impl Iterator for Counter`

## Default generic type parameters and operator overloading
- default generic type can be defined by `<PlaceholderType=ConcreteType>`
- this is useful with *operator overloading*, in which you want to customize the behavior of an operator (such as +) 
- Rust doesn't allow you to create your own operators or overload arbitrary operators
  - but you can overload the operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator.
- default type parameters are used in two main ways
  - to extend a type without breaking existing code
  - to allow customization in specific cases most users won't need
    - `Add` uses it for second purpose
```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// this allows you to add two points with +
impl Add for Point {
    // add trait has associated type
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
// trait definition in std::ops
// has default type parameter of Self
// Self will refer to the type that you are implementing this trait on
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

// default type parameter can be overwritten when you want to use another type for rhs
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```
