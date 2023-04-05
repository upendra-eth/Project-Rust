// use draw::draw_line;
use draw::color::draw_line;
use draw::shapes;



fn main() {
    println!("Hello, world!");
    
    let color = RGB { r: 255, g: 255, b: 255 };
draw_line( 2, 3, &color);
    // draw_line(2,3,3);
}
