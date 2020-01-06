extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Wlecome to the guessing game");
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess :");
    // println!("Secret is {}", secret);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line!");

        println!("You guessed {}", guess);
        let guess: u32 = guess.trim().parse().expect("Please Type a number.");
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Awesome!!!!!!");
                break;
            }
        };
    }
}
