# Testing
- a test module is automatically generated when initicalizing a library project using `Cargo new --lib`
- there can be multiple test modules and test functions
- run `cargo test` to run tests
```rust
// test module is defined using test attribute 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail"); // this fails the test
    }
}
```
- each test runs in a new thread, and when the main thread sees that one of the tests had died, the test is marked as failed
- test fails when it panics

## assert! macro
- checks if the argument is true, panics if not
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*; // import everything from parent module

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

## assert_eq and assert_ne macros
- checks if two arguments are equal/not equal, and pass if they are and faile if they are not

## adding custom messages to test results that fail
- any additional arguments to `assert!` `assert_eq!` `assert_ne!` are passed down to `format!` macro

## test error handling behavior with should_panic
- tests with `shoud_panic!` macro passes when the code running panics
- to ensure that the test passes only when the code is panicing at the intended place, use `expected` parameter in `should_panic!`
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200); // Guess panics with 200 
    }
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## Using Result<T, E> in tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) // when passes
        } else {
            Err(String::from("two plus two does not equal four")) // when fails
        }
    }
}
```
- this enables you to write the test that uses `?` operator
- to assert that an operation returns and `Err` variant, use `assert!(value.is_err())`
