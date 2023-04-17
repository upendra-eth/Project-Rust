trait Add<T> {
    fn add(&self, other: T) -> T;
}

struct Calculator;

impl Add<i32> for Calculator {
    fn add(&self, other: i32) -> i32 {
        42 + other
    }
}

impl Add<T> for Calculator {
    fn add(&self, other:T) -> 
}

fn main() {
    let calculator = Calculator;
    let result = calculator.add(8);
    println!("Result: {}", result); // prints "Result: 50"
}
