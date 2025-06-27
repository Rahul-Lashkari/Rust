fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The len of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s is now: {}", s);

    // let r1 = &mut s;
    // let r2 = &mut s; // error
    // println!("{}, {}", r1, r2);

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 & r2 scopes end here

    let r3 = &mut s;
    println!("{}", r3);

    // let reference_to_nothing = dangle(); // error
    let owned_string = no_dangle();
    println!("Got string: {}", owned_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
