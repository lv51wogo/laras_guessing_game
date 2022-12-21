use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess my number!");

    let top_secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your number!");

         let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match 
            guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&top_secret_number){
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too hight!"),
            Ordering::Equal => {
                println!("Yey you win!");
                break;
            }
        }
    }
}
