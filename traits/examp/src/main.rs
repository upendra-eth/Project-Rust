trait Animal {
    fn name(&self) -> &'static str;

    // fn talk(&self) {
    //     println!("{} says hello!", self.name());
    // }

    fn eat(&self, food: &str) {
        println!("{} is eating {}.", self.name(), food);
    }

    fn walk(&self) {
        println!("{} is walking.", self.name());
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    // fn talk(&self) {
    //     println!("{} says meow!", self.name());
    // }
}


fn main() {
    let cat = Cat { name: "Whiskers" };
    // cat.talk();
    cat.eat("fish");
    cat.walk();

 
}
