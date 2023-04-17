trait Container<T> {
    fn add(&mut self, item: T);
}

trait Container2 {
    fn add(&self);
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Container<T> for Stack<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

impl<T> Container2 for T {
    fn add1(&self){
        println!("give the thing ")
    }
}

fn main() {
    let mut stack = Stack { items: vec![] };

    stack.add(1);

    let mut stack1 = Stack { items: vec![] };

    stack1.add("two");
    let item1 = stack.items;
    let item2 = stack1.items.pop().unwrap();
    println!("{:?} {}", item1, item2); // prints "two 1"

    let mut x:i32 = 50;
    x.add();
}
