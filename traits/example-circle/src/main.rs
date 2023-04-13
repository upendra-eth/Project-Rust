struct circle {
    x:u32,
    y:u32,
    radius:u32,

}
trait Properties {

    fn area (&self);
    // fn area(&self) {
    //     let r = self.radius;
    //     let area = 3.14 * r * r;
    //     println!("area of circle with radius {} is {}", r, area);
    // }
}
 impl Properties for circle{
    
        fn area(&self) {
            let r = self.radius;
            let area =  r * r;
            println!("area of circle with radius {} is {}", r, area);
        }
    
 }

fn main() {
    println!("Hello, world!");

    let bigCircle = circle{
        x:2,
        y:3,
        radius:5,
    };

    bigCircle.area();
}
