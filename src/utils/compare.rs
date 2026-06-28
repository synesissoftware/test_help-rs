// compare.rs : src/utils

use crate::ComparisonResult;


// Helper functions

/// Compares `expected` and `actual` using absolute `margin_factor`
/// tolerance.
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

/// Compares `expected` and `actual` using relative `multiplier_factor`
/// tolerance.
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

/// Compares `expected` and `actual` using `margin_factor` when either
/// operand is zero, otherwise `multiplier_factor`.
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
    #![cfg_attr(debug_assertions, allow(unused_imports))]


    use super::{
        compare_approximate_equality_by_margin,
        compare_approximate_equality_by_multiplier,
        compare_approximate_equality_by_zero_margin_or_multiplier,
    };

    use crate::ComparisonResult;


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


// ///////////////////////////// end of file //////////////////////////// //
