mod module1;

use module1::module11::print_module11;

fn main() {
    println!("Hello, world!");
    print_module11();
    module_xyz::print_xyz();
}

mod module_xyz {
    use super::*;
    pub fn print_xyz() {
        println!("print this xyz");
    }
}
