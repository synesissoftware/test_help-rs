// vector_comparison_result.rs : test_help-rs

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


// ///////////////////////////// end of file //////////////////////////// //
