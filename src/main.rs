use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Welcome to the guessing game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please enter a number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to take a number");

        println!("You entered number {guess}");
        let guess : i32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => {
            println!("You entered incorrect input");
            continue;}  
        } ;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        };
    }
    
}