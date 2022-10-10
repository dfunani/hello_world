use rand::Rng;
use std::io;

fn main() {
    let randNum = rand::thread_rng().gen_range(1..=100);
    println!("I am thinking of a number!!!");
    println!("Can you guess it?");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to make an apt guess.");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!(),
    };

    println!("Hello, world! {guess} vs {randNum}");

    if guess < randNum {
        println!("Too Low!!!");
    } else if guess > randNum {
        println!("Too High!!!");
    } else {
        println!("Great Guess!!!");
    }
}
