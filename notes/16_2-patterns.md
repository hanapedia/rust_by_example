# Patterns and matching
## Pattern Syntax
### matching literals
```rust
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```
### matching named variables
```rust
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // this shadows y, meaning this is a new variable
        // different from y from outer scope
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    // this y holds 10
    println!("at the end: x = {:?}, y = {y}", x);
```
- `match` starts a new scope
- to create a `match` expression that compares the values of the outer `x` and `y`, rather than introducing a shadowed variable, we would need to use a `match` guard conditional instead.
### multiple patterns
```rust
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // matches if either 1 or 2
        3 => println!("three"),
        _ => println!("anything"),
    }
```
### matching ranges of values with ..=
```rust
    let x = 5;

    match x {
        1..=5 => println!("one through five"), // includes 5
        _ => println!("something else"),
    }
```
- allowed only with `char` or numeric
### destructuring to break apart values
#### structs
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // destructures to a and b
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // shorthand Syntax to destructure to name of the fields
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    // can use literal values as part of the struct pattern
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"), // matches when y is 0
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```
#### enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        // struct-like enums
        Message::Move { x, y } => {  
            println!(
                "Move in the x direction {x} and in the y direction {y}"
            );
        }
        // tuple like
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {r}, green {g}, and blue {b}",
        ),
    }
}
```
#### nested structs and enums
```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        // destructured using nested parenthesis
        // if color in changecolor is of rgb 
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        // if color in changecolor is of hsv
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change color to hue {h}, saturation {s}, value {v}"
        ),
        _ => (),
    }
}
```
#### structs and tuples
- can destruct any complex types into their component
```rust
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```
