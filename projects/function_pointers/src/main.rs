use std::io;

fn add(x : i32, y : i32) -> i32 {
    x + y
}

fn sub(x : i32, y : i32) -> i32 {
    x - y
}

fn mul(x : i32, y : i32) -> i32 {
    x * y
}

fn main() {
    println!("The simple calculator");
    println!("1. Add");
    println!("2. Subtract");

    let mut action = String::new();
    //Grab input
    io::stdin().read_line(&mut action)
        .expect("Failed to read line");
    //Chomp and parse to int
    let action : i32 = action.trim().parse()
        .expect("Please enter a number");
    
    // Default action is multiply 
    let mut f : fn(i32, i32) -> i32 = mul;
    
    if action == 1 {  
        f = add;
    }
    else if action == 2 {
        f = sub;
    }
    
    println!("The answer is {}", f(12,5));
}
