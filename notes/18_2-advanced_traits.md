# Advanced Traits
## Fully Qualified Syntax for Disambiguation: Calling Methods with the same name
- methods in traits can have the same name as another trait's Methods
  - an both methods can be implemented on the same type
  - it's also possible to implement a method directly on the type with the same name as a method from traits
```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    // this defaults to the method implemented on Human
    person.fly();
    // to call the methods from the traits
    Pilot::fly(&person); // reference to person is required as they take self
    Wizard::fly(&person);
}
```
- for associated functions
```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // this prints "Spot" and ignores trait implemented associated function
    println!("A baby dog is called a {}", Dog::baby_name());
    // this throws error as rust cannot tell the type associated function is implemented on
    println!("A baby dog is called a {}", Animal::baby_name());
    // so use fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

## Using supertraits to require one traits functionality within another trait
- sometimes you want to write trait definition that depends on another trait
```rust
use std::fmt;

// this states that to implement OutlinePrint, Display must also be implemented on the type
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```
## Using the newtype pattern to implement external traits on external types
- the orphan rule states that we're only allowed to implement a trait on a type of either the trait of the type are local to our crate
- it's possible to get around this rule using newtype pattern
  - in which you create new type in a tuple struct
```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 refers to the Vec 
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```
- downside of this technique is that `Wrapper` is a new type, so it doesn't have the methods of the value it's holding 
  - a solution is to implement `Deref` on the `Wrapper` to return the inner type
