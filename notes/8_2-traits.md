# Traits: Defining shared behavior
- trait defines functionality a particular type has and can share with other types
- similar to interfaces in other languages

## definition and implementation
```rust
pub trait Summary { // define useing trait keyword
    fn summarize(&self) -> String; // semicolon instead of block, just the signature
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle { // use impl and for keyword
    fn summarize(&self) -> String { // with same signature, and add code block
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
- other crates that depends on the trait can bring in the trait into the scope and implement it on their own types
- we can implement a traint on a type only if at least one of the trait or the type is local to our crate

## Default implementation
```rust
pub trait Summary {
    fn summarize(&self) -> String { // a default implementation
        String::from("(Read more...)")
    }
}
impl Summary for NewsArticle {} // to use the default implementation, define empty method

pub trait Summary {
    fn summarize_author(&self) -> String;

    // the default implementation can call the other methods in the trait 
    // even if it does not have a default implementation
    // this allows the implementer to work on less code when most of the implementation 
    // is done in the default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

## Traits as parameters 
```rust
// this function can take any type that implment Summary trait
pub fn notify(item: &impl Summary) { 
    println!("Breaking news! {}", item.summarize());
}
// above is a syntax sugar for trait bound 
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// trait bound is more expressive. egs.
// this does not say item1 and item 2 are of same type
// it only says that they have same trait
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// using trait bound however expresses that item1 and item2 are of same type
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// specify multiple trait bounds with + syntax
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

// clearer trait bounds with where clause
// instead of
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// write
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```
## Returning traits from a function
- this is useful when working with closures and iterators 
- note that this cannot return more than one type.
  - also returning NewsArticle does not work in the case below
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

## Using trait bounds to conditionally implement methods
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
  // new method is always defined for Pair struct
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
  // cmp_display is defined for Pair only if the type implements Display and PartialOrd
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// you can also define a trait for any type that implement another trait
// this is called blanket implementation, and is extensively used in rust
impl<T: Display> ToString for T {
    // --snip--
}
```
