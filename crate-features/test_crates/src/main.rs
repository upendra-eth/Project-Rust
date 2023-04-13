use print_somthing::printxyz;
// use print_somthing::notprintxyz;
use my_crate::fibonacci;

fn main() {
    println!("Hello, world!");
    // println!("using print_xyz {:?}", printxyz());
    // println!("using print_xyz {:?}", notprintxyz());

    println!(
        "i am using my_crate which gives fibo of 9! = {}",
        fibonacci(0)
    )
}
