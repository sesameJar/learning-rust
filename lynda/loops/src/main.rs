fn main() {

    loop {
        println!("Again!");
        break;
    }

    let mut number = 3;
    while number != 0{
        println!("{}!",number);
        number-=1;
    }

    let a = [1,2,3,4,5];
    for elm in a.iter() {
        println!("elm : {}", elm);
    }
}
