use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess: ");
    //Define a mutable string 
    let mut guess =  String::new();
    
    //Grab input and store in guess and handle error with appropriate message
    //Pass mutable reference to guess explicitly as rust references are immutable by default
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed : {} ", guess);
}
 
