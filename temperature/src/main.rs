use std::io;

fn main() {
    println!("Input temperature in Fahrenheit");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let celcius = (fahrenheit.trim().parse::<f32>().unwrap() - 32.0) / 1.8;

    println!("{}", celcius);
}
