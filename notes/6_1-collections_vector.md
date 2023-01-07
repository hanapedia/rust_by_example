# Common collections
- these data collections are store in the heap, meaning that you don't need to know the size at compile time and it can shrink and grow in runtime
- all the collection types in standard library can be found at [collections doc](https://doc.rust-lang.org/std/collections/index.html)

## Vectors
### create new vectors by
```rust
    let v: Vec<i32> = Vec::new();
    // or with macro, compiler infers type. in this case i32
    let v = vec![1, 2, 3];
```
### add elements using push
```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```
### referencing the values in a vector
- `get` and `[]` behaves different when it encouters out-range-index
    - `[]` panics, but `get` returns `None`
- vectors are reallocated in memory if the length after inserting new item exceeds its capacity
```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```
- borrow checker enforces ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remains valid.
  - this prevents the reference to point to deallocated memory when the value of the vector has to be copied to other place in memory to account for new elements added
```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable reference

    v.push(6); // mutable reference

    println!("The first element is: {first}"); // immutable reference used here
```
### Iterating over the values
- iterate over elements instead of indices
```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50; // dereference operator must be used to mutate the value
    }
```
### Using iterators to store multiple types
```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![ // this is gonna be a vector of SpreadsheetCell enum
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```
### other methods
refer to [vector api doc](https://doc.rust-lang.org/std/vec/struct.Vec.html)
