pub fn fibonacci(n: u64) -> u64 {
    println!("using recursive");
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
