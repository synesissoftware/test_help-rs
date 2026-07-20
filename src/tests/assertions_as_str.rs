// assertions.rs : src/tests

#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(unused_imports))]


mod TEST_str_WITH_str {
    #![allow(non_snake_case)]

    use super::*;


    #[test]
    fn TEST_SUCCEEDING() {
        // assert_as_str_eq
        {
            let cases = &[
                ("", ""),
                ("abc", "abc"),
                ("abcdefghijklm12345_NOPQRSTUVWXYZ", "abcdefghijklm12345_NOPQRSTUVWXYZ"),
            ];

            cases.iter().for_each(|(expected, actual)| {
                assert_as_str_eq!(expected, actual);
            });
        }

        // assert_as_str_ne
        {
            let cases = &[
                ("", "a"),
                ("abc", "ABC"),
                ("ABCDEFGHIJKLM12345_nopqrstuvwxyz", "abcdefghijklm12345_NOPQRSTUVWXYZ"),
            ];

            cases.iter().for_each(|(expected, actual)| {
                assert_as_str_ne!(expected, actual);
            });
        }

        {
            assert_as_str_ne!("", "a");
            assert_as_str_ne!("abc", "ABC");
            assert_as_str_ne!("ABCDEFGHIJKLM12345_nopqrstuvwxyz", "abcdefghijklm12345_NOPQRSTUVWXYZ");
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: \"abc\"\n right: \"ABC\"")]
    fn TEST_str_EQ_str_FAILURE_1() {
        assert_as_str_eq!("abc", "ABC");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: failed to establish 'abc' equal to 'ABC'")]
    fn TEST_str_EQ_str_FAILURE_2() {
        assert_as_str_eq!("abc", "ABC", "failed to establish '{}' equal to '{}'", "abc", "ABC");
    }

    #[test]
    #[should_panic(expected = "assertion `left != right` failed\n  left: \"abc\"\n right: \"abc\"")]
    fn TEST_str_NE_str_FAILURE_1() {
        assert_as_str_ne!("abc", "abc");
    }

    #[test]
    #[should_panic(expected = "assertion `left != right` failed: failed to establish 'ABC' not equal to 'ABC'")]
    fn TEST_str_NE_str_FAILURE_2() {
        assert_as_str_ne!("ABC", "ABC", "failed to establish '{}' not equal to '{}'", "ABC", "ABC");
    }
}


mod TEST_str_WITH_String {
    #![allow(non_snake_case)]

    use super::*;


    #[test]
    fn TEST_SUCCEEDING() {
        // assert_as_str_eq
        {
            let cases = &[
                ("", ""),
                ("abc", "abc"),
                ("abcdefghijklm12345_NOPQRSTUVWXYZ", "abcdefghijklm12345_NOPQRSTUVWXYZ"),
            ];

            cases.iter().for_each(|(expected, actual)| {
                assert_as_str_eq!(expected, String::from(*actual));
            });
        }

        // assert_as_str_ne
        {
            let cases = &[
                ("", "a"),
                ("abc", "ABC"),
                ("ABCDEFGHIJKLM12345_nopqrstuvwxyz", "abcdefghijklm12345_NOPQRSTUVWXYZ"),
            ];

            cases.iter().for_each(|(expected, actual)| {
                assert_as_str_ne!(expected, String::from(*actual));
            });
        }

        {
            assert_as_str_ne!("", String::from("a"));
            assert_as_str_ne!("abc", String::from("ABC"));
            assert_as_str_ne!(
                "ABCDEFGHIJKLM12345_nopqrstuvwxyz",
                String::from("abcdefghijklm12345_NOPQRSTUVWXYZ")
            );
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: \"abc\"\n right: \"ABC\"")]
    fn TEST_str_EQ_str_FAILURE_1() {
        assert_as_str_eq!("abc", String::from("ABC"));
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: failed to establish 'abc' equal to 'ABC'")]
    fn TEST_str_EQ_str_FAILURE_2() {
        assert_as_str_eq!(
            "abc",
            String::from("ABC"),
            "failed to establish '{}' equal to '{}'",
            "abc",
            "ABC"
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left != right` failed\n  left: \"abc\"\n right: \"abc\"")]
    fn TEST_str_NE_str_FAILURE_1() {
        assert_as_str_ne!("abc", String::from("abc"));
    }

    #[test]
    #[should_panic(expected = "assertion `left != right` failed: failed to establish 'ABC' not equal to 'ABC'")]
    fn TEST_str_NE_str_FAILURE_2() {
        assert_as_str_ne!(
            "ABC",
            String::from("ABC"),
            "failed to establish '{}' not equal to '{}'",
            "ABC",
            "ABC"
        );
    }
}


mod TEST_String_WITH_str {
    #![allow(non_snake_case)]

    use super::*;


    #[test]
    fn TEST_SUCCEEDING() {
        // assert_as_str_eq
        {
            let cases = &[
                ("", ""),
                ("abc", "abc"),
                ("abcdefghijklm12345_NOPQRSTUVWXYZ", "abcdefghijklm12345_NOPQRSTUVWXYZ"),
            ];

            cases.iter().for_each(|(expected, actual)| {
                assert_as_str_eq!(String::from(*expected), actual);
            });
        }

        // assert_as_str_ne
        {
            let cases = &[
                ("", "a"),
                ("abc", "ABC"),
                ("ABCDEFGHIJKLM12345_nopqrstuvwxyz", "abcdefghijklm12345_NOPQRSTUVWXYZ"),
            ];

            cases.iter().for_each(|(expected, actual)| {
                assert_as_str_ne!(String::from(*expected), actual);
            });
        }

        {
            assert_as_str_ne!(String::from(""), "a");
            assert_as_str_ne!(String::from("abc"), "ABC");
            assert_as_str_ne!(
                String::from("ABCDEFGHIJKLM12345_nopqrstuvwxyz"),
                "abcdefghijklm12345_NOPQRSTUVWXYZ"
            );
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: \"abc\"\n right: \"ABC\"")]
    fn TEST_str_EQ_str_FAILURE_1() {
        assert_as_str_eq!(String::from("abc"), "ABC");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: failed to establish 'abc' equal to 'ABC'")]
    fn TEST_str_EQ_str_FAILURE_2() {
        assert_as_str_eq!(
            String::from("abc"),
            "ABC",
            "failed to establish '{}' equal to '{}'",
            "abc",
            "ABC"
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left != right` failed\n  left: \"abc\"\n right: \"abc\"")]
    fn TEST_str_NE_str_FAILURE_1() {
        assert_as_str_ne!(String::from("abc"), "abc");
    }

    #[test]
    #[should_panic(expected = "assertion `left != right` failed: failed to establish 'ABC' not equal to 'ABC'")]
    fn TEST_str_NE_str_FAILURE_2() {
        assert_as_str_ne!(
            String::from("ABC"),
            "ABC",
            "failed to establish '{}' not equal to '{}'",
            "ABC",
            "ABC"
        );
    }
}


// ///////////////////////////// end of file //////////////////////////// //
