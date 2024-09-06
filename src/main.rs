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
    bitwise();
    boolean();
    practice1();
    array();
    multi_dimensional_array();
    tuple();
    functions();
    let result = square(13);
    println!("result is {}", result);
    let multi = multi_return(13);
    println!("result for multi is {:?}", multi);
    celsius_to_farenheit();
}

fn celsius_to_fahrenheit(temp: i32) -> i32 {
    ((1.8 * temp as f64) + 32 as f64) as i32;
}
// If you don't include the return type:
// Unit Data Type
// You may use -> () or leave it as blank

fn multi_return(x: i32) -> (i32, i32) {
    println!("squaring multi {}", x);
    return (x, x * x);
    println!("End of function");
}
fn square(x: i32) -> i32 {
    println!("squaring {}", x);
    x * x // this is an expression -> result gets passed out as a return value
    // Cannot add any code here unless you add return keyword and add semicolon. It is still unreachable
}

fn say_a_number(number: i32) {
    println!("number is {}", number); // called from functions() function
}
fn functions() {
    println!("My new function!");
    say_a_number(13);

    let x = 1;
    let y = 2;
    say_the_sum(x, y);
}
fn say_the_sum(a: u8, b:u8) {
    let sum = a + b;
    println!("sum is {}", sum);  
}

fn tuple() {
    // Group multiple items of mixed data types
    // Elements are ordered
    // Stored in a fixed-length, contiguous section memory
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);
}

fn multi_dimensional_array() {
    let parking_lot = [[1,2,3], [4,5,6]];
    let number = parking_lot[1][2];
    println!("number is {}", number);

    let garage = [[[0; 100]; 20]; 5]; // Three dimensinal array
}

fn array() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32;5]; // to determine how much memory
    numbers = [0; 5]; // 0 0 0 0 0
    let index: usize = numbers.len();
    println!("Index is {}", index);
    println!("last number is {}", numbers[index - 1]); // if you try numbers[5] it fails due to index out of bounds

    // usize Data Type
    // size is based on number of bytes needed to reference memory
}
fn practice1() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;
    assert_eq!(average, 45.1);
    println!("Test passed!");
}

fn boolean() {
    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    let c = (a ^ b) | (a & b);
    println!("c is {}", c);

    //let d = (a ^ b) | panic!(); // panic will terminate program immediately, however left side is already True therefore it retruns true
}

fn bitwise() {
    let mut value = 0b11110101u8; // Prefix 0b, you may add underscores for better readability
    // Added suffix u8 to indicate it's an unsigned 8 bit integer
    println!("value is {}", value);
    println!("value is {:08b}", value); // padding leading 0s value is 11110101

    // Bitwise Operators: NOT (!)
    value = !value;
    println!("value is {:08b}", value); // value is 00001010
    // AND (&): Check value of a specific bit
    value = value & 0b1111_0111;
    println!("value is {:08b}", value); // value is 00000010

    // OR (|): set value of a specific bit
    value = value | 0b0100_0000;
    println!("value is {:08b}", value); // 01000010

    // XOR (^): if it differs
    value = value ^0b0101_0101;
    println!("value is {:08b}", value); // 00010111

    // Bit shift
    value = value << 4;
    println!("value is {:08b}", value);

    value = value >> 2;
    println!("value is {:08b}", value);
}