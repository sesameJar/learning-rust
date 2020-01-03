fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5);
    let a = sum(2, 3);
    println!("sum = {}", a);
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("Th value of x : {}", x);
}

fn sum(x: i32, y: i32) -> i32 {
    let mut b = x + y;
    b= b * 100;
    return b;
}
