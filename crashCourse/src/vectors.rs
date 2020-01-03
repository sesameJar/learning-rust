// Vectors - resizable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    //Re-assign vlaue
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    //Get single val
    println!("Single Value : {}", numbers[0]);

    //Get Vector length
    println!("Vector length : {}",numbers.len() );

    // Vector are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers) );

    //Get slice
    let slice:&[i32] = &numbers[1..3];
    println!("slice : {:?}",slice);

    // Loop thriugh vector vals
    for x in  numbers.iter() {
        println!("Number : {}", x);
    }


    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
        
    }
    println!("Numbers Vec : {:?}", numbers);
}