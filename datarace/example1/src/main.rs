use std::thread;

fn main() {
    let mut x = 0;
    let mut handles = Vec::new();

    for _ in 0..10 {
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                x += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", x);
}
