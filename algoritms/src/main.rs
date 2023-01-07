fn main() {
    let mut a: i32 = 24;
    let b: i32 = 1;
    // let mut min: i32 = 0;
    while a & b == 0 {
        a = a >> 1;
    }
    println!("{}", a );
}
