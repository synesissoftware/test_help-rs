// mod.rs : src/tests

#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(unused_imports))]

use crate as test_helpers;

use test_helpers::{
    traits::ApproximateEqualityEvaluator,
    ComparisonResult,
    margin,
    multiplier,
    zero_margin_or_multiplier,
};

use std::rc as std_rc;


mod TEST_margin {
    #![allow(non_snake_case)]

    use super::*;


    #[test]
    fn TEST_margin_1() {
        let margin_factor = 0.0;
        let m = margin(margin_factor);

        assert_eq!(ComparisonResult::ExactlyEqual, m.evaluate(0.0, 0.0).0);

        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.000001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.00001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.0001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.01, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.1, 0.0).0);
    }

    #[test]
    fn TEST_margin_2() {
        let margin_factor = 0.001;
        let m = margin(margin_factor);

        assert_eq!(ComparisonResult::ExactlyEqual, m.evaluate(0.0, 0.0).0);

        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(0.000001, 0.0).0);
        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(0.00001, 0.0).0);
        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(0.0001, 0.0).0);
        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(0.001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.0010001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.001001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.00101, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.0011, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.01, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.1, 0.0).0);
    }
}


mod TEST_multiplier {
    #![allow(non_snake_case)]

    use super::*;


    #[test]
    fn TEST_multiplier_1() {
        let multiplier_factor = 0.0;
        let m = multiplier(multiplier_factor);

        assert_eq!(ComparisonResult::ExactlyEqual, m.evaluate(0.0, 0.0).0);

        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.000001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.00001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.0001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.01, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.1, 0.0).0);
    }

    #[test]
    fn TEST_multiplier_2() {
        let multiplier_factor = 0.001;
        let m = multiplier(multiplier_factor);

        assert_eq!(ComparisonResult::ExactlyEqual, m.evaluate(0.0, 0.0).0);

        assert_eq!(ComparisonResult::ExactlyEqual, m.evaluate(1.0, 1.0).0);
        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(1.000001, 1.0).0);
        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(1.00001, 1.0).0);
        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(1.0001, 1.0).0);
        assert_eq!(ComparisonResult::ApproximatelyEqual, m.evaluate(1.001, 1.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.0010001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.001001, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.00101, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.0011, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.01, 0.0).0);
        assert_eq!(ComparisonResult::Unequal, m.evaluate(0.1, 0.0).0);
    }
}


mod TEST_SCALAR_ASSERTS {
    #![allow(non_snake_case)]

    use super::*;


    struct CustomEvaluator{}

    impl ApproximateEqualityEvaluator for CustomEvaluator {
        fn evaluate(
            &self,
            expected : f64,
            actual : f64,
        ) -> (
            ComparisonResult, // comparison_result
            Option<f64>,      // margin_factor
            Option<f64>,      // multiplier_factor
        )
        {
            (
                if expected == actual {
                    ComparisonResult::ExactlyEqual
                } else {
                    ComparisonResult::Unequal
                },
                Some(0.0),
                Some(0.0),
            )
        }
    }


    mod TEST_assert_scalar_eq_approx_2_PARAMETER_FOR_EXACTLY_EQUAL_VALUES {
        #![allow(non_snake_case)]

        use std::f64 as std_f64;


        #[test]
        fn TEST_literals() {

            assert_scalar_eq_approx!(-1.23456789e-10, -1.23456789e-10);
            assert_scalar_eq_approx!(-0.123456789, -0.123456789);
            assert_scalar_eq_approx!(-0.1, -0.1);
            assert_scalar_eq_approx!(0.0, 0.0);
            assert_scalar_eq_approx!(0.1, 0.1);
            assert_scalar_eq_approx!(0.123456789, 0.123456789);
            assert_scalar_eq_approx!(1.23456789e+10, 1.23456789e+10);
        }

        #[test]
        fn TEST_f64_ASSOCIATED_CONSTANTS() {

            assert_scalar_eq_approx!(f64::EPSILON, f64::EPSILON);
            assert_scalar_eq_approx!(f64::INFINITY, f64::INFINITY);
            assert_scalar_eq_approx!(f64::MAX, f64::MAX);
            assert_scalar_eq_approx!(f64::MIN, f64::MIN);
            assert_scalar_eq_approx!(f64::MIN_POSITIVE, f64::MIN_POSITIVE);
            assert_scalar_eq_approx!(f64::NEG_INFINITY, f64::NEG_INFINITY);

            #[cfg(feature = "nan-equality")]
            {
                assert_scalar_eq_approx!(f64::NAN, f64::NAN);
            }
            #[cfg(not(feature = "nan-equality"))]
            {
                assert_scalar_ne_approx!(f64::NAN, f64::NAN);
            }
        }

        #[test]
        fn TEST_std_f64_CONSTANTS() {
            use std_f64::consts::*;

            assert_scalar_eq_approx!(E, E);
            assert_scalar_eq_approx!(FRAC_1_PI, FRAC_1_PI);
            assert_scalar_eq_approx!(FRAC_1_SQRT_2, FRAC_1_SQRT_2);
            assert_scalar_eq_approx!(FRAC_2_PI, FRAC_2_PI);
            assert_scalar_eq_approx!(FRAC_2_SQRT_PI, FRAC_2_SQRT_PI);
            assert_scalar_eq_approx!(FRAC_PI_2, FRAC_PI_2);
            assert_scalar_eq_approx!(FRAC_PI_3, FRAC_PI_3);
            assert_scalar_eq_approx!(FRAC_PI_4, FRAC_PI_4);
            assert_scalar_eq_approx!(FRAC_PI_6, FRAC_PI_6);
            assert_scalar_eq_approx!(FRAC_PI_8, FRAC_PI_8);
            assert_scalar_eq_approx!(LN_10, LN_10);
            assert_scalar_eq_approx!(LN_2, LN_2);
            assert_scalar_eq_approx!(LOG10_2, LOG10_2);
            assert_scalar_eq_approx!(LOG10_E, LOG10_E);
            assert_scalar_eq_approx!(LOG2_10, LOG2_10);
            assert_scalar_eq_approx!(LOG2_E, LOG2_E);
            assert_scalar_eq_approx!(PI, PI);
            assert_scalar_eq_approx!(SQRT_2, SQRT_2);
            assert_scalar_eq_approx!(TAU, TAU);
        }

        #[test]
        #[cfg(rustc_1_94_or_newer)]
        fn TEST_std_f64_CONSTANTS_rust_1_94() {
            use std_f64::consts::*;

            assert_scalar_eq_approx!(EULER_GAMMA, EULER_GAMMA);
            assert_scalar_eq_approx!(GOLDEN_RATIO, GOLDEN_RATIO);
        }

        #[test]
        #[cfg(feature = "nightly-constants")]
        fn TEST_std_f64_CONSTANTS_NIGHTLY() {
            use std_f64::consts::*;

            assert_scalar_eq_approx!(FRAC_1_SQRT_PI, FRAC_1_SQRT_PI);
            assert_scalar_eq_approx!(FRAC_1_SQRT_2PI, FRAC_1_SQRT_2PI);
            assert_scalar_eq_approx!(FRAC_1_SQRT_3, FRAC_1_SQRT_3);
            assert_scalar_eq_approx!(FRAC_1_SQRT_5, FRAC_1_SQRT_5);
            assert_scalar_eq_approx!(SQRT_3, SQRT_3);
            assert_scalar_eq_approx!(SQRT_5, SQRT_5);
        }
    }

    #[test]
    #[cfg_attr(not(feature = "nan-equality"), should_panic(expected = "assertion failed: failed to verify approximate equality: expected=NaN, actual=NaN, margin_factor=0.0001, multiplier_factor=0.000001"))]
    fn TEST_assert_scalar_eq_approx_2_PARAMETER_WITH_NAN() {

        assert_scalar_eq_approx!(f64::NAN, f64::NAN);
    }
    #[test]
    #[cfg_attr(feature = "nan-equality", should_panic(expected = "assertion failed: failed to verify approximate inequality: expected=NaN, actual=NaN, margin_factor=0.0001, multiplier_factor=0.000001"))]
    fn TEST_assert_scalar_ne_approx_2_PARAMETER_WITH_NAN() {

        assert_scalar_ne_approx!(f64::NAN, f64::NAN);
    }

    /// Demonstrate that feature `"nan-equality"` only changes stock
    /// behaviour
    #[test]
    fn TEST_assert_scalar_ne_approx_3_PARAMETER_WITH_CustomEvaluator() {

        assert_scalar_ne_approx!(f64::NAN, f64::NAN, CustomEvaluator{});
    }

    #[test]
    fn TEST_assert_scalar_eq_approx_2_PARAMETER_FOR_APPROXIMATELY_EQUAL_VALUES() {

        assert_scalar_eq_approx!(0.12345678, 0.12345679);
        assert_scalar_eq_approx!(0.12345678, 0.12345677);
    }

    #[test]
    fn TEST_assert_scalar_eq_approx_3_PARAMETER_margin_FOR_APPROXIMATELY_EQUAL_VALUES() {
        assert_scalar_eq_approx!(0.12345678, 0.12345679, margin(0.1));
        assert_scalar_eq_approx!(0.12345678, 0.12345679, margin(0.01));
        assert_scalar_eq_approx!(0.12345678, 0.12345679, margin(0.001));
        assert_scalar_eq_approx!(0.12345678, 0.12345679, margin(0.0001));
        assert_scalar_eq_approx!(0.12345678, 0.12345679, margin(0.00001));
        assert_scalar_eq_approx!(0.12345678, Box::new(0.12345679), margin(0.000001));
        assert_scalar_eq_approx!(std_rc::Rc::new(0.123456780), 0.12345679, margin(0.0000001));
        assert_scalar_eq_approx!(0.12345678, 0.12345679, margin(0.00000001));
    }

    #[test]
    #[should_panic(expected = "assertion failed: failed to verify approximate equality: expected=0.12345678, actual=0.12345679, margin_factor=0.000000001")]
    fn TEST_assert_scalar_eq_approx_3_PARAMETER_margin_SHOULD_FAIL_1() {
        assert_scalar_eq_approx!(0.12345678, 0.12345679, margin(0.000000001));
    }

    #[test]
    #[should_panic(expected = "assertion failed: failed to verify approximate inequality: expected=0.12345678, actual=0.12345678, margin_factor=0.0001, multiplier_factor=0.000001")]
    fn TEST_assert_scalar_ne_approx_2_PARAMETER_FOR_APPROXIMATELY_EQUAL_VALUES_SHOULD_FAIL_1() {

        assert_scalar_ne_approx!(0.12345678, 0.12345678);
    }
}


mod TEST_VECTOR_ASSERTS {
    #![allow(non_snake_case)]

    use super::*;


    #[test]
    fn TEST_assert_vector_eq_approx_2_PARAMETER_EMPTY_ARRAY_INSTANCES() {
        let expected : [f64; 0] = [];
        let actual : [f64; 0] = [];

        assert_vector_eq_approx!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "assertion failed: failed to verify approximate inequality for vectors")]
    fn TEST_assert_vector_ne_approx_2_PARAMETER_EMPTY_ARRAY_INSTANCES() {
        let expected : [f64; 0] = [];
        let actual : [f64; 0] = [];

        assert_vector_ne_approx!(expected, actual);
    }

    #[test]
    fn TEST_assert_vector_eq_approx_3_PARAMETER_EMPTY_SLICE_INSTANCES() {
        let expected : &[f64] = &[];
        let actual : &[f64] = &[];

        assert_vector_eq_approx!(expected, actual, margin(0.0001));
    }

    #[test]
    fn TEST_assert_vector_eq_approx_2_PARAMETER_EMPTY_Vec_INSTANCES() {
        let expected : Vec<f64> = Vec::new();
        let actual : Vec<f64> = Vec::new();

        assert_vector_eq_approx!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "assertion failed: failed to verify approximate equality for vectors: expected-length 2 differs from actual-length 1")]
    fn TEST_assert_vector_eq_approx_2_PARAMETER_SLICE_INSTANCES_DIFFERENT_LENGTHS() {
        let expected : &[f64] = &[ -2.0, -3.0 ];
        let actual : &[f64] = &[ 0.0 ];

        assert_vector_eq_approx!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "assertion failed: failed to verify approximate equality for vectors: at index 1 expected=-3.0, actual=-3.001, margin_factor=0.01, multiplier_factor=0.0001")]
    fn TEST_assert_vector_eq_approx_3_PARAMETER_VECTORS_SAME_LENGTH_DIFFERENT_ELEMENTS() {
        let expected : &[f64] = &[ -2.0, -3.0, -4.0 ];
        let actual = Vec::from([ -2.0, -3.001, -4.0 ]);

        assert_vector_eq_approx!(expected, actual, zero_margin_or_multiplier(0.0001, 0.01));
    }

    #[test]
    fn TEST_assert_vector_eq_approx_3_PARAMETER_VECTORS_SAME_LENGTH_DIFFERENT_ELEMENTS_WITH_PERMISSIVE_multiplier() {
        let expected : &[f64] = &[ -2.0, -3.0, -4.0 ];
        let actual = Vec::from([ -2.0, -3.000001, -4.0 ]);

        assert_vector_eq_approx!(expected, actual, multiplier(0.01));
    }
}


mod TEST_README_EXAMPLES {
    #![allow(non_snake_case)]

    use super::*;


    #[test]
    fn TEST_example_test_of_scalar_evaluation() {
        let expected = 3.0;
        let actual = 3.0001;
        assert_scalar_eq_approx!(expected, actual, margin(0.0001));
    }

    #[test]
    fn TEST_example_test_of_vector_evaluation() {
        let expected = &[ 3.0, -40404.0, 1.23456 ];
        let actual = Vec::from([ 3.0, -40410.0, 1.234567 ]);
        assert_vector_eq_approx!(expected, actual, multiplier(0.00015));
    }

}


// ///////////////////////////// end of file //////////////////////////// //
