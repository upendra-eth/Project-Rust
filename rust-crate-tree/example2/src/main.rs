mod module1; // declearing module module1

use module1::*;
fn main() {
    println!("Hello, world!");

    print_module1();
    print_xyz();
}

fn print_xyz() {
    println!("printing xyz");
    print_module1();
}
