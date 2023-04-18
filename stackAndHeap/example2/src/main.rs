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
