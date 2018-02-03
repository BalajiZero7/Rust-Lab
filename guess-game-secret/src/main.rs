use std::io;

fn main() {

    println!("Let's start guess game" );
    println!("Enter your guess number");

   let mut guess = String::new();
    io::stdin().read_line(& mut guess).expect("Failed to read line");
    println!("Your guessed number is {}", guess);

}
