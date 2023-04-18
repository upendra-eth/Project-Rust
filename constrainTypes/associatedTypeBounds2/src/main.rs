trait Shape {
    type Output;

    fn area(&self) -> Self::Output;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    type Output = f64;

    fn area(&self) -> Self::Output {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    type Output = f64;

    fn area(&self) -> Self::Output {
        self.side * self.side
    }
}

struct ShapeWrapper<'a> {
    shape: &'a dyn Shape<Output = f64>,
}

impl<'a> ShapeWrapper<'a> {
    fn new(shape: &'a dyn Shape<Output = f64>) -> Self {
        ShapeWrapper { shape }
    }
}

fn sum_areas(shapes: &[ShapeWrapper]) -> f64 {
    shapes.iter().map(|s| s.shape.area()).sum()
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let square = Square { side: 2.0 };
    let shapes = vec![ShapeWrapper::new(&square), ShapeWrapper::new(&circle)];

    let total_area = sum_areas(&shapes);

    println!("Total area: {}", total_area);
}

////////////////////////////////////////////////////////
/// 
/// Note that this version of ShapeWrapper is more flexible because it allows
///  you to use any type that implements the Shape trait, not just trait objects.
/// 
/// In Rust, when working with traits, you can choose to either use trait objects or generic types to abstract over the specific types that implement the trait.

// In the original implementation of ShapeWrapper, it used trait objects via the dyn keyword to store a reference to a type that implements the Shape trait. This meant that it could only accept trait objects as input, which can have some performance overhead due to dynamic dispatch.

// However, in the updated version of ShapeWrapper without dyn, it uses a generic type parameter bounded by Shape instead of a trait object. This means that it can accept any concrete type that implements the Shape trait, rather than only accepting trait objects. This approach has the benefit of being more performant because the code is generated at compile-time rather than run-time.

// Therefore, the updated version of ShapeWrapper is more flexible because it can accept any type that implements the Shape trait, not just trait objects.

//////////////////////////////////////////////////
/// 
//////////////////////////////////////////////////

// struct ShapeWrapper<'a, T>
// where
//     T: Shape,
// {
//     shape: &'a T,
// }

// impl<'a, T> ShapeWrapper<'a, T>
// where
//     T: Shape,
// {
//     fn new(shape: &'a T) -> Self {
//         ShapeWrapper { shape }
//     }
// }

// fn sum_areas<T: Shape>(shapes: &[ShapeWrapper<T>]) -> T::Output {
//     shapes.iter().map(|s| s.shape.area()).sum()
// }

// fn main() {
//     let circle = Circle { radius: 3.0 };
//     let square = Square { side: 2.0 };
//     let shapes = vec![ShapeWrapper::new(&square), ShapeWrapper::new(&circle)];

//     let total_area = sum_areas(&shapes);

//     println!("Total area: {}", total_area);
// }
