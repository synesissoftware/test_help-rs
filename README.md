# test_help-rs <!-- omit in toc -->

Test helpers for Rust


## Introduction

Rust has powerful and easy-to-use unit-testing mechanisms, but there are some missing elements, particular around the use of floating-point values - `f32` and `f64` - that are provided by this crate to allowing for assertion of approximate equality, as in:

```Rust
use test_helpers::{
	assert_scalar_eq_approx,
	assert_vector_eq_approx,
	margin,
	multiplier,
};

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
```


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Components](#components)
	- [Constants](#constants)
	- [Enumerations](#enumerations)
	- [Functions](#functions)
	- [Macros](#macros)
	- [Structures](#structures)
	- [Traits](#traits)
- [Examples](#examples)
- [Project Information](#project-information)
	- [Where to get help](#where-to-get-help)
	- [Contribution guidelines](#contribution-guidelines)
	- [Dependencies](#dependencies)
			- [Dev Dependencies](#dev-dependencies)
	- [Related projects](#related-projects)
	- [License](#license)


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
test_help-rs = { version = "~0.1" }
```


## Components

### Constants

The following constants are defined:

* `DEFAULT_MARGIN` - specifies the default comparison margin value, which is a xxxx;
* `DEFAULT_MULTIPLIER` - specifies the default comparison multiplier value, which is a xxxx;


### Enumerations

The following enuemrations are defined:

* `ComparisonResult` - ... TBC;
* `VectorComparisonResult` - ... TBC;


### Functions

The following functions are defined:

* `margin() -> impl ApproximateEqualityEvaluator` - creates an implementation of the `ApproximateEqualityEvaluator` trait that defines a margin-based evaluator instance;
* `multiplier() -> impl ApproximateEqualityEvaluator` - creates an implementation of the `ApproximateEqualityEvaluator` trait that defines a multiplier-based evaluator instance;
* `zero_margin_or_multiplier() -> impl ApproximateEqualityEvaluator` - creates an implementation of the `ApproximateEqualityEvaluator` trait that defines both a margin to be used when expected value and/or actual value is zero, and a multiplier to be used in all other cases;
* `evaluate_scalar_eq_approx()` - a generic function that may be used to compare expected and actual scalar values of types that are logically `f64`, along with an evaluator (of type `&dyn ApproximateEqualityEvaluator`). This function is used in the crate macros, but may also be used as part of the implementation of such macros for testing application-defined types;
* `evaluate_vector_eq_approx()` - a generic function that may be used to compare expected and actual values that are vectors of types that are logically `f64`, along with an evaluator (of type `&dyn ApproximateEqualityEvaluator`). This function is used in the crate macros, but may also be used as part of the implementation of such macros for testing application-defined types;


### Macros

The following macros are defined:

* `assert_scalar_eq_approx!()` - asserts approximate equality of expected and actual values, with an optional evaluator;
* `assert_scalar_ne_approx!()` - asserts approximate inequality of expected and actual values, with an optional evaluator;
* `assert_vector_eq_approx!()` - asserts approximate equality of expected and actual vectors of values, with an optional evaluator;
* `assert_vector_ne_approx!()` - asserts approximate inequality of expected and actual vectors of values, with an optional evaluator;


### Structures

No public structures are defined at this time.


### Traits

The following traits are defined:

* `ApproximateEqualityEvaluator` - prescribes the (non-mutating) instance method `#evaluate()`, allowing custom evaluators to be defined for use with the assertion macros;
* `TestableAsF64` - prescribes the (non-mutating) instance method `#testable_as_f64() : f64`, and provides implementation for any type that implements the `ToF64` trait defined in the [**base-traits**](https://github.com/synesissoftware/base-traits) crate;


## Examples

T.B.C.


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/test_help-rs "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/test_help-rs.


### Dependencies


Crates upon which **test_help-rs** depend:

* [**base-traits**](https://github.com/synesissoftware/base-traits);


##### Dev Dependencies

None currently.


### Related projects

* [**shwild.Rust**](https://github.com/synesissoftware/shwild.Rust) defines functionality for matching strings against **SH**ell-compatible **WILD**card patterns, including matching assertion macros that are useful in unit-testing;


### License

**test_help-rs** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->

