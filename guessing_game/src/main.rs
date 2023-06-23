use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Please type in a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(k) => {
                println!("Error:{k}. \nUser input: {guess}");
                continue; //must return error handling, in this case since we're in a loop we can just continue.
                          //we could handle error by returning 0 and handle if 0 continue.
                          //0 without; or return 0;
            }
        };
        println!("you guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
