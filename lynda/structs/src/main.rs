fn main() {
    struct User {
        username: String,
        email: String,
        sing_in_count: u64,
        active: bool,
    };

    let user1 = User {
        email: String::from("example@asda.com"),
        username: String::from("Yo"),
        active: true,
        sing_in_count: 1,
    };

    println!("{}", user1.email);
    let user2 = User {
        email: String::from("asdasd"),
        username: String::from("Ben"),
        ..user1
    };

    println!("{:?}", user2.sing_in_count);
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sing_in_count: 1,
//     }
// }
