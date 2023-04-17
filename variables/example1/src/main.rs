fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    println!("The value of x is: {x}");
    mut_var(&mut x);
    println!("The value of x is: {x}");
}

fn mut_var ( x:&mut i32) {
*x= 7;
println!("new value is {}",x)

}
