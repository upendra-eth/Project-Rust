trait mytrait {
    fn my_function(&self);
}

struct p {
    ram: bool,
    syaam: u32,
}



impl mytrait for i32 {
    fn my_function(&self) {
        println!("Specialized implementation for i32 is {}",&self);
    }
}

impl mytrait for u32 {
    fn my_function(&self) {
        println!("Specialized implementation for u32  is {}" , self);
    }
}

impl mytrait for p {
    fn my_function(&self) {
        let q = self.syaam;
        println!("print special bool value {}",q);
    }
}

fn main() {
    println!("Hello, world!");
    let y = 89;
    let x: u32 = 5;
    let z = p {
        ram: true,
        syaam:3,
    };
    y.my_function();
    x.my_function();
    z.my_function();

}