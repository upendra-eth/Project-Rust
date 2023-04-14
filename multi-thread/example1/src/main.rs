use std::thread;

fn main() {
    // Create a new thread
    let handle = thread::spawn(|| {
        // Code to be executed by the thread
        println!("Hello from a new thread!");
    });

    // Wait for the thread to finish executing
    handle.join().unwrap();

    // Code executed by the main thread
    println!("Hello from the main thread!");
}
