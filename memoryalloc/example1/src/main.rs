    fn main() {
        let x = 5;
        {
            let y = 10;
            let z = x + y;
            println!("{}", z);
            func1(&y);
            func2(&y);
        }
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        println!("{:?}", v);
    }

    fn func1(y: &i32) {
        println!("printing {}", y)
    }

    fn func2(y: &i32) {
        println!("printing again {}", y)
    }
