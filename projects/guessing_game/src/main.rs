extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    
    // Generate a random number between 1 and 100
    let secret_num = rand::thread_rng().gen_range(1,101);

    
    loop {
        println!("Please input your guess: ");
        //Define a mutable string 
        let mut guess =  String::new();
    
        //Grab input and store in guess and handle error with appropriate message
        //Pass mutable reference to guess explicitly as rust references are immutable by default
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        //convert guess to an integer
        //if input is not an integer, ignore and ask for input again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {} ", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less      => println!("Too small"),
            Ordering::Greater   => println!("Too big"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            },
        }
    }
}
 
