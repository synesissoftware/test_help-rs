// assertions_as_str.rs : test_help-rs

// As-str assertion macros

/// Asserts that two values are equal when compared as strings ([`&str`]).
///
/// # Forms
///
/// * `assert_as_str_eq!(expected, actual)` — compares via
///   [`crate::base_traits::AsStr::as_str`];
/// * `assert_as_str_eq!(expected, actual, …)` — same, with a custom
///   panic message (as for [`assert_eq!`]);
///
/// Operands must provide an `#as_str()` method — either inherently or via
/// [`crate::base_traits::AsStr`].
///
/// # Examples
///
/// ```
/// use test_helpers::assert_as_str_eq;
///
/// assert_as_str_eq!("abc", String::from("abc"));
/// ```
///
/// # Panics
///
/// Panics when the string views are unequal (via [`assert_eq!`]).
#[macro_export]
macro_rules! assert_as_str_eq {
    ($expected:expr, $actual:expr) => {
        let expected_arg = &$expected;
        let actual_arg = &$actual;

        let expected = {
            use $crate::base_traits::AsStr as _;

            expected_arg.as_str()
        };
        let actual = {
            use $crate::base_traits::AsStr as _;

            actual_arg.as_str()
        };

        assert_eq!(expected, actual);
    };

    ($expected:expr, $actual:expr, $($arg:tt)+) => {
        let expected_arg = &$expected;
        let actual_arg = &$actual;

        let expected = {
            use $crate::base_traits::AsStr as _;

            expected_arg.as_str()
        };
        let actual = {
            use $crate::base_traits::AsStr as _;

            actual_arg.as_str()
        };

        assert_eq!(expected, actual, $($arg)+);
    };
}

/// Asserts that two values are not equal when compared as strings.
///
/// # Forms
///
/// * `assert_as_str_ne!(expected, actual)` — compares via
///   [`crate::base_traits::AsStr::as_str`];
/// * `assert_as_str_ne!(expected, actual, …)` — same, with a custom
///   panic message (as for [`assert_ne!`]);
///
/// Operands must provide an `#as_str()` method — either inherently or via
/// [`crate::base_traits::AsStr`].
///
/// # Examples
///
/// ```
/// use test_helpers::assert_as_str_ne;
///
/// assert_as_str_ne!("abc", String::from("ABC"));
/// ```
///
/// # Panics
///
/// Panics when the string views are equal (via [`assert_ne!`]).
#[macro_export]
macro_rules! assert_as_str_ne {
    ($expected:expr, $actual:expr) => {
        let expected_arg = &$expected;
        let actual_arg = &$actual;

        let expected = {
            use $crate::base_traits::AsStr as _;

            expected_arg.as_str()
        };
        let actual = {
            use $crate::base_traits::AsStr as _;

            actual_arg.as_str()
        };

        assert_ne!(expected, actual);
    };

    ($expected:expr, $actual:expr, $($arg:tt)+) => {
        let expected_arg = &$expected;
        let actual_arg = &$actual;

        let expected = {
            use $crate::base_traits::AsStr as _;

            expected_arg.as_str()
        };
        let actual = {
            use $crate::base_traits::AsStr as _;

            actual_arg.as_str()
        };

        assert_ne!(expected, actual, $($arg)+);
    };
}


// ///////////////////////////// end of file //////////////////////////// //
