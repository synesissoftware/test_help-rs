// examples/strings.rs

use test_helpers::{
    assert_as_str_eq,
    assert_as_str_ne,
};

use base_traits::AsStr;

use std::panic as std_panic;


struct MyString {
    label : String,
}

impl AsStr for MyString {
    fn as_str(&self) -> &str {
        &self.label
    }
}

fn main() {
    {
        println!();
        println!("compare a `MyString` with `String` with same underlying string contents:");

        let expected : String = "abcd".into();
        let actual : MyString = MyString {
            label : "abcd".into()
        };

        // this one passes
        assert_as_str_eq!(expected, actual);
        println!("passes");
        println!();

        // this one does not
        let failed = std_panic::catch_unwind(|| {
            assert_as_str_ne!(expected, actual);
        })
        .is_err();
        assert!(failed, "`assert_as_str_ne!()` was expected to fail but did not");

        println!();
    }

    {
        println!();
        println!("compare a `MyString` with `String` with different underlying string contents:");

        let expected : String = "abcd".into();
        let actual : MyString = MyString {
            label : "abce".into()
        };

        // this one passes
        assert_as_str_ne!(expected, actual);
        println!("passes");
        println!();

        // this one does not
        let failed = std_panic::catch_unwind(|| {
            assert_as_str_eq!(expected, actual);
        })
        .is_err();
        assert!(failed, "`assert_as_str_eq!()` was expected to fail but did not");

        println!();
    }
}


// ///////////////////////////// end of file //////////////////////////// //
