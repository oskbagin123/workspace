use std::io;

fn main() {
    println!("Guess the number!");
    println!("input ur guess:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Faield to read line");

    println!("You guessed: {}!", guess)

}
