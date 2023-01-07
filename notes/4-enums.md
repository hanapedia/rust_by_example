# Enums
- enums allows you say a value is one of a possible set of values
```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1")); 

    fn route(ip_kind: IpAddrKind) {} // can take both v4 and v6

```
- data can be attatched to each variants of enums by defining types in ()
  - structs can also be used in enum types, or even another enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
    impl Message { // methods can also be defined
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```
## Options enum
[Option doc](https://doc.rust-lang.org/std/option/enum.Option.html)
- when a value could be something or nothing
  - rust does not have the feature of null unlike other languages
  - the feature of null is addressed by Options enum
  - this let's you work with "null" like value only when Options enum is used
  - in other words, anywhere there isn't Option enum, you can safely assume that value will always be there
```rust
enum Option<T> {
    None,
    Some(T),
}
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; //this throws an error as the compiler will not treat i8 and Option<i8> as same type
```

## match controll flow
- compare a value against series of patterns and then execute code based on which pattern matches
  - patterns can be literal values, variable names, wildcards, and many other things
- it checks the arms in order and if the first match is found, no other code is executed
- must exhaust every last possibility
  - catch-all with variable at the end for arbitral number of options
```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // can access parts of the value that matches pattern
            println!("State quarter from {:?}!", state);
            25
        }
    }
}```

### with placeholder
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // for any other values. variable can be named freely. use _ if you don't need the value
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```

### with Option
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```
## if let control flow
- used when you want to match one particular pattern while ignoring the rest
  - pattern is same as a match arm
```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // is equibrant to 
    let config_max = Some(3u8);
    // pattern = expression
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        // handle rest of the patterns
        // same as pattern _ in match
    }
```
