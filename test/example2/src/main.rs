

fn main() {
    println!("Hello, world!");

    let utf8_bytes = vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]; // "Hello World" in UTF-8 bytes
let my_string = String::from_utf8(utf8_bytes).unwrap();
println!("{}", my_string); // pr



///////////////////////////////
/// 
let valid_utf8_bytes = vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]; // "Hello World" in UTF-8 bytes
let result = String::from_utf8(valid_utf8_bytes);
match result {
    Ok(s) => println!("Valid string: {}", s),
    Err(e) => println!("Error: {}", e),
}

let invalid_utf8_bytes = vec![0xC3, 0x28, 0x2A, 0x29]; // Invalid UTF-8 bytes
let result = String::from_utf8(invalid_utf8_bytes);
match result {
    Ok(s) => println!("Valid string: {}", s),
    Err(e) => println!("Error: {}", e),
}

let utf8_bytes = vec![120, 0x41, 0x75, 0xF0, 0x9F, 0x91, 0x8D, 80, 82, 10, 0xE2, 0x99, 0xAB]; // Invalid UTF-8 bytes
let result = String::from_utf8(utf8_bytes);
match result {
    Ok(s) => println!("Valid string: {}", s),
    Err(e) => println!("Error: {}", e),
}

}
