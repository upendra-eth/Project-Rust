use rand;

// Define a trait with a single method
trait Drawable {
    fn draw(&self);
}

// Define a struct that implements the Drawable trait
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}.", self.radius);
    }
}

// Define a struct that implements the Drawable trait
struct Square {
    side_length: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side length {}.", self.side_length);
    }
}

// Define a function that takes a trait object as an argument
fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
}

fn main() {
    // Create a Circle or Square struct randomly
    let shape: Box<dyn Drawable> = if rand::random() { 
        Box::new(Circle { radius: 1.0 }) 
    } else { 
        Box::new(Square { side_length: 2.0 }) 
    };

    // Call the function with the trait object
    draw_shape(&*shape);
}
