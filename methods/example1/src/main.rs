#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        rect1.area()
    );
}
