use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Let's play Guess the number!");
    loop {
        println!("Please guess a number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too Low"),
            Ordering::Equal => {
                println!("You Win! You guessed it right");
                break;
            }
        }
    }
}
