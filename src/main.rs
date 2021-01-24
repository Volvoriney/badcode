use std::io;

fn main(){
    println!("This is guessing game");
    println!("Just input some number please!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guessed number is: {}", guess);
}