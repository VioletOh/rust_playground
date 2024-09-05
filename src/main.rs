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
}
