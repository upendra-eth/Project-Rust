use std::any::type_name;

trait Container2<T> {
    fn add1(&self ,x:T );
}

impl<T> Container2<T> for T {
    fn add1(&self ,x:T) {
        
       let  items: Vec<T> =;
         
        items.push(x);
        // println!("this is : {}", items[0]);
        // println!("Type of x: {}", type_name::<typeof(x)>());
    } 
}

fn main() {
    let mut x : Vec<i32>= vec![];
    // let mut x = vec![1];
    x.add1(x);
            // println!("this is : {}", items[0]);


}
