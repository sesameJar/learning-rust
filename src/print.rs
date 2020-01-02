pub fn run() {
    //public function

    println!("Hello from print.rs file.",); // print to console
    println!("{} is from {}", "Mehrad", "Mashhad"); // basic formatting
                                                    //positional arguments
    println!(
        "{0} is from {1} and {0} is good in {2}",
        "Mehrad", "mashhad", "coding"
    );

    // Named Arguments
    println!(
        "{name} likes {activity}",
        name = "mehrad",
        activity = "coding"
    );

    // Placeholders Traits

    println!("Binary : {:b} Hex:{:x} Octal :{:o}", 10, 10, 10);

    // Placeholder for debug trais
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 : {}", 10 + 10);
}
