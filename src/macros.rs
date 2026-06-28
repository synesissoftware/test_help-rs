// macros.rs : test_help-rs

// Macros

/// Asserts that two scalar values are approximately equal.
///
/// # Forms
///
/// * `assert_scalar_eq_approx!(expected, actual)` — stock
///   [`zero_margin_or_multiplier`] evaluator
///   ([`constants::DEFAULT_MULTIPLIER`] /
///   [`constants::DEFAULT_MARGIN`]);
/// * `assert_scalar_eq_approx!(expected, actual, evaluator)` — custom
///   [`traits::ApproximateEqualityEvaluator`];
///
/// Operands must implement [`traits::TestableAsF64`].
///
/// # Examples
///
/// ```
/// use test_helpers::{assert_scalar_eq_approx, margin};
///
/// assert_scalar_eq_approx!(3.0, 3.0001, margin(0.0001));
/// ```
///
/// # Panics
///
/// Panics when the evaluator reports [`ComparisonResult::Unequal`].
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

/// Asserts that two scalar values are not approximately equal.
///
/// # Forms
///
/// * `assert_scalar_ne_approx!(expected, actual)` — stock
///   [`zero_margin_or_multiplier`] evaluator;
/// * `assert_scalar_ne_approx!(expected, actual, evaluator)` — custom
///   [`traits::ApproximateEqualityEvaluator`];
///
/// Operands must implement [`traits::TestableAsF64`].
///
/// # Panics
///
/// Panics when the evaluator reports [`ComparisonResult::ExactlyEqual`]
/// or [`ComparisonResult::ApproximatelyEqual`].
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

/// Asserts that two vectors are approximately equal element-wise.
///
/// # Forms
///
/// * `assert_vector_eq_approx!(expected, actual)` — stock
///   [`zero_margin_or_multiplier`] evaluator;
/// * `assert_vector_eq_approx!(expected, actual, evaluator)` — custom
///   [`traits::ApproximateEqualityEvaluator`];
///
/// `expected` and `actual` may be slices, arrays, or [`Vec`]; element
/// types must implement [`traits::TestableAsF64`].
///
/// # Examples
///
/// ```
/// use test_helpers::{assert_vector_eq_approx, multiplier};
///
/// let expected = &[3.0, -40404.0, 1.23456];
/// let actual = vec![3.0, -40410.0, 1.234567];
/// assert_vector_eq_approx!(expected, actual, multiplier(0.00015));
/// ```
///
/// # Panics
///
/// Panics when lengths differ, when any element pair is unequal under the
/// evaluator, or when [`VectorComparisonResult::UnequalElements`] applies.
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

/// Asserts that two vectors are not approximately equal.
///
/// # Forms
///
/// * `assert_vector_ne_approx!(expected, actual)` — stock
///   [`zero_margin_or_multiplier`] evaluator;
/// * `assert_vector_ne_approx!(expected, actual, evaluator)` — custom
///   [`traits::ApproximateEqualityEvaluator`];
///
/// `expected` and `actual` may be slices, arrays, or [`Vec`]; element
/// types must implement [`traits::TestableAsF64`].
///
/// # Panics
///
/// Panics when lengths match and every element pair is equal under the
/// evaluator ([`VectorComparisonResult::ExactlyEqual`] or
/// [`VectorComparisonResult::ApproximatelyEqual`]).
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

// ///////////////////////////// end of file //////////////////////////// //
