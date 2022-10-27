use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("🎲 Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;

    loop {
        println!("️❔ Please input your guess:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(num) => num,
            Err(_) => {
                println!("❗ Failed to read line");
                return ExitCode::FAILURE;
            }
        };

        let input: &str = input.trim();

        if input == "q" {
            println!("👋 Bye! (Giving up after {tries} tries...)");
            return ExitCode::SUCCESS;
        }

        let guess: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❗ Enter a valid number or \"q\" to quit!");
                continue;
            }
        };

        tries += 1;

        match guess.cmp(&secret) {
            Ordering::Less => println!("🔺 The number {guess} was too low, try again!"),
            Ordering::Greater => println!("🔻 The number {guess} was too high, try again!"),
            Ordering::Equal => break,
        };
    }

    println!("✅ You won! (Took you only {tries} tries.)");
    return ExitCode::SUCCESS;
}
