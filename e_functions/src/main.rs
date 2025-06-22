fn main() {
    println!("hello from main");

    another_function();

    takes_one_param(42);

    print_labeled_measurement(98, 'C');

    let y = {
        let x = 3;
        x + 1
    };

    println!("y = {y}");

    let num = five();
    println!("five() -> {num}");

    let val = plus_one(99);
    println!("plus_one(99) = {val}");

    // let oops = broken_plus_one(10);
}

fn another_function() {
    println!("inside another_function");
}

fn takes_one_param(x: i32) {
    println!("got: {x}");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("measurement: {value}{unit}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn broken_plus_one(x: i32) -> i32 {
//     x + 1; // ; breaks the return
// }
