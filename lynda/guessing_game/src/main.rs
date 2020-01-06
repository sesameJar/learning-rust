extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess :");
    println!("Secret is {}", secret);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line!");

    println!("You guessed {}", guess);
    
}
