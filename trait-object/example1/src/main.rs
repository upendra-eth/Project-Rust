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

// Define a function that takes a trait object as an argument
fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
}

// Define a function that takes a trait object as an argument
fn draw_shape_simple(shape: Circle) {
    shape.draw();
}

fn main() {
    // Create a Circle struct and a trait object that points to it
    let circle = Circle { radius: 1.0 };
    let circle_trait_object: &dyn Drawable = &circle;

    // Call the function with the trait object
    draw_shape(circle_trait_object);

    // Call the function with circle type
    draw_shape_simple(circle);
}


///////////////////////////////////////////////////
///////////////////////////////////////////////////
/// 

// You are correct that in this particular example, we could simply call the draw method on the Circle struct directly instead of using a trait object. However, in more complex examples, using trait objects can provide benefits such as abstraction and dynamic dispatch.

// For example, imagine we had multiple types that implemented the Drawable trait, and we wanted to write a function that could work with any of these types. We could use a trait object to abstract over the specific type and write generic code that works with any type that implements the Drawable trait:

// rust
// Copy code
// fn draw_shapes(shapes: &[&dyn Drawable]) {
//     for shape in shapes {
//         shape.draw();
//     }
// }
// Here, we're able to pass in a slice of trait objects that could be any type that implements the Drawable trait. The draw_shapes function is able to call the draw method on each of these trait objects without knowing the specific type of each shape.

// Using trait objects can also allow for dynamic dispatch, which means that the specific method to call is determined at runtime instead of at compile time. This can be useful in situations where you don't know the exact type of an object until runtime.

// In general, the choice of whether to use a trait object or not depends on the specific requirements of your code. If you only need to work with a single type that implements a trait, it may be simpler to just call the trait methods on that type directly. However, if you need to work with multiple types or need the flexibility of dynamic dispatch, trait objects can be a powerful tool.




