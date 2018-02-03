use std::io;

fn main() {

    println!("Let's play guess game now");
    println!("Please enter you guess!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read_line!");

    println!("Your guess is {}", guess);
}