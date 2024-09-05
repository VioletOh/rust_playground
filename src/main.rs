/* 
    Cargo 
    - Manages dependencies for repeatable builds
    - Downloads and builds external libraries
    - Calls rustc with correct parameters

    cargo --version
    cargo new ___
    cargo run
    cargo build --release

    Configuration file: Cargo.toml
    
    TOML
    - Tom's Obvious Minimal Language

    By default, variables are immutable
    Declare mutable using mut keyword

    Floating-point:
    f32 and f64
    value stored as fractional and exponential components

*/

fn main() {
    // let x = -10;
    // println!("x is {}", x);
    // x = 20; cannot assign twice to immutable variable

    // signed number by default
    // let y: u8 = -10; // not possible
    let y: u8 = 255; // initializing it to 1000 wouldn't work since maximum possible val is 255

    let mut x: u8 = 255;

    let z = 10.0;
    println!("z is {}", x); // is 10

    let k: f32 = 10.123423423523238484; // reaches limit of 64 unless you put :f32
    println!("k is {}", k);

    let a = 10.0;
    let b = 3.0;
    let c = a / b; // + - * / (just 3) % 
    // let c = a as f64 / b;  => to get decimal points
    // let c = a as f64 / (b + 1.0); => ordering in arithmetic operations
    println!("c is {:.3}", c); // print c with exactly three decimal places of precision
    println!("c is {:8.3}", c); // Pad 8 chars in front
    println!("c is {:08.3}\na is {}", c, a); // Pad 8 leading zeros in front =? 0003.333
    // You can put numbers within the curly braces to indicate which variable you are wishing to print & you can also put variable names staright in the braces
    // {0:08.3}\na is {1}

}
