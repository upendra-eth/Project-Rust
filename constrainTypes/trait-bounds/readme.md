# Trait Bounds

Trait bounds specify that a type must implement one or more traits. For example, this code specifies that the generic type T must implement the Display trait:

use std::fmt::Display;

fn print<T: Display>(val: T) {
    println!("{}", val);
}


In this example, the print function can only be called with types that implement Display, which provides a way of formatting values as strings.

You can also specify multiple trait bounds by separating them with a + sign. For example, this code specifies that the generic type T must implement both the Display and Debug traits:

use std::fmt::{Display, Debug};

fn print<T: Display + Debug>(val: T) {
    println!("{:?}", val);
}
