struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 3, y: 4 };
    let longest_point;

    {
        let shorter_point;
        if point1.x + point1.y < point2.x + point2.y {
            shorter_point = &point1;
        } else {
            shorter_point = &point2;
        }
        longest_point = longest(shorter_point, &point2);
    }

    println!("The longest point is ({}, {})", longest_point.x, longest_point.y);
}

fn longest<'a>(x: &'a Point, y: &'a Point) -> &'a Point {
    if x.x + x.y > y.x + y.y {
        x
    } else {
        y
    }
}
