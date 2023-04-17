fn main() {
    let spaces = "    ";
    // let spaces = spaces.len();

    println!("Hello, world!");
    println!("spaces {}", spaces);
    print_spec(spaces);
}

fn print_spec(x:&str) {
    println!("spaces {}", x);
}
