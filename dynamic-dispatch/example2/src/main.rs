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

// Define a generic function that takes any type that implements the Drawable trait and draws it
fn draw<T: Drawable>(shape: &T) {
    shape.draw();
}

fn main() {
    // Create a Circle struct and a trait object that points to it
    let circle = Circle { radius: 1.0 };
    let circle_trait_object: &dyn Drawable = &circle;


    // Call the generic function with a Circle struct and a trait object
    draw(&circle);
    draw(circle_trait_object);
}
