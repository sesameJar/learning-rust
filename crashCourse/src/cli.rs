use std::env;

pub fn run(){
    let args:Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Mehrad";
    let status = "100%";

    // println!("Command: {}",command );
    if command == "hello" { // cargo run hello
        println!("Hi {}, how are you?", name);
    } else if command == "status" { // cargo run status
        println!("Status is {}", status);
    } else { // cargo run hi ---- NOT cargo run
        println!("That is not a valid command");
    }
}