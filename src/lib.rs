// lib.rs : test_help-rs

#![allow(non_camel_case_types)]


// /////////////////////////////////////////////////////////
// crate-level feature definitions

#![cfg_attr(test, feature(more_float_constants))]


// /////////////////////////////////////////////////////////
// crate-level feature discrimination


// /////////////////////////////////////////////////////////
// imports

use std::{
    convert as std_convert,
    fmt as std_fmt,
};


// /////////////////////////////////////////////////////////
// constants

/// Constants.
pub mod constants {

    /// The default margin.
    pub const DEFAULT_MARGIN : f64 = 0.0001;

    /// The default multiplier.
    pub const DEFAULT_MULTIPLIER : f64 = 0.000001;
}


// /////////////////////////////////////////////////////////
// types

/// Comparison result type.
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum ComparisonResult {
    /// The comparands are exactly equal.
    ExactlyEqual,
    /// The comparands are equal within the tolerance of the given margin or
    /// multiplier.
    ApproximatelyEqual,
    /// The comparands are not equal within the tolerance of the given
    /// margin or multiplier.
    Unequal,
}

/// Vector comparison result type.
#[derive(Debug)]
pub enum VectorComparisonResult {
    ExactlyEqual,
    ApproximatelyEqual,
    DifferentLengths {
        expected_length : usize,
        actual_length :   usize,
    },
    UnequalElements {
        index_of_first_unequal_element :          usize,
        expected_value_of_first_unequal_element : f64,
        actual_value_of_first_unequal_element :   f64,
    },
}


/// Traits.
pub mod traits {
    use super::ComparisonResult;

    use base_traits::ToF64;

    use std::fmt as std_fmt;


    /// Trait that defines a mechanism for performing approximate equality
    /// evaluation.
    pub trait ApproximateEqualityEvaluator {
        fn evaluate(
            &self,
            expected : f64,
            actual : f64,
        ) -> (
            ComparisonResult, // comparison_result
            Option<f64>,      // margin_factor
            Option<f64>,      // multiplier_factor
        );
    }

    /// Trait that allows an implementing type instance to be evaluated with the
    /// constructs of this crate.
    ///
    /// NOTE: it is implemented for any types that implement
    /// `base_traits::ToF64` (and `std::fmt::Debug`).
    pub trait TestableAsF64: std_fmt::Debug {
        fn testable_as_f64(&self) -> f64;
    }

    impl<T> TestableAsF64 for T
    where
        T : ToF64 + std_fmt::Debug,
    {
        fn testable_as_f64(&self) -> f64 {
            self.to_f64()
        }
    }
}


mod internal {

    use super::{
        traits::ApproximateEqualityEvaluator,
        utils::{
            compare_approximate_equality_by_margin,
            compare_approximate_equality_by_multiplier,
            compare_approximate_equality_by_zero_margin_or_multiplier,
        },
        ComparisonResult,
    };


    /// T.B.C.
    #[derive(Debug)]
    pub struct MarginEvaluator {
        pub(crate) factor : f64,
    }

    /// T.B.C.
    #[derive(Debug)]
    pub struct MultiplierEvaluator {
        pub(crate) factor : f64,
    }

    /// T.B.C.
    #[derive(Debug)]
    pub struct ZeroMarginOrMultiplierEvaluator {
        pub(crate) multiplier_factor :  f64,
        pub(crate) zero_margin_factor : f64,
    }

    // Trait implementations

    impl ApproximateEqualityEvaluator for MarginEvaluator {
        fn evaluate(
            &self,
            expected : f64,
            actual : f64,
        ) -> (
            ComparisonResult, // comparison_result
            Option<f64>,      // margin_factor
            Option<f64>,      // multiplier_factor
        ) {
            let comparison_result = compare_approximate_equality_by_margin(expected, actual, self.factor);

            (comparison_result, Some(self.factor), None)
        }
    }

    impl ApproximateEqualityEvaluator for MultiplierEvaluator {
        fn evaluate(
            &self,
            expected : f64,
            actual : f64,
        ) -> (
            ComparisonResult, // comparison_result
            Option<f64>,      // margin_factor
            Option<f64>,      // multiplier_factor
        ) {
            let comparison_result = compare_approximate_equality_by_multiplier(expected, actual, self.factor);

            (comparison_result, None, Some(self.factor))
        }
    }

    impl ApproximateEqualityEvaluator for ZeroMarginOrMultiplierEvaluator {
        fn evaluate(
            &self,
            expected : f64,
            actual : f64,
        ) -> (
            ComparisonResult, // comparison_result
            Option<f64>,      // margin_factor
            Option<f64>,      // multiplier_factor
        ) {
            let comparison_result = compare_approximate_equality_by_zero_margin_or_multiplier(
                expected,
                actual,
                self.multiplier_factor,
                self.zero_margin_factor,
            );

            (
                comparison_result,
                Some(self.zero_margin_factor),
                Some(self.multiplier_factor),
            )
        }
    }
}


mod utils {
    use super::ComparisonResult;


    /// T.B.C.
    pub(crate) fn compare_approximate_equality_by_margin(
        expected : f64,
        actual : f64,
        margin_factor : f64,
    ) -> ComparisonResult {
        debug_assert!(
            margin_factor >= 0.0,
            "`margin_factor` must not be negative, but {margin_factor} given"
        );

        if expected == actual {
            return ComparisonResult::ExactlyEqual;
        }

        #[cfg(feature = "nan-equality")]
        {
            if expected.is_nan() && actual.is_nan() {
                return ComparisonResult::ExactlyEqual;
            }
        }

        // TODO: determine if can elide this explicit check
        if 0.0 == margin_factor {
            return ComparisonResult::Unequal;
        }

        let expected_lo = expected - margin_factor;
        let expected_hi = expected + margin_factor;

        result_from_range_(expected_lo, expected_hi, actual)
    }

    /// T.B.C.
    pub(crate) fn compare_approximate_equality_by_multiplier(
        expected : f64,
        actual : f64,
        multiplier_factor : f64,
    ) -> ComparisonResult {
        debug_assert!(
            multiplier_factor >= 0.0,
            "`multiplier_factor` must not be negative, but {multiplier_factor} given"
        );

        if expected == actual {
            return ComparisonResult::ExactlyEqual;
        }

        #[cfg(feature = "nan-equality")]
        {
            if expected.is_nan() && actual.is_nan() {
                return ComparisonResult::ExactlyEqual;
            }
        }

        // TODO: determine if can elide this explicit check
        if 0.0 == multiplier_factor {
            return ComparisonResult::Unequal;
        }

        let expected_lo = expected * (1.0 - multiplier_factor);
        let expected_hi = expected * (1.0 + multiplier_factor);

        result_from_range_(expected_lo, expected_hi, actual)
    }

    /// T.B.C.
    pub(crate) fn compare_approximate_equality_by_zero_margin_or_multiplier(
        expected : f64,
        actual : f64,
        multiplier_factor : f64,
        margin_factor : f64,
    ) -> ComparisonResult {
        debug_assert!(
            multiplier_factor >= 0.0,
            "`multiplier_factor` must not be negative, but {multiplier_factor} given"
        );
        debug_assert!(
            margin_factor >= 0.0,
            "`margin_factor` must not be negative, but {margin_factor} given"
        );

        if expected == actual {
            return ComparisonResult::ExactlyEqual;
        }

        #[cfg(feature = "nan-equality")]
        {
            if expected.is_nan() && actual.is_nan() {
                return ComparisonResult::ExactlyEqual;
            }
        }

        let (expected_lo, expected_hi) = if 0.0 == expected || 0.0 == actual {
            // TODO: determine if can elide this explicit check
            if 0.0 == margin_factor {
                return ComparisonResult::Unequal;
            }

            let expected_lo = expected - margin_factor;
            let expected_hi = expected + margin_factor;

            (expected_lo, expected_hi)
        } else {
            // TODO: determine if can elide this explicit check
            if 0.0 == multiplier_factor {
                return ComparisonResult::Unequal;
            }

            let expected_lo = expected * (1.0 - multiplier_factor);
            let expected_hi = expected * (1.0 + multiplier_factor);

            (expected_lo, expected_hi)
        };

        result_from_range_(expected_lo, expected_hi, actual)
    }

    fn result_from_range_(
        lo : f64,
        hi : f64,
        actual : f64,
    ) -> ComparisonResult {
        let r = if lo <= hi { lo..=hi } else { hi..=lo };

        if r.contains(&actual) {
            ComparisonResult::ApproximatelyEqual
        } else {
            ComparisonResult::Unequal
        }
    }


    #[cfg(test)]
    #[rustfmt::skip]
    mod tests {
        #![allow(non_snake_case)]


        use super::{
            compare_approximate_equality_by_margin,
            compare_approximate_equality_by_multiplier,
            compare_approximate_equality_by_zero_margin_or_multiplier,
        };

        use super::super::ComparisonResult;


        #[test]
        fn TEST_compare_approximate_equality_by_margin_1() {

            // expected == actual == 0.0
            {
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.0));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.0000001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.000001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.00001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.0001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.01));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.1));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_margin(0.0, 0.0, 0.5));
            }

            // expected == 0.0, actual == 0.1, f == *
            {
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.0, 0.1, 0.0));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.0, 0.1, 0.0000001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.0, 0.1, 0.000001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.0, 0.1, 0.00001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.0, 0.1, 0.0001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.0, 0.1, 0.001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.0, 0.1, 0.01));
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_margin(0.0, 0.1, 0.1));
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_margin(0.0, 0.1, 0.5));
            }

            // expected == 0.099, actual == 0.1, f == *
            {
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.099, 0.1, 0.0));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.099, 0.1, 0.0000001));        // expected [ 0.0989999-0.0990001 ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.099, 0.1, 0.000001));         // expected [  0.098999-0.099001  ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.099, 0.1, 0.00001));          // expected [   0.09899-0.09901   ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_margin(0.099, 0.1, 0.0001));           // expected [    0.0989-0.0991    ]
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_margin(0.099, 0.1, 0.001)); // expected [     0.098-0.1       ]
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_margin(0.099, 0.1, 0.01));  // expected [     0.089-0.109     ]
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_margin(0.099, 0.1, 0.02));  // expected [     0.089-0.119     ]
            }
        }

        #[test]
        fn TEST_compare_approximate_equality_by_multiplier_1() {

            // expected == actual == 0.0
            {
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.0));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.0000001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.000001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.00001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.0001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.01));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.1));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_multiplier(0.0, 0.0, 0.5));
            }

            // expected == 0.0, actual == 0.1, f == *
            {
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.0));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.0000001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.000001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.00001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.0001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.01));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.1));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.0, 0.1, 0.5));
            }

            // expected == 0.099, actual == 0.1, f == *
            {
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.0));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.0000001)); // expected [ 0.0989999901-0.0990000099 ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.000001));   // expected [  0.098999901-0.099000099  ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.00001));     // expected [   0.09899901-0.09900099   ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.0001));       // expected [    0.0989901-0.0990099    ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.001));         // expected [     0.098901-0.099099     ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.01));           // expected [      0.09801-0.09999      ]
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.1));  // expected [       0.0891-0.1089       ]
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_multiplier(0.099, 0.1, 0.5));  // expected [       0.0495-0.1485       ]
            }
        }

        #[test]
        fn TEST_compare_approximate_equality_by_zero_margin_or_multiplier_1() {

            // expected == actual == 0.0
            {
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.0, 0.0));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.0000001, 0.0000001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.000001, 0.000001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.00001, 0.00001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.0001, 0.0001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.001, 0.001));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.01, 0.01));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.1, 0.1));
                assert_eq!(ComparisonResult::ExactlyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.0, 0.5, 0.5));
            }

            // expected == 0.0, actual == 0.1, f == *
            {
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.0, 0.0));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.0000001, 0.0000001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.000001, 0.000001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.00001, 0.00001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.0001, 0.0001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.001, 0.001));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.01, 0.01));
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.1, 0.1));
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.0, 0.1, 0.5, 0.5));
            }

            // expected == 0.099, actual == 0.1, f == *
            {
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.0, 0.0));
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.0000001, 0.0000001)); // expected [ 0.0989999901-0.0990000099 ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.000001, 0.000001));     // expected [  0.098999901-0.099000099  ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.00001, 0.00001));         // expected [   0.09899901-0.09900099   ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.0001, 0.0001));             // expected [    0.0989901-0.0990099    ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.001, 0.001));                 // expected [     0.098901-0.099099     ]
                assert_eq!(ComparisonResult::Unequal, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.01, 0.01));                     // expected [      0.09801-0.09999      ]
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.1, 0.1));              // expected [       0.0891-0.1089       ]
                assert_eq!(ComparisonResult::ApproximatelyEqual, compare_approximate_equality_by_zero_margin_or_multiplier(0.099, 0.1, 0.5, 0.5));              // expected [       0.0495-0.1485       ]
            }
        }
    }
}


// /////////////////////////////////////////////////////////
// API functions

pub fn evaluate_scalar_eq_approx<T_expected, T_actual>(
    expected : &T_expected,
    actual : &T_actual,
    evaluator : &dyn traits::ApproximateEqualityEvaluator,
) -> (
    ComparisonResult, // comparison_result
    Option<f64>,      // margin_factor
    Option<f64>,      // multiplier_factor
)
where
    T_expected : traits::TestableAsF64 + std_fmt::Debug,
    T_actual : traits::TestableAsF64 + std_fmt::Debug,
{
    let (expected, actual) = {
        let expected : &dyn traits::TestableAsF64 = expected;
        let actual : &dyn traits::TestableAsF64 = actual;

        let expected = expected.testable_as_f64();
        let actual = actual.testable_as_f64();

        (expected, actual)
    };

    evaluator.evaluate(expected, actual)
}

pub fn evaluate_vector_eq_approx<T_expected, T_actual, T_expectedElement, T_actualElement>(
    expected : &T_expected,
    actual : &T_actual,
    evaluator : &dyn traits::ApproximateEqualityEvaluator,
) -> (
    VectorComparisonResult, // comparison_result
    Option<f64>,            // margin_factor
    Option<f64>,            // multiplier_factor
)
where
    T_expected : std_convert::AsRef<[T_expectedElement]>,
    T_actual : std_convert::AsRef<[T_actualElement]>,
    T_expectedElement : traits::TestableAsF64 + std_fmt::Debug,
    T_actualElement : traits::TestableAsF64 + std_fmt::Debug,
{
    /*
    let expected_param = expected;
    let actual_param = actual;
     */

    let expected = expected.as_ref();
    let actual = actual.as_ref();

    let expected_length = expected.len();
    let actual_length = actual.len();

    if expected_length != actual_length {
        (
            VectorComparisonResult::DifferentLengths {
                expected_length,
                actual_length,
            },
            None,
            None,
        )
    } else {
        let mut any_inexact = false;
        let mut margin_factor = None;
        let mut multiplier_factor = None;

        for ix in 0..expected_length {
            let expected_element = &expected[ix];
            let actual_element = &actual[ix];

            let (scalar_comparison_result, scalar_margin_factor, scalar_multiplier_factor) =
                evaluate_scalar_eq_approx(expected_element, actual_element, evaluator);

            match scalar_comparison_result {
                ComparisonResult::ExactlyEqual => (),
                ComparisonResult::ApproximatelyEqual => {
                    if !any_inexact {
                        any_inexact = true;
                        margin_factor = scalar_margin_factor;
                        multiplier_factor = scalar_multiplier_factor;
                    }
                },
                ComparisonResult::Unequal => {
                    let (expected_value_of_first_unequal_element, actual_value_of_first_unequal_element) = {
                        let expected : &dyn traits::TestableAsF64 = &expected[ix];
                        let actual : &dyn traits::TestableAsF64 = &actual[ix];

                        let expected = expected.testable_as_f64();
                        let actual = actual.testable_as_f64();

                        (expected, actual)
                    };

                    return (
                        VectorComparisonResult::UnequalElements {
                            index_of_first_unequal_element : ix,
                            expected_value_of_first_unequal_element,
                            actual_value_of_first_unequal_element,
                        },
                        scalar_margin_factor,
                        scalar_multiplier_factor,
                    );
                },
            };
        }

        (
            if any_inexact {
                VectorComparisonResult::ApproximatelyEqual
            } else {
                VectorComparisonResult::ExactlyEqual
            },
            margin_factor,
            multiplier_factor,
        )
    }
}

/// Creates an [`ApproximateEqualityEvaluator`] that operates by applying
/// the given `factor` as a margin to determine approximate equality.
pub fn margin(factor : f64) -> impl traits::ApproximateEqualityEvaluator {
    internal::MarginEvaluator {
        factor,
    }
}

/// Creates an [`ApproximateEqualityEvaluator`] that operates by applying
/// the given `factor` as a multiplier to determine approximate equality.
pub fn multiplier(factor : f64) -> impl traits::ApproximateEqualityEvaluator {
    internal::MultiplierEvaluator {
        factor,
    }
}

/// Creates an [`ApproximateEqualityEvaluator`] that operates by applying
/// the given `multiplier_factor` as a multiplier to determine approximate
/// equality in all cases except when or both comparands is zero, in which
/// case it applies the `zero_margin_factor` as a margin to determine
/// approximate equality.
pub fn zero_margin_or_multiplier(
    multiplier_factor : f64,
    zero_margin_factor : f64,
) -> impl traits::ApproximateEqualityEvaluator {
    internal::ZeroMarginOrMultiplierEvaluator {
        multiplier_factor,
        zero_margin_factor,
    }
}


// /////////////////////////////////////////////////////////
// macros

#[macro_export]
macro_rules! assert_scalar_eq_approx {
    ($expected:expr, $actual:expr, $evaluator:expr) => {
        let expected_param = &$expected;
        let actual_param = &$actual;

        let (expected, actual) = {
            let expected : &dyn $crate::traits::TestableAsF64 = expected_param;
            let actual : &dyn $crate::traits::TestableAsF64 = actual_param;

            let expected = expected.testable_as_f64();
            let actual = actual.testable_as_f64();

            (expected, actual)
        };
        let evaluator : &dyn $crate::traits::ApproximateEqualityEvaluator = &$evaluator;

        // scope to protect against multiple `use`s of crate type(s)
        {
            use $crate::ComparisonResult as CR;

            let (comparison_result, margin_factor, multiplier_factor) = evaluator.evaluate(expected, actual);

            match comparison_result {
                CR::ExactlyEqual | CR::ApproximatelyEqual => (),
                CR::Unequal => {
                    match margin_factor {
                        Some(margin_factor) => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate equality: expected={expected_param:?}, actual={actual_param:?}, margin_factor={margin_factor}, multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate equality: expected={expected_param:?}, actual={actual_param:?}, margin_factor={margin_factor}",
                                    );
                                },
                            };
                        },
                        None => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate equality: expected={expected_param:?}, actual={actual_param:?}, multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    panic!("VIOLATION: This should not occur, and may only result from an improperly written implementor of `ApproximateEqualityEvaluator`");
                                }
                            };
                        },
                    };
                },
            };
        }
    };
    ($expected:expr, $actual:expr) => {
        let evaluator = $crate::zero_margin_or_multiplier($crate::constants::DEFAULT_MULTIPLIER, $crate::constants::DEFAULT_MARGIN);

        assert_scalar_eq_approx!($expected, $actual, evaluator);
    };
}

#[macro_export]
macro_rules! assert_scalar_ne_approx {
    ($expected:expr, $actual:expr, $evaluator:expr) => {
        let expected_param = &$expected;
        let actual_param = &$actual;

        let (expected, actual) = {
            let expected : &dyn $crate::traits::TestableAsF64 = expected_param;
            let actual : &dyn $crate::traits::TestableAsF64 = actual_param;

            let expected = expected.testable_as_f64();
            let actual = actual.testable_as_f64();

            (expected, actual)
        };
        let evaluator : &dyn $crate::traits::ApproximateEqualityEvaluator = &$evaluator;

        // scope to protect against multiple `use`s of crate type(s)
        {
            use $crate::ComparisonResult as CR;

            let (comparison_result, margin_factor, multiplier_factor) = evaluator.evaluate(expected, actual);

            match comparison_result {
                CR::Unequal => (),
                CR::ExactlyEqual | CR::ApproximatelyEqual => {
                    match margin_factor {
                        Some(margin_factor) => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate inequality: expected={expected_param:?}, actual={actual_param:?}, margin_factor={margin_factor}, multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate inequality: expected={expected_param:?}, actual={actual_param:?}, margin_factor={margin_factor}",
                                    );
                                },
                            };
                        },
                        None => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate inequality: expected={expected_param:?}, actual={actual_param:?}, multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    panic!("VIOLATION: This should not occur, and may only result from an improperly written implementor of `ApproximateEqualityEvaluator`");
                                }
                            };
                        }
                    };
                },
            };
        }
    };
    ($expected:expr, $actual:expr) => {
        let evaluator = $crate::zero_margin_or_multiplier($crate::constants::DEFAULT_MULTIPLIER, $crate::constants::DEFAULT_MARGIN);

        assert_scalar_ne_approx!($expected, $actual, evaluator);
    };
}

#[macro_export]
macro_rules! assert_vector_eq_approx {
    ($expected:expr, $actual:expr, $evaluator:expr) => {
        /*
        let expected_param = &$expected;
        let actual_param = &$actual;
         */
        let expected = &$expected;
        let actual = &$actual;
        let evaluator : &dyn $crate::traits::ApproximateEqualityEvaluator = &$evaluator;

        // scope to protect against multiple `use`s of crate type(s)
        {
            use $crate::VectorComparisonResult as CR;

            let (comparison_result, margin_factor, multiplier_factor) = $crate::evaluate_vector_eq_approx(&expected, &actual, evaluator);

            match comparison_result {
                CR::ExactlyEqual | CR::ApproximatelyEqual => (),
                CR::DifferentLengths {
                    expected_length,
                    actual_length,
                } => {
                    assert!(
                        false,
                        "assertion failed: failed to verify approximate equality for vectors: expected-length {expected_length} differs from actual-length {actual_length}",
                    );
                },
                CR::UnequalElements {
                    index_of_first_unequal_element,
                    expected_value_of_first_unequal_element,
                    actual_value_of_first_unequal_element,
                } => {
                    match margin_factor {
                        Some(margin_factor) => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate equality for vectors: at index {index_of_first_unequal_element} expected={expected_value_of_first_unequal_element:?}, actual={actual_value_of_first_unequal_element:?}, margin_factor={margin_factor}, multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate equality for vectors: at index {index_of_first_unequal_element} expected={expected_value_of_first_unequal_element:?}, actual={actual_value_of_first_unequal_element:?}, margin_factor={margin_factor}",
                                    );
                                },
                            };
                        },
                        None => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate equality for vectors: at index {index_of_first_unequal_element} expected={expected_value_of_first_unequal_element:?}, actual={actual_value_of_first_unequal_element:?}, multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    panic!("VIOLATION: This should not occur, and may only result from an improperly written implementor of `ApproximateEqualityEvaluator`");
                                }
                            };
                        },
                    };
                },
            };
        }
    };
    ($expected:expr, $actual:expr) => {
        let evaluator = $crate::zero_margin_or_multiplier($crate::constants::DEFAULT_MULTIPLIER, $crate::constants::DEFAULT_MARGIN);

        assert_vector_eq_approx!($expected, $actual, evaluator);
    };
}

#[macro_export]
macro_rules! assert_vector_ne_approx {
    ($expected:expr, $actual:expr, $evaluator:expr) => {
        /*
        let expected_param = &$expected;
        let actual_param = &$actual;
         */
        let expected = &$expected;
        let actual = &$actual;
        let evaluator : &dyn $crate::traits::ApproximateEqualityEvaluator = &$evaluator;

        // scope to protect against multiple `use`s of crate type(s)
        {
            use $crate::VectorComparisonResult as CR;

            let (comparison_result, margin_factor, multiplier_factor) = $crate::evaluate_vector_eq_approx(&expected, &actual, evaluator);

            match comparison_result {
                CR::DifferentLengths { ..} | CR::UnequalElements {..} => (),
                CR::ExactlyEqual | CR::ApproximatelyEqual => {
                    match margin_factor {
                        Some(margin_factor) => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate inequality for vectors; margin_factor={margin_factor},  multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate inequality for vectors; margin_factor={margin_factor}",
                                    );
                                },
                            };
                        },
                        None => {
                            match multiplier_factor {
                                Some(multiplier_factor) => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate inequality for vectors; multiplier_factor={multiplier_factor}",
                                    );
                                },
                                None => {
                                    assert!(
                                        false,
                                        "assertion failed: failed to verify approximate inequality for vectors",
                                    );
                                }
                            };
                        }
                    };
                },
            };
        }
    };
    ($expected:expr, $actual:expr) => {
        let evaluator =
            $crate::zero_margin_or_multiplier($crate::constants::DEFAULT_MULTIPLIER, $crate::constants::DEFAULT_MARGIN);

        assert_vector_ne_approx!($expected, $actual, evaluator);
    };
}


#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #![allow(non_snake_case)]

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
        fn TEST_margin_TEST_1() {
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
        fn TEST_margin_TEST_2() {
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
        fn TEST_multiplier_TEST_1() {
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
        fn TEST_multiplier_TEST_2() {
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


        #[test]
        fn TEST_assert_scalar_eq_approx_2_PARAMETER_FOR_EXACTLY_EQUAL_VALUES() {

            assert_scalar_eq_approx!(-1.23456789e-10, -1.23456789e-10);
            assert_scalar_eq_approx!(-0.123456789, -0.123456789);
            assert_scalar_eq_approx!(-0.1, -0.1);
            assert_scalar_eq_approx!(0.0, 0.0);
            assert_scalar_eq_approx!(0.1, 0.1);
            assert_scalar_eq_approx!(0.123456789, 0.123456789);
            assert_scalar_eq_approx!(1.23456789e+10, 1.23456789e+10);

            assert_scalar_eq_approx!(f64::INFINITY, f64::INFINITY);
            assert_scalar_eq_approx!(f64::NEG_INFINITY, f64::NEG_INFINITY);

            assert_scalar_eq_approx!(f64::MIN, f64::MIN);
            assert_scalar_eq_approx!(f64::MIN_POSITIVE, f64::MIN_POSITIVE);
            assert_scalar_eq_approx!(f64::MAX, f64::MAX);

            #[cfg(feature = "nan-equality")]
            {
                assert_scalar_eq_approx!(f64::NAN, f64::NAN);
            }
            #[cfg(not(feature = "nan-equality"))]
            {
                assert_scalar_ne_approx!(f64::NAN, f64::NAN);
            }

            {
                use std::f64::consts::*;

                assert_scalar_eq_approx!(PI, PI);
                assert_scalar_eq_approx!(TAU, TAU);
                assert_scalar_eq_approx!(PHI, PHI);
                assert_scalar_eq_approx!(EGAMMA, EGAMMA);
                assert_scalar_eq_approx!(FRAC_PI_2, FRAC_PI_2);
                assert_scalar_eq_approx!(FRAC_PI_3, FRAC_PI_3);
                assert_scalar_eq_approx!(FRAC_PI_4, FRAC_PI_4);
                assert_scalar_eq_approx!(FRAC_PI_6, FRAC_PI_6);
                assert_scalar_eq_approx!(FRAC_PI_8, FRAC_PI_8);
                assert_scalar_eq_approx!(FRAC_1_PI, FRAC_1_PI);
                assert_scalar_eq_approx!(FRAC_1_SQRT_PI, FRAC_1_SQRT_PI);
                assert_scalar_eq_approx!(FRAC_1_SQRT_2PI, FRAC_1_SQRT_2PI);
                assert_scalar_eq_approx!(FRAC_2_PI, FRAC_2_PI);
                assert_scalar_eq_approx!(FRAC_2_SQRT_PI, FRAC_2_SQRT_PI);
                assert_scalar_eq_approx!(SQRT_2, SQRT_2);
                assert_scalar_eq_approx!(FRAC_1_SQRT_2, FRAC_1_SQRT_2);
                assert_scalar_eq_approx!(SQRT_3, SQRT_3);
                assert_scalar_eq_approx!(FRAC_1_SQRT_3, FRAC_1_SQRT_3);
                assert_scalar_eq_approx!(E, E);
                assert_scalar_eq_approx!(LOG2_10, LOG2_10);
                assert_scalar_eq_approx!(LOG2_E, LOG2_E);
                assert_scalar_eq_approx!(LOG10_2, LOG10_2);
                assert_scalar_eq_approx!(LOG10_E, LOG10_E);
                assert_scalar_eq_approx!(LN_2, LN_2);
                assert_scalar_eq_approx!(LN_10, LN_10);
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

        /// Demonstrate that feature `"nan-equality"` only changes stock behaviour
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
        fn example_test_of_scalar_evaluation() {
            let expected = 3.0;
            let actual = 3.0001;
            assert_scalar_eq_approx!(expected, actual, margin(0.0001));
        }

        #[test]
        fn example_test_of_vector_evaluation() {
            let expected = &[ 3.0, -40404.0, 1.23456 ];
            let actual = Vec::from([ 3.0, -40410.0, 1.234567 ]);
            assert_vector_eq_approx!(expected, actual, multiplier(0.00015));
        }

    }
}


// ///////////////////////////// end of file //////////////////////////// //
