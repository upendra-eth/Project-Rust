Sure, here's an example of how data can be stored in the stack and heap in Rust:

fn main() {
    // Example of data stored on the stack
    let x = 10; // This creates an integer variable 'x' and assigns it the value 10
    let y = 5; // This creates another integer variable 'y' and assigns it the value 5
    let z = x + y; // This creates a third integer variable 'z' and assigns it the value of 'x' plus 'y'

    println!("The value of z is: {}", z); // This prints the value of 'z' to the console
}


In this example, the variables x, y, and z are all stored on the stack. When the program runs, memory is allocated on the stack to store the values of these variables. When the program exits the main function, the memory used by the variables is automatically deallocated.

Here's another example that shows how data can be stored on the heap:

fn main() {
    // Example of data stored on the heap
    let mut v = Vec::new(); // This creates a new empty vector 'v' on the heap
    v.push(1); // This adds the value 1 to the vector 'v'
    v.push(2); // This adds the value 2 to the vector 'v'
    v.push(3); // This adds the value 3 to the vector 'v'

    println!("The values in v are: {:?}", v); // This prints the values in the vector 'v' to the console
}


In this example, the variable v is a vector that is allocated on the heap. When the Vec::new() function is called, memory is allocated on the heap to store the vector. When elements are added to the vector using the push() method, additional memory is allocated on the heap as needed. When the program exits the main function, the memory used by the vector is automatically deallocated.