fn main() {
    let x = 6;

    if x < 5 {
        println!("x is small");
    } else {
        println!("x is not small");
    }
    
    let n = 15;
    if n % 4 == 0 {
        println!("n is div by 4");
    } else if n % 3 == 0 {
        println!("n is div by 3");
    } else {
        println!("n is not div by 4 or 3");
    }

    let is_active = true;
    let status = if is_active { "online" } else { "offline" };
    println!("user status: {status}");

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("result: {result}");

    // labeled loops
    let mut count = 0;
    'outer: loop {
        println!("outer count: {count}");
        let mut inner_count = 10;
        loop {
            if inner_count == 9 {
                break;
            }
            if count == 2 {
                println!("breaking outer loop");
                break 'outer;
            }
            inner_count -= 1;
        }
        count += 1;
    }
    println!("final count: {count}");

    let mut num = 3;
    while num != 0 {
        println!("{num}!");
        num -= 1;
    }
    println!("LIFTOFF!");
    
    // for loop
    let a = [10, 20, 30, 40, 50];
    for item in a {
        println!("the value is: {item}");
    }

    // for loop with range
    for number in (1..4).rev() { 
        println!("{number}!");
    }
    println!("LIFTOFF AGAIN!");

    println!("\n10th fibonacci no: {}", fib(10));
}

fn fib(n: u32) -> u32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}