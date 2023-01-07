# Macros
- macros referes to a family of features in Rust: *declartive* macros with `macro_rules!` and three kinds of *procedural* macros
  - Custom `#[derive]` macros that specify code added with the derive attribute used on structs and enums
  - Attribute-like macros that define custom attributes usable on any item
  - Funciton-like macros that look like function calls but operate on the tokens specified as their arguments
## Difference between macros and functions
- macros are way of writing code that writes other code
  - `derive` attribute generates implementations for you
  - `println!` ,etc, generate more code than you've written manually
- macros can take variable number of parameters
- macros are expanded before the copiler interprets the meaning of the code
- macro definitions are more comlex as you are writing rust code that writes rust code

## Declartive macros with macro_rules! for general metaprogramming
- compare a value to patterns that are associated with particular code: the values is the litral rust source code passed to the macro; the patterns are compared with the structure of the source code; andthe code associated with each pattern, when matched, replaces the code passes to the macro
### example with vec!
- the body of the macro definition is similar to `match` expression
- except that the macros patterns are matched against code structure rather than values
  - `()` to encompass the whole pattern
  - `$` to declare a variable in the macro system that contain the rust code matching the pattern
  - `()` to capture the values that match the pattern within in the parenthesis for use in the replacement code
  - `$x:expr` matches any rust expression and gives the expression the name `$x`
  - `,` indicates that a literal comma separator character could optionaly appear after the code that matches the code in `$()`
  - `*` specifies the pattern matches zero or more of whatever precedes the `*`
- when the macro is called with `vec![1,2,3]`, the `$x` pattern matches three times with the three expressions `1`, `2`, and `3`
```rust
let v: Vec<u32> = vec![1,2,3];

// vec macro definition (simplified)
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
// code generated
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

## Procedural macros for generating code from attributes
- accepts some code as an input, operate on that code, and produce some code as an output 
- the definitions of procedural macros must reside in their own crate with special crate type
```rust
use proc_macro;

// some_attribute is a placeholder for the macro variety
// takes TokenStream and returns TokenStream
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```
### How to write a custom derive macro
- create a separate crate for the macro, and configure in `Cargo.toml`

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```
- in `src/lib.rs` of macros crate
```rust
use proc_macro::TokenStream;
use quote::quote; // turns syn data structure back into rust code
use syn; // parses rust code from string intoa a data structure

#[proc_macro_derive(HelloMacro)]
// this is called when users use #[derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
// holds the identifyier for the struct or enum to implement the trait for
    let name = &ast.ident; 
    let gen = quote! {
        // #name is replaced by the value in name
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify turns the argument into string literal
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

### Attribute-like macros
- Attribute-like macros allow you to create new attributes
- those attributes can be applied to other items such as functions, in addition to structs and enums
```rust
// web-application framework might have something like this
#[route(GET, "/")]
fn index() {

// route definition
#[proc_macro_attribute]
// attr holds `GET` and `"/"` part
// item holds the body of index
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

### Function-like macros
- simlar to `macro_rules!` macro, but does not use match-like syntax
- they can take unknown number of arguments
- take a `TokenStream` parameter and their definition manipulates that `TokenStream` using rust code as the other two types of procedural macros do
```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```
