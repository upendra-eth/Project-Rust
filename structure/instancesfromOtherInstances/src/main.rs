struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user( username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("someusername123") ,String::from("someone@example.com"));
    
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!(" print user1 email  -  {}", user1.email);
    println!(" print user2 email  -  {}", user2.email);



}
