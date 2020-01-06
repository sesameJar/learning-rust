fn main() {
    let refer_nothing = dangle();
    println!("{}", refer_nothing);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
