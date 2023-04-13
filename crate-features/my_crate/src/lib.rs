#[cfg(not(feature = "xyz"))]
pub mod iterative_fibonacci;
#[cfg(feature = "xyz")]
pub mod recursive_fibonacci;

pub fn fibonacci(n: u64) -> u64 {
    // default implementation (recursive)
    #[cfg(feature = "xyz")]
    let x = recursive_fibonacci::fibonacci(n);

    #[cfg(not(feature = "xyz"))]
    let x = iterative_fibonacci::fibonacci(n);

    x
}

#[test]
fn test1() {
    println!("printing fab {}", fibonacci(5))
}
