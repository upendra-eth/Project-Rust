struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

fn main() {
    // Example of data stored on the stack and heap
    let p1 = Person::new(String::from("Alice"), 25); // This creates a new Person struct on the heap
    let p2 = Person::new(String::from("Bob"), 30); // This creates another new Person struct on the heap

    let p3 = Person { // This creates a third Person struct on the stack
        name: String::from("Charlie"),
        age: 35,
    };

    p1.greet(); // This calls the 'greet' method on the 'p1' Person struct
    p2.greet(); // This calls the 'greet' method on the 'p2' Person struct
    p3.greet(); // This calls the 'greet' method on the 'p3' Person struct
}




In this example, the Person struct contains two fields: a name field of type String and an age field of type u32. When a new Person struct is created using the new() method, memory is allocated on the heap to store the String field. When the Person structs are passed to the greet() method, a reference to the struct is passed by reference on the stack.

In the main() function, three Person structs are created: p1 and p2 are created using the new() method and are allocated on the heap, while p3 is created on the stack using the struct literal syntax.

When the greet() method is called on each Person struct, it prints a greeting to the console that includes the name and age fields of the struct.

Overall, this example demonstrates how Rust programs can store data on both the stack and the heap, and how Rust's ownership and borrowing rules help to ensure that memory is allocated and deallocated safely and efficiently.

