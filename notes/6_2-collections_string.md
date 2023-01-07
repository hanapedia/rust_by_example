# Common collections
- these data collections are store in the heap, meaning that you don't need to know the size at compile time and it can shrink and grow in runtime
- all the collection types in standard library can be found at [collections doc](https://doc.rust-lang.org/std/collections/index.html)

## Strings
- rust only has one type of *string* which is string slices `str`, usually seen as `&str`
  - string literals are also string literals just that `&str` points to the string stored in program's binary
- when Rustaceans say "String" they can be referring to `String` or `&str`
- all strings are UTF-8 encoded
- many of the same operations as Vectors work with strings
  - strings are implemented as a wrapper to vector of bytes
### creating strings
```rust
    let mut s = String::new();

    // from string literals
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
```
### updating strings
Apppending 
```rust
// appending
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // does not take ownership
    println!("s2 is {s2}"); // so this has no problem
// appending single char
    let mut s = String::from("lo");
    s.push('l');
```
Concatenating
``` rust
// concatenating
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // + operator uses add method under the hood 
    fn add(self, s: &str) -> String {}
    // takes the ownership of s1 with self, but does not take the ownership for s2 as s is a reference to a string slice
    // rust compiler coerce the &String to &str, thus passing &s2 works even though it is &String

    // format! macros can be used for complex concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // format! macros does not take the ownership of any value
```
Indexing (DOES NOT WORK)
- cannot use integers to index stings unlike other languages
  - rust string is a wrapper over `Vec<u8>`, and u8 character does not always correspond to a single byte. characters in other languages might have more than one byte for a character.
``` rust
// Indexing into strings
    let s1 = String::from("hello");
    let h = s1[0]; // throws error and does not compile

    let hello = String::from("Здравствуйте"); // this has length of 24 instead of 12
```
Slicing
```rust
let hello = "Здравствуйте";

let s = &hello[0..4]; // returns first four bytes
// however &hello[0..1] panics at runtime

```
Iteration
- best way to operate on piece of string is to be explicit about whether you want characters or bytes.
```rust
// as chars
for c in "Зд".chars() {
    println!("{c}");
}
// or as bytes
for b in "Зд".bytes() {
    println!("{b}");
}
```

