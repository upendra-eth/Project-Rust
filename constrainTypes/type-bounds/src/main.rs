fn main() {
    println!("Hello, world!");
}

fn add<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + Copy>(a: T, b: T) -> T {
    a + b
}
