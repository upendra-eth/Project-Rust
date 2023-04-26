fn main() {
    use parent::*;
    println!("Hello, world!");
    outer_func();
}

mod parent {
    pub fn outer_func() {
        println!("print parentmode function1 ");

        inner_mod::inner_func();
        inner_mod::inner_func2();
        self::inner_mod::inner_func2();
        inner_mod::use_super();

        println!(" ends here ////////////////////// outer_func ");
    }

    fn root_func() {
        println!("print root function ");
        println!("/////////// end root func /////////// ");
    }

    mod inner_mod {

        use super::*;

        pub fn inner_func() {
            println!("print inner func");
            println!(" ends here /////////////////// inner_func ");
        }

        pub fn inner_func2() {
            println!("print inner 2 func");
            inner_func();

            // The `self` keyword refers to the current module scope - in this case `my`.
            // Calling `self::function()` and calling `function()` directly both give
            // the same result, because they refer to the same function.
            self::inner_func();
            println!("ends here /////////////// inner_func2  ");
        }

        pub fn use_super() {
            println!("we are using uper to call function from parent mod here ");
            root_func();
            // super::outer_func();
            println!("ends here ///////super  ");
        }
    }
}
