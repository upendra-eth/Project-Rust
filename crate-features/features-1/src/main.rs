struct MyStruct {
    name: String,
}

#[cfg(feature = "xyz")]
fn debug_main() {
    let my_struct = MyStruct {
        name: "features-used".to_string(),
    };
    println!("{}", my_struct.name);
}

#[cfg(not(feature = "xyz"))]
fn main() {
    let my_struct = MyStruct {
        name: "fetures-not-used".to_string(),
    };
    println!("{}", my_struct.name);
}

#[cfg(feature = "xyz")]
fn main() {
    debug_main();
}
