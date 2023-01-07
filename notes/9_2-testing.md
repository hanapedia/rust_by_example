# tests 
## controlling how tests are run
### parallel or consecutive
- in case the tests have shared state, running in parallel results the tests to fail for unintended reason
- in such case, tests can run consecutively with setting `--test-threads` to 1
### showing stdout
- output to stdout by the code tested will not show by default
  - to show, use `--show-output` flag
### running subset of tests by name
- you can spesify which tests to run by passing the name of the test function
- you can also pass part of a test name to run tests with matching name
### ignoring tests
```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore] // this test will be ignored
fn expensive_test() {
    // code that takes an hour to run
}
```
- run with `--ignored` to only run the tests with ignore annotation

## test organization
### Unit tests
- the purpose of the unit test is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected 
- the convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`
  - `#[cfg(test)]` annotation unsures that the test module is compiled only with `cargo test`
#### Private functions
- rust allows testing of private function as child modules can read private functions of their ancestors
```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```
### Integration tests 
- the purpose of the integration test is to test whether many parts of your library work together correctly.
#### test directory
```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```
- cargo knows to look for integration tests file in the `tests` directory
```rust
use adder; // need to bring in the modules

#[test] // no need to add #[cfg(test)], as this is in tests directory
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```
- running test with integration test, will output three sections: unit test, integration test, and documentation test
- use `--test` and file name of the integration test to run only the tests in that file
#### using submodules in test directory
the old module convention, `tests/common/mod.rs` must be used over `tests/common.rs` to tell the compiler that `common` module does not include a integration test
```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```
#### integration tests for binary crates
- if the project only contains `src/main.rs` and no `src/lib.rs`, you cannot use the `test` directory and bring functions defined in the `src/main.rs` file into scope with `use` statement
- this is why Rust projects that provide a binary have a straightforward `src/main.rs` file that calls logic that lives in the `src/lib.rs` file.
