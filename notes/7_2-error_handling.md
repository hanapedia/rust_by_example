# Error Handling
## To panic or not to panic
### Prototype code and tests
- `unwrap` and `expect` is useful for prototyping and testing
### Cases in which you have more information than the compiler
```rust
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
```
- when you know that the code is logically impossible to fail, it it practical to use `unwrap` and `expect`
### Guidelines for error handling
panic when it's possible that you code could end up in a bad state. bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradicroty values, or missing values are passed to your code ——plus one or more of the following:
- the bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format
- your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step
- There's not a good way to encode this information in the types you use. 
### Creating custom types for validation
In a guessing game where user is expected to enter an interger between 1 and 100, one can implement a check like this:
```rust
    loop {
        // --snip--
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        match guess.cmp(&secret_number) {
            // --snip--
    }
```
- although you can be confident that the value will valid, this is impractical if there are multiple functions with this requirement, having the same check like this would be tedious and might impact performance. 

- instead you can define a custom type with validation function included in the associated function for creating the instance of that type
```rust
pub struct Guess {
    // must be private so that the code outside will not be able to set its value directly
    // this ensures that the value set will always go through the validation
    value: i32, 
}
impl Guess {
    // initialization function with validation
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
    // getter function for accessing private 'value' field after initialization
    pub fn value(&self) -> i32 { 
        self.value
    }
}
```
- then any function that requires the validation can use this type as its parameter or return type
