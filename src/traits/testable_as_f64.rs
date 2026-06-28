// testable_as_f64.rs : src/traits

use base_traits::ToF64;

use std::fmt as std_fmt;


/// Trait that allows an implementing type instance to be
/// evaluated with the constructs of this crate.
///
/// NOTE: it is implemented for any types that implement
/// `base_traits::ToF64` (and [`std_fmt::Debug`]).
pub trait TestableAsF64: std_fmt::Debug {
    fn testable_as_f64(&self) -> f64;
}


// Trait implementations

impl<T> TestableAsF64 for T
where
    T : ToF64 + std_fmt::Debug,
{
    fn testable_as_f64(&self) -> f64 {
        self.to_f64()
    }
}


// ///////////////////////////// end of file //////////////////////////// //
