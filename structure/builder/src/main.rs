struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {

    let mut user1 = build_user(String::from("uvsingh2336@gmail.com"),String::from("upendra"));
    println!(" print user name  -  {}", user1.username);

    user1.email = String::from("anotheremail@example.com");
    println!(" print user email  -  {}", user1.email);
    


}
