fn main() {
    let a = 22;
    let b = add_one(a);
    println!("Hello, world!");
}

 fn add_one(i: i32) -> &'static i32 {
    let result = i + 1;
    &result
}
