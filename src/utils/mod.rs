// mod.rs : src/utils

mod compare;


pub(crate) use compare::{
    compare_approximate_equality_by_margin,
    compare_approximate_equality_by_multiplier,
    compare_approximate_equality_by_zero_margin_or_multiplier,
};


// ///////////////////////////// end of file //////////////////////////// //
