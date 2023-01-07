# Closures anonymous functions that capture their environment
- closures are anonymous functions you can save in a variable or pass as arguemnts to other functions.
- Unlike functions, closures can capture values from the scope in which they're defined.
```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // this is a closure with no arguemnts
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```
- `unwrap_or_else` method on `Option` returns `Some` value or the result of the closure provdied
- closure in this example captures `self` reference

## Closure type inference and annotations
- closures don't usually require type annotations for parameters and return values, as they are not exposed interface
  - but you can add type annotations for the cost of verbosity
  - closures allows more concise definition
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
- once the closure type is inferred, the type definition is locked in, and you cannot use the same closure with different types

## Capturing references or moving ownership 
- closures can capture their environment in three ways: borrowing mutably, borrowing immutably, and taking ownership
### borrowing immutably
```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
```
### borrowing mutably
```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably(); // cannot create the mutable reference until immutable reference has gone out of scope
    println!("After calling closure: {:?}", list);
}
```
### moving ownership
- the closure can take the ownership of the values it uses in the environment even though the body of the closure doesn't strictly need ownership by using `move` keyword before paremeter list
- this technique is useful when passing a closure to a new thread to move the data so that it's owned by the new thread. more on this in concurrency
```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```
- when using threads, the main thread might finish executing before the new thread, and if the new thread did not take the ownership of the value used, the value will be cleared by the main thread. thus you need to move the ownership to the new thread

### Moving captured values out of closures and the `Fn` traits
- a closure body can do one of the following
  - move a captured value out of the closure
  - mutate the captured value
  - neither move or mutate the value
  - capture nothing from the environment to begin with
- closure implements one or more traits, which are how functions and structs can specify what kinds of closures they can use. closures automatically implement one, two, or all three of the `Fn` traits in additive fashion
1. `FnOnce` applies to closures that can called once.
  - All closures implement at least this trait, because all the closures can be called.
  - A closure that moves the captured values will implement only this trait. 
2. `FnMut` applies to closures that don't move captured values, but that might mutate the values. 
  - these closures can be called more than once
3. `Fn` applies to closures that don't move captured values and that don't mutate the values, as well as closures that captures nothing from their environment
  - these closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently
```rust
// definition of unwrap_or_else
impl<T> Option<T> {
    // f is the closure to be called by the method on None
    // f must be able to be called once, take no arguments, and return a T
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T // this is as loose a restriction can get for a closure
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
// ——————————————————————————————————————————
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    // this closure implements all three traits
    list.sort_by_key(|r| r.width);
// ——————————————————————————————————————————
    let value = String::from("by key called");
    // this closure implements only FnOnce as 'value' is moved
    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
// ——————————————————————————————————————————
    // this implements FnMut as += operation takes mutable reference
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
```

