# Patterns and matching
## patterns syntax
### Ignoring values in a pattern
#### Ignoring an entire value with _
- `_` can be used to ignore entire value in any pattern
- can be used to avoid compiler warnings about unused function parameters
```rust

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    // 3 is ignored entirly
    foo(3, 4);
}
```
### ignoring parts of a value with a nested _
- when you want to test only part of a value
```rust
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // runs only setting_value and new_setting_value are both of Some
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // ignore parts of tuple
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
```
### ignoring an unused variable by starting its name with _
```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```
- this is different from `_` as using something like `_x` actually defines `_x`
```rust
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // this throws error because the value in s is moved to _s
    println!("{:?}", s);
```
### ignoring remaining parts of a value with ..
```rust
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        // ignores y and z
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // this throws error as you can only use one .. per tuple pattern
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
```
## Extra condtionals with match guards
- additional if statement after the pattern in `match` arm
- condition can use variables created in the pattern
- the downside is that the compiler does not check exhaustiveness
```rust
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // use outer y for condition
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    match x {
        // (4, 5, or 6), and if y 
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
```
### @ bindings
- create a variable taht holds a value at the same time as we're testing that value for a pattern match
```rust
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // will be able to use value in id field
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // won't be able to use id in the code within the arm
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // will be able to use value in id field as it does not apply any tests
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
```
