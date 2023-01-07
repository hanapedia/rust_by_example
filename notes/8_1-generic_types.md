# Generic types
- generics are used to create definitions for items like function signatures or structs
- main benefit is in reducing the amount of duplicate code
- parameterizes types
  - follows CamelCase naming convention
## Function definition
```rust
// duplicate
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// duplicate
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// reduced to 
fn largest<T>(list: &[T]) -> &T {                          // this does not compile
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { // this compiles
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// this is still invalid as we want to compare the values of type T, 
// so we can only use types whose values can be oredered.
// to enable comparison, types must implement std::cmp::PartialOrd trait
```

## Struct definition
```rust
// single type
struct Point<T> {
    x: T,
    y: T,
}
// multiple type
struct Point2<T, U> {
    x: T,
    y: U,
}

// method definition
impl<T> Point<T> { // <T> after impl is required to imply that <T> after Point is a generic type instead of a concrete type
    fn x(&self) -> &T {
        &self.x
    }
}
// method definiton for a specific type of a generic type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
- generics can also be used for enum definition
- Generic type parameteres in a struct definition aren't always the same as those you use in that same struct's method signatures, for example:
```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2); // mixup can takes a point of differnt type than that of the first

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```
## Generics performance
- generics does not hinder the performance of the code at runtime 
- at compile time, it generates code with concrete types inserted in place of generics
