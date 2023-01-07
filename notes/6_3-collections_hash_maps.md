# Common collections
- these data collections are store in the heap, meaning that you don't need to know the size at compile time and it can shrink and grow in runtime
- all the collection types in standard library can be found at [collections doc](https://doc.rust-lang.org/std/collections/index.html)

## Hash maps
- hash map stores a mapping of keys of the type K to values of type V using a hashing function
  - all the keys must have same data type, and all of the values must have same data type

### Basic operaitons
Creating a hash map
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

// Accesssing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name) // returns Option<&i32>
      .copied()                        // converts Option<&i32> to Option<i32>
      .unwrap_or(0);                   // set score to zero if None

// Iterating over a hash map
    for (key, value) in &scores { // use reference so the loop does not take ownership
        println!("{key}: {value}");
    }
```

Ownership
- for the values with types that are stored in the stack, the value is copied into hash map
- for the values with types that are stored in the heap, the hash map takes ownership
- if a reference to a value was inserted to the hashmap, the value must be valid at least as long as the hash map is valid. (lifetimes)
```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```

Updating hash maps
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
// overwriting the value
    scores.insert(String::from("Blue"), 25);
// add only when it doesn't exist
    scores.entry(String::from("Yellow")) // return Entry enum that represents a value that might or might not exist like Option
      .or_insert(50); // returns reference to existing value or reference for the new value if the value does not exist
    scores.entry(String::from("Blue")).or_insert(50);

// updating value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { // split_whitespace returns iterator over sub-slices
        let count = map.entry(word).or_insert(0); // returns mutable reference 
        *count += 1; // dereference the mutable reference and update
    } // mutable reference goes out of scope each iteration

    println!("{:?}", map);
    println!("{:?}", scores);

```

### Hash functions
- uses SipHash by default, which is not the fastest hasing function but provides resistence to DoS attacks involving hash tables
  - [SipHash](https://en.wikipedia.org/wiki/SipHash)
