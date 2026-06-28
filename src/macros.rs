// macros.rs : test_help-rs

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

// ///////////////////////////// end of file //////////////////////////// //
