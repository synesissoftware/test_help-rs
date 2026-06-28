// examples/vectors.rs

use test_helpers::{
    assert_vector_eq_approx,
    assert_vector_ne_approx,
    margin,
    multiplier,
};

use std::panic as std_panic;


fn main() {
    {
        println!();
        println!("compare two vectors (README-style, with `multiplier()`):");

        let expected = &[3.0, -40404.0, 1.23456];
        let actual = Vec::from([3.0, -40410.0, 1.234567]);

        assert_vector_eq_approx!(expected, actual, multiplier(0.00015));
    }

    {
        println!();
        println!("compare two vectors (with `margin()`):");

        let expected = &[1.0, 2.0, 3.0];
        let actual = Vec::from([1.0, 2.001, 3.0]);

        // this one passes
        assert_vector_ne_approx!(expected, actual, margin(0.0));

        // this one does not
        let failed = std_panic::catch_unwind(|| {
            assert_vector_eq_approx!(expected, actual, margin(0.0));
        })
        .is_err();
        assert!(failed, "expected assert_vector_eq_approx! to fail");
    }

    {
        println!();
        println!("compare two vectors (with `multiplier()`):");

        let expected = &[1.0, 2.0, 3.0];
        let actual = Vec::from([1.0, 2.001, 3.0]);

        // this one passes
        assert_vector_ne_approx!(expected, actual, multiplier(0.0));

        // this one does not
        let failed = std_panic::catch_unwind(|| {
            assert_vector_eq_approx!(expected, actual, multiplier(0.0));
        })
        .is_err();
        assert!(failed, "expected assert_vector_eq_approx! to fail");
    }
}


// ///////////////////////////// end of file //////////////////////////// //
