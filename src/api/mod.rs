// mod.rs : src/api

mod evaluate_scalar;
mod evaluate_vector;
mod margin;
mod multiplier;
mod zero_margin_or_multiplier;

pub use evaluate_scalar::evaluate_scalar_eq_approx;
pub use evaluate_vector::evaluate_vector_eq_approx;
pub use margin::margin;
pub use multiplier::multiplier;
pub use zero_margin_or_multiplier::zero_margin_or_multiplier;


// ///////////////////////////// end of file //////////////////////////// //
