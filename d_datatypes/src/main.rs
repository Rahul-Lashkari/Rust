use std::io;

fn main() {
    // == Scalar Types ==

    // Integer types
    let int8: i8 = -128;
    let uint8: u8 = 255;
    let int16: i16 = -32_000;
    let uint16: u16 = 65_535;
    let int32 = 1_000_000; // defaults to i32
    let int64: i64 = -9_223_372_036_854_775_808;
    let int128: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let usize_val: usize = 100; 

    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    println!("Integer types: {}, {}, {}, {}, {}, {}, {}, {}", int8, uint8, int16, uint16, int32, int64, int128, usize_val);
    println!("Literals: {}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);

    // floating-point types
    let x = 2.0;       // f64 by default
    let y: f32 = 3.0;  // f32 explicitly

    println!("Floats: f64 = {}, f32 = {}", x, y);

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("Maths => sum: {}, diff: {}, product: {}, quotient: {}, truncated: {}, remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );

    // Booleans
    let t = true;
    let f: bool = false;

    println!("Booleans => t: {}, f: {}", t, f);

    // Characters
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Chars => {}, {}, {}", c, z, heart_eyed_cat);

    // == Compound Types ==

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("Destructured tuple => x: {}, y: {}, z: {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("Access via index => {}, {}, {}", five_hundred, six_point_four, one);

    let unit = (); // The unit type
    println!("Unit value: {:?}", unit);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", 
                  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    let array_with_type: [i32; 5] = [1, 2, 3, 4, 5];
    let same_value_array = [3; 5]; // [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

    println!("Array a: {:?}, first: {}, second: {}", a, first, second);
    println!("Months: {:?}", months);
    println!("Same value array: {:?}", same_value_array);

    // Try to read from array using user input (runtime panic possible!)
    println!("Enter an index (0 to 4):");

    let mut index_input = String::new();
    io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read input");

    let index: usize = index_input.trim().parse().expect("Please enter a number");

    let element = a[index];
    println!("Element at index {} is: {}", index, element);
}
