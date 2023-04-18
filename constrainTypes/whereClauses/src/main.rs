// Where clauses provide an alternative syntax for specifying trait bounds and
// type bounds. For example, this code specifies that the generic type T must
// implement both the Display and Debug traits, and that it must be a reference:

use std::fmt::{Debug, Display};

fn main() {
    println!("Hello, world!");
}

// In this example, the print function can only be called with types that 
// implement Display and Debug, and that are references. The AsRef<str> trait 
// allows the function to treat the value as a string reference, regardless of its 
// underlying type.

fn print<T>(val: T)
where
    T: Display + Debug + AsRef<str>,
{
    println!("{:?}", val.as_ref());
}
