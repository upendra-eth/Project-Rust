fn main() {
    let s = String::from("hello, 世界!");  // A string containing ASCII and non-ASCII characters

    let r = String::from("xAu👍PQR");
    for c in s.chars() {

        // Note that the non-ASCII characters in the string are represented as Unicode code points. You can use the as u32 cast to convert each character to its corresponding code point.

        println!("Unicode code point of {} is {} ", c , c as u32); 
        
    }

    for c in r.chars() {

        // Note that the non-ASCII characters in the string are represented as Unicode code points. You can use the as u32 cast to convert each character to its corresponding code point.

        println!("Unicode code point of {} is {} ", c , c as u32); 
        
    }
}
