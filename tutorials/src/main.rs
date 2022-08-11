fn main() {
    // println!("Hello, world!");
    // println!("I'm a Rustacean");
    // let x = 5 + 5;
    // println!("Insert value by {{}}"); // Curly braces are ecaped by {{}}
    // println!("x = {}", x); // Curly braces are ecaped by {{}}

    // {} parses any arguments
    println!("{} days", 31); 

    // number can be used to specify which argument goes where
    println!("{0} is first argument, {1} is the second argument", "Alice", "Bob");

    // names can also be used
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can invoked by specified format character after a
    // `:`.
    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);

    // For Rust 1.58 and above, you can directly capture the argument from
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");

    // Activity
    let pi = 3.141592; 
    println!("Pi is roughly {pi:.3}");
        
    return ;
}

// semicolon at the end of line is not optional
/* Block comment */
