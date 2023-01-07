# Module system
module system in rust includes:
- Packages: A Cargo feature that lets you build test and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope and privacy of paths
- Paths: A way of naming an item such as a struct, function, or module

## Packages and crates
- A crate is a smallest amount of code that Rust compiler considers at a time.
- there are two kinds of crates: Binary crates and Library crates
  - binary crates are programs that you can compile to executables
  - library crates don't. instead it provides functionalities shared across multiple projects
  - crates means library crate most of the time and they have the same meaning as 'library' in other languages
- A package is a bundle of one or more crates that provides a set of functionality
  - a package can contain as many binary crates as you like, but at most only one library crate
  - `cargo new` creates new package with one binary crate in `src/main.rs`
  - add `src/lib.rs` to add library crate
  - define mutiple binary crates in `src/bin` directory
  - in either case `main.rs` and `lib.rs` are the root of each crate

## modules, paths and `use` and `pub` keyword
### module rules
- start from the root crate: compiler first looks in the crate root file
- in crate root file, you can declare new modules; say you declare "garden" module with `mod garden;` the copiler will look for the module's code
  - inline within {} replacing ;
  - in the file `src/garden.rs` (newer style)
  - in the file `src/garden/mod.rs` (older style)
- in any other file other than the crate root, you can declare submodules the same way as modules in root. `mod vegetables;` submodule declared in garden -> `src/garden/vegetables.rs`, etc
- Once a module is part of the crate, the code can be referred from anywhere in the crate by using its Path as long as privacy allows. "Asparagus" type in vegetables can be found at `crate::garden::vegetables:Asparagus` 
- Code within a module is private from its parent modules by default. use `pub` keyword before `mod` to make the module public, and before the items to make the items within the modules public
- `use` keyword creates shortcuts to items `use crate::garden::vegetables::Asparagus;` -> `Asparagus`
```rust
// src/main.rs
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

// src/garden.rs
pub mod vegetables;

// src/garden/vegetables.rs
#[derive(Debug)]
pub struct Asparagus {}
```

## Path
- relative and absolute
- common practice is to use the absolute path
- child submodules are private to its parents by default, but the ancestors are not to their children
- use `super` keyword to reference parent module (like .. in filesystem) 
```rust
mod front_of_house { // no need to add pub as front_of_house and eat_at_restaurant are siblings
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
### public Structs and enums
- each field of a struct can be made public individually
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast { 
        // consructor associated funtion is required since the struct has a private field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```
- for enums in contrast, all of the variants of a public enum is also public

### Bringing paths into scope with the use keyword
- for bringing in functions, it is better practice to create shortcut to the module and not the function itself as it clarifies where the function is defined
- for structs and enums, create shortcuts directly
- use `as` keyword to avoid naming conflicts when bringing in two types of the same name from different modules
- imported names can also be re-exported using `pub use`
  - this allows you to write code in one structure but expose it in another

### External packages
- include in `Cargo.toml`
- bring in by `use` with crate name of the package instead of `crate` keyword

### Nested Paths and glob operator
```rust
use std::cmp::Ordering;
use std::io;
// can be reduced to
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// can be reduced to
use std::io::{self, Write};

// this brings in all public items
use std::collections::*;
```

### Extracting modules to separate files
```rust
// in single file 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// in separate files
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// src/front_of_house.rs
pub mod hosting;
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```
- must declare modules in parent using `mod` then `use`
  - this means that `mod` keyword is different from import statements in other languages
