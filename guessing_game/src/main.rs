use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(101);

    print!("The Secret number is {}", secret_number);

    println!("Please input your guess!");


    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to Read line");

    print!("You guessed the input: {}" , guess);

}
