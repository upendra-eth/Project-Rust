#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //  constructors
    //  Associated functions that aren’t methods are often used for constructors    that will return a new instance of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn rectangle(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::rectangle(20,40);

    // will return a new instance of the struct square

    let square = Rectangle::square(7);

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the rectangle 2 is {} square pixels.",
        rect2.area()
    );

    println!(
        "The area of the square is {} square pixels.",
        square.area()
    );
}
