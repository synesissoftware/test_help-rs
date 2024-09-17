// examples/scalar.rs

use test_helpers::{
    assert_scalar_eq_approx,
    assert_scalar_ne_approx,
    margin,
    multiplier,
};


fn main() {
    {
        println!();
        println!("compare two f64 instances:");

        let expected = 123456.0;
        let actual = 123456.01;

        // this one passes
        assert_scalar_ne_approx!(expected, actual, multiplier(0.0));

        // this one does not
        assert_scalar_eq_approx!(expected, actual, multiplier(0.0));
    }
}


// ///////////////////////////// end of file //////////////////////////// //
