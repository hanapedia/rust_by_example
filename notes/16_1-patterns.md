# Patterns and matching
A pattern consists of some combination of the following:
- Literals 
- Destructured arrays, enums, structs or tuples
- variable
- wildcards
- placeholders

## All the places patterns can be used 
### match arms
```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```
- one requirement for match expressions is that they need to be exhaustive in the sense that all possibilities for the value in the `match` expression must be accounted for
### conditional if let expressions
- rust doesn't require that the conditions in a series of `if let`, `else if`, `else if let` arms to relate to each other unlike the `match` statement
- one downside of using `if let` is that the compiler doesn't check for exhaustiveness
```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

### while let conditional loops
- `while let` conditional loop allows a `while` loop to run as long as a pttern continues to match.
```rust
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // while loop runs as long as pop returns Some
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
```

### for loops
- the value that directly follows the keyword `for` is a pattern.
  - For example, in `for x in y` the `x` is the pattern
```rust
    let v = vec!['a', 'b', 'c'];

    // the tuple returned by emurate method is destructured using pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
```

### let statements
```rust
let x = 5;
// this is also using a pattern
let PATTERN = EXPRESSION;
```
- `x` is a pattern that means "bind what matches here to the variable x"
- because the name `x` is the whole pattern, this pattern effectively means "bind everyting to the variable x, whatever the value is"
```rust 
    let (x, y, z) = (1, 2, 3);
```

### Function parameters
```rust
fn foo(x: i32) {
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```
- The `x` part is a pattern.
  - As we did with `let`, we could match a tuple in a function's arguments to the pattern. 
- you can also use patterns in closures parameters list


## Refutability 
- pattern comes in two forms: refutable and irrefutable
  - patterns that will match for any possible values are irrefutable
    - `let x = 5;`
    - function parameters, `let`, `for`
    - `if let`, `while let` with warnings from the compiler, as they are intended for refutable patterns
  - patterns that fail to match for some possible value are refutable
    - `if let Some(x) = a_value`, when `a_value` is `None` it does not match 
    - `if let`,`while let`, `Some(x)`
```rust
// this will throw error as your trying to use refutable pattern 
// in place of irrefutable pattern
  let Some(x) = some_option_value;
  // instead
  if let Some(x) = some_option_value {
      println!("{}", x);
  }
  // however this gives warning as youre trying to use irrefutable pattern 
  // in place of refutable pattern
  if let x = 5 {
      println!("{}", x);
  };
```
