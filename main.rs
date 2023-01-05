use std::io;

fn main(){
  println!("Let's Get Rustyyyy!!!1");


  println!("Please input your guess here....");


  let mut guess = String::new();
  
  io::stdin().read_line(&mut guess)
  .expect("Failed to read line");
  
  println!("Your guessed: {}",guess);
  
}