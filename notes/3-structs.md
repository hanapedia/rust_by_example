# Structures
## Definition and initialization
```rust
// definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // initialization
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@example.com"); // fields can be changed if the isntance is mutable

    let user2 = User { // this moves data in user1 to user2, meaning user1 can no longer be used
        email: String::from("another@example.com"),
        ..user1 // struct update syntax
    };
}

// using builder functions
fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand syntax
        username, // shorthand syntax
        active: true,
        sign_in_count: 1,
    }
}
```
- To change the fields, entire instance must be mutable, rust does not allow individual fields to be mutable

### tuple structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
- named tuple types
- black and orign are of different types even though they have same set of types in tuple

### unit like structs
- structs without any fields
- useful when you want to implement a trait on some type but don't have any data that you want to store in the type itself

## Ownership of struct data 
- in the example above each data of the fields of the structs were owned by the instance of the struct.
- it is possible to assign data owned by other variable to fields of the struct using references and lifetime(Chapter 10)
  - lifetime ensures that the data of the field does not go out of scope until the struct that uses it does

## formatting and printing structs
```rust
// add this outer attribute to be able to print structs in debug format
// this derives Debug trait
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); 
    // {:?} formats to debug
    // {:#?} for prettier print for larger structs 
    // dbg! macro prints the content to stderr instead
      // it olso takes ownership of the variable and returns it 
      // println only takes reference
}
```

## Methods
- methods are defined within the context of a struct(or enum or a trait object), 
- rust automatically dereferences if the method is called on the reference of the struct
  - in other words rust makes borrowing implicit for method calls
- there can be multiple imple blocks for a struct type
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // first parameter must always be reference to self
    // self is a alias for self: &Self
    // &Self is equibriant to the type of the impl block is for
    // self can be immutable or mutable if you want to change the struct instnce
    fn area(&self) -> u32 { 
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
### Associated functions 
- All functions within an impl block are called associated functions 
- you can define associated functions(not method) that do not take self as a parameter
  - useful for defining constructor functions for the struct
  - to call associated functions, use `::` instead of `.`
```rust
impl Rectangle {
    fn square(size: u32) -> Self { // does not take self as a param
        Self {
            width: size,
            height: size,
        }
    }
}
```
