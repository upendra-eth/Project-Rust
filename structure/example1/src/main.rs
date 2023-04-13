struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!(" struct is active ? -  {}", user1.active);
    println!(" print user email  -  {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!(" print user email  -  {}", user1.email);
}
