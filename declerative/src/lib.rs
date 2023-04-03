


#[macro_export]
macro_rules! makevec {
    ( $( $x:expr ),* ) => {
        {
            let mut x = Vec::new();
            $(
                x.push($x);
            )*
            x
        }
    };
}
#[test]
fn check() {
    let val = makevec!(4,5);
    assert_eq!(val[0], 4);
}


#[test]
fn check2() {
    let x: Vec<u32> = makevec!(4);
    assert!(x.is_empty());
}


