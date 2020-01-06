fn main() {
    let name = String::from("Meh");

    println!("{}", 
        match name.chars().nth(4) {
            Some(expr) => expr.to_string(),
            None => "No character found".to_string(),
        }
    )
}
