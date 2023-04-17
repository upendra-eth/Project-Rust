struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self {
            name: name,
            age: age,
        }
    }
    
    fn say_hello(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

struct Greeter(Person);

impl Greeter {
    fn new(person: Person) -> Self {
        Self(person)
    }
    
    fn greet(&self) {
        println!("Greetings!");
        self.0.say_hello();
    }
}

fn main() {
    let person = Person::new("Alice".to_string(), 30);
    let greeter = Greeter::new(person);
    greeter.greet();
}
