use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn generate_random_number(min: u8, max: u8) -> u8 {
    return rand::thread_rng()
                 .gen_range(min..=max);
}

fn main() {
    let secret_number = generate_random_number(1, 100);
    let mut tries = 0;
    loop {
        println!("Guess the secret number (1-100): ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("An Error ocurred reading the input");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        tries = tries + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
        
    println!("You guessed after {} tries", tries);
}
