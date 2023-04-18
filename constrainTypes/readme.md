# Different ways to constrain types in Rust:

1 -Trait Bounds:

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

2 -Type Bounds:

Type bounds specify that a type must be of a certain kind or type. For example, this code specifies that the generic type T must be an integer:


fn add<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + Copy>(a: T, b: T) -> T {
    a + b
}
In this example, the add function can only be called with types that are integers, which allows the function to perform arithmetic operations on the values.

3 - Lifetime Bounds:

Lifetime bounds specify the lifetime of a reference, ensuring that it lives at least as long as the data it references. For example, this code specifies that the lifetime of the reference x must be at least as long as the lifetime of the reference y:


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
In this example, the longest function returns a reference to the longer of two strings. The lifetime 'a ensures that the reference returned by the function lives at least as long as the data it references.

4 - Where Clauses:

Where clauses provide an alternative syntax for specifying trait bounds and type bounds. For example, this code specifies that the generic type T must implement both the Display and Debug traits, and that it must be a reference:


use std::fmt::{Display, Debug};

fn print<T>(val: T)
where
    T: Display + Debug + AsRef<str>,
{
    println!("{:?}", val.as_ref());
}

In this example, the print function can only be called with types that implement Display and Debug, and that are references. The AsRef<str> trait allows the function to treat the value as a string reference, regardless of its underlying type.

5 - Associated Type Bounds:

Associated type bounds specify that a type must have a certain associated type. For example, this code specifies that the generic type K must have a Key associated type, which can be used to index into a collection:


trait Collection {
    type Key;
    fn get(&self, key: Self::Key) -> Option<&Self::Item>;
    type Item;
}

fn get_first<C: Collection>(collection: &C) -> Option<&C::Item> {
    collection.get(collection::Key::first())
}