use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    println!("Input your guess");
    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=5);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line...");
    
    let guess: u16 = guess.trim().parse().expect("Expect an positive integer");

    if secret_number == guess {
        println!("Congratulations your guessed number was right: {guess}");
    }
    else {
        println!("Your guess was wrong: {guess} != {secret_number}");
    }
    println!("You guessed: {guess}");
}
