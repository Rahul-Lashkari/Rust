fn main() {
    {
        let s = String::from("hello");
        println!("s in inner scope: {s}");
    }
    // s is now out of scope

    let s1 = String::from("i will be moved");
    let s2 = s1;

    // println!("s1 is: {s1}"); // Error: borrow of moved value
    println!("s2 is: {s2}");

    let s3 = String::from("hello");
    let s4 = s3.clone(); // use clone for a deep copy

    println!("s3 = {s3}, s4 = {s4}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    let my_string = String::from("take me");
    takes_ownership(my_string);
    // println!("{my_string}"); // Error: value moved

    let my_int = 100;
    makes_copy(my_int);
    println!("my_int is still valid: {my_int}");

    let str1 = gives_ownership();
    let str2 = String::from("world");
    let str3 = takes_and_gives_back(str2);

    println!("str1 = {str1}, str3 = {str3}");

    let (s5, len) = calculate_length(str3);
    println!("The length of '{}' is {}.", s5, len);
}

fn takes_ownership(some_string: String) {
    println!("Inside takes_ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Inside makes_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let new_string = String::from("yours");
    new_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}