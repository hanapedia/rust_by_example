# Writing cli program
## Notes
- `collect` method on iterator turns iterator into a vector containing all values in iterator
- create functions for single functionality
- `lines` method on string slices returns iterating object for each line
- `eprintln!` macro prints to stderr instead of stdout
### Separation of concern for binary projects
- split your program into a `main.rs` and `lib.rs` and move your program's logic to `lib.rs`
- as long as your command line parsong logic is small, it can remain in main.rs
- when the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`

The responsibilities remaining in the `main` function after this process should be limited to the following
- calling the command line parsing logic with the argument values
- setting up any other config
- calling a `run` function in `lib.rs`
- handling the error if `run` returns an erro

An indicator that you don't have enough abstraction is when you are breaking up a tuple right away, try grouping them together into a struct

Avoid using `clone` method on bigger data because of its runtime cost.

#### Closures with unwrap_or_else method
- calling `unwrap_or_else` method on `Result` returns the value if `Ok` else runs the closure given with `Err` value
#### Returning errors with trait objects
- trait object allows you to return a type that implements a trait without specifying the particular type
- `dyn` is short for dynamic

#### Test Driven Development
1. Write a test that fails and run it to make sure it fails for the reason you expect
2. Write or modify just enough code to make the new test pass
3. Reafactor the code you just added or changed and make sure the tests continue to pass
4. Repeat from step 1
- writing the test before the code helps to maintain high test coverage thoughout the process

#### Working with environmental variables
- 
### reminder
- `if let` statement allows you to handle only one particular pattern while ignoring the rest
- `is_ok` method on `Results` returns true if `Ok` has value and false otherwise

## Improving the code with iterators
- update the `build` of the `Config` struct to use iterators instead of refenrence to the array
  - and the function takes the ownership of the iterator as well
- this allows you not to clone the values from the refenrence
- update the `search` function to use iterator adaptors to rmove the mutable state
