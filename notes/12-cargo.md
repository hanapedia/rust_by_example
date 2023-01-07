# More about Cargo and crates.io
## Realease profiles
- Rust has release profiles which are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.
```
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```
- profiles can be configured in `Cargo.toml` in `[profile.*]` section
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
- opt-level refers to optimization levels, which can be set from 0 to 3
 - higher optimization results in longer compilation time

## Publishing a crate to Crates.io
### Documentation comments
- use `///` to add Documentation comments which generates HTML Documentation
  - the documentation is generated using `cargo doc` command, and `--open` flag which will open it in browser
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// (```)
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// (```)
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
#### Commonly used sections
- Examples
- Panics: scenarios in which the function being documented could panic.
- Errors: describe the kinds of errors that might occur and what conditions might cause those
- Safety: if the functions is unsafe 
#### documentation comments as tests
- running `cargo test` with documentation comments will also run the code in the example section of the documentation comments
#### Commenting contained items
- use `//!` to document the item that is containing the comment instead of the item that comes after the comment
  - this is often used to document the crate itself and modules
### Exporting a convinient public API with pub use
- `pub use` allows you to re-export the imported modules, which takes a public modules in one location and makes it public in another location
  - exposing modules at the top level improves the experience for the users who uses the crate in its documentation and implementation
### Addning metadata to a new crate
- the package name must be unique in crates.io
- the package must have a license listed in linux foundation's SPDX
  - many people in the community use dual license of `MIT OR Apache-2.0`
- the package must have a description
```toml
[package]
name = "unique_name"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```
- with all these added, the crate can be published
- once the package is published, it is permanent and no version can be deleted
- to update, change the version value in `Cargo.toml`
- version of your crate can be deprecated using `yank`
  - yanking prevents any future projects from using that version and has no effect on the existing projects
  - yank can be undoed

## Cargo workspaces
- cargo workspaces helps you manage multiple related packages that are developed in tandem
- create workspaces by defining it in `Cargo.toml`
```toml
[workspace]
member = [
  "adder",
  "add_one",
]
```
```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
### inter crates dependencies
- if the `adder` crate depends on `add_one`, list the `add_one` crate and its path in dependencies of `Cargo.toml` in `adder`
```toml
[dependencies]
add_one = { path = "../add_one" }
```
### external crates dependencies
- a single `Cargo.lock` can be found in the workspace directory, which ensures that all the crates using the same version of the external crate
- to use external dependencies in a crate, it needs to be listed in `Cargo.toml` of workspace and the crate
### testing
- `cargo test` at the workspace directory will run the tests in all of the crates
- use -p to specify the crate to run tests

## Installing binaries with cargo install
- binay crates can be installed using `cargo install`, and the binary will be installed at ` ~/.cargo/bin` by default

## Extending cargo with custom commands
- Cargo can be extended with new subcommands without having to modify cargo. if a binary in your `$PATH` is named `cargo-something`, you can run it as if it was a cargo subcommand by running `cargo something`
