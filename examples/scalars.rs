// examples/scalars.rs

use test_helpers::{
    assert_scalar_eq_approx,
    assert_scalar_ne_approx,
    margin,
    multiplier,
};

use std::panic as std_panic;


fn main() {
    {
        println!();
        println!("compare two f64 instances (with `margin()`):");

        let expected = 123456.0;
        let actual = 123456.01;

        // this one passes
        assert_scalar_ne_approx!(expected, actual, margin(0.0));

        // this one does not
        let failed = std_panic::catch_unwind(|| {
            assert_scalar_eq_approx!(expected, actual, margin(0.0));
        })
        .is_err();
        assert!(failed, "`assert_scalar_eq_approx!()` was expected to fail but did not");
    }

    {
        println!();
        println!("compare two f64 instances (with `multiplier()`):");

        let expected = 123456.0;
        let actual = 123456.01;

        // this one passes
        assert_scalar_ne_approx!(expected, actual, multiplier(0.0));

        // this one does not
        let failed = std_panic::catch_unwind(|| {
            assert_scalar_eq_approx!(expected, actual, multiplier(0.0));
        })
        .is_err();
        assert!(failed, "`assert_scalar_eq_approx!()` was expected to fail but did not");
    }
}


// ///////////////////////////// end of file //////////////////////////// //
