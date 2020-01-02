// Conditionals - Used to check the condition of something and act on the result
pub fn run(){
    let age:u8 = 22;
    let check_id:bool = true;
    let know_person_of_age = true;

    if age >= 21 && check_id || know_person_of_age{
        println!("Bartemder : What would you like to drink?");
    } else if age< 21 && check_id{
        println!("Bartemder : Sorry you have to leave.");
    } else {
        println!("Bartemder : I need to see your ID.");
    }

    // Shorthand If

    let is_of_age = if age >= 21{true} else {false};
    println!("Is of age : {}", is_of_age);
}