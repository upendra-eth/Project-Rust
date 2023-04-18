use std::fmt::{Display , Debug};
use std::fmt;

fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct X {
        x: i32,
        y: u32,
    }

    impl Display for X {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "X {{ x: {}, y: {} }}", self.x, self.y)
        }
    }

    let r = X { x: 3, y: 4 };

    print(56);
    print(r);
}

// use std::fmt::{Display, Debug};

fn print<T: Display + Debug>(val: T) {
    println!(" {} ", val);
}

