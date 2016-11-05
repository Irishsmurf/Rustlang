extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Input your guess:");
    
    let secret_number = rand::thread_rng()
                                .gen_range(1, 101);
    
    println!("Secret number is: {}", secret_number);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: u32 = guess.trim().parse()
                            .expect("Invalid Number!");

        println!("You guessed: {}", guess);
        


        match guess.cmp(&secret_number) {
            Ordering::Less          => println!("Too Small"),
            Ordering::Greater       => println!("Too big!"),
            Ordering::Equal          => {
                println!("You win!");
                break;
            }
        }
    }
}
