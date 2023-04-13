#[cfg(feature = "xyz")]
pub fn printxyz() {
    println!("printing xyz")
}

#[cfg(not(feature = "xyz"))]
pub fn notprintxyz() {
    println!("printing not-xyz")
}

#[test]
fn test1() {
    printxyz();
}

#[test]
fn test2() {
    notprintxyz();
}
