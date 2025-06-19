use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(0..=100);
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("secret number: {secret_number}");

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid input!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
