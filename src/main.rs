use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("You will have 5 chances to guess the number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut count = 0;
    loop {
        if count == 5 {
            println!("\n\nYou lose! The secret number is {secret_number}!\n\n");
            break;
        }
        
        if count > 0{
            println!("You have {} chances left.", 5 - count);
            println!("Please input a number as your guess:");
        } else {
            println!("Please input your guess:");
        }
        
        count += 1;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if (secret_number - guess) < 5 {
                    println!("Too small! But you are close!\n");
                } else {
                    println!("Too small!\n");
                }
            },
            Ordering::Greater => {
                if (guess - secret_number) < 5 {
                    println!("Too big! But you are close!\n");
                } else {
                    println!("Too big!\n");
                }
            },
            Ordering::Equal => {
                println!("\n\nYou win!\n\n");
                break;
            }
        }
    }
}