use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("input ur guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}.", secret_number);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Faield to read line");

    let guess : u32 = guess.trim().parse().expect("Please type a number.");

    print!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
