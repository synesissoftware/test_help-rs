# test_help-rs <!-- omit in toc -->

Test helpers for Rust

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Crates.io](https://img.shields.io/crates/v/test_help-rs.svg)](https://crates.io/crates/test_help-rs)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/test_help-rs.svg)](https://github.com/synesissoftware/test_help-rs/releases/latest)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![CI](https://github.com/synesissoftware/test_help-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/test_help-rs/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/test_help-rs/badge.svg)](https://docs.rs/test_help-rs)


## Introduction

Rust has powerful and easy-to-use unit-testing mechanisms, but there are some missing elements, particularly around the use of floating-point values - `f32` and `f64` - that are provided by this crate for asserting approximate equality, as in:

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
	- [Features](#features)
	- [Functions](#functions)
	- [Macros](#macros)
	- [Structures](#structures)
	- [Traits](#traits)
- [Examples](#examples)
- [Project Information](#project-information)
	- [Where to get help](#where-to-get-help)
	- [Contribution guidelines](#contribution-guidelines)
	- [Dependencies](#dependencies)
		- [Efferent (fan-out)](#efferent-fan-out)
		- [Build Dependencies](#build-dependencies)
		- [Development Dependencies](#development-dependencies)
		- [Afferent (fan-in)](#afferent-fan-in)
	- [Related projects](#related-projects)
	- [License](#license)


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
test_help-rs = { version = "0.1" }
```


## Components

### Constants

The following constants are defined in [`constants`](https://docs.rs/test_help-rs/latest/test_helpers/constants/index.html):

* `DEFAULT_MARGIN` — default absolute margin (`0.0001`, i.e. 1e-4) used by the stock two-argument assertion macros when no custom evaluator is supplied;
* `DEFAULT_MULTIPLIER` — default relative multiplier (`0.000001`, i.e. 1e-6) used together with `DEFAULT_MARGIN` by the stock [`zero_margin_or_multiplier()`](https://docs.rs/test_help-rs/latest/test_helpers/fn.zero_margin_or_multiplier.html) evaluator path;


### Enumerations

The following enumerations are defined:

* [`ComparisonResult`](https://docs.rs/test_help-rs/latest/test_helpers/enum.ComparisonResult.html) — outcome of comparing two scalar `f64` values:
	* `ExactlyEqual` — the values are identical (including matching infinities and, when the `"nan-equality"` feature is enabled, both `NaN`);
	* `ApproximatelyEqual` — the values differ but are within the margin or multiplier tolerance of the evaluator;
	* `Unequal` — the values are not equal under the evaluator;
* [`VectorComparisonResult`](https://docs.rs/test_help-rs/latest/test_helpers/enum.VectorComparisonResult.html) — outcome of comparing two vectors of `f64` values:
	* `ExactlyEqual` — same length and every element pair is exactly equal;
	* `ApproximatelyEqual` — same length and every element pair is equal within the evaluator tolerance;
	* `DifferentLengths { expected_length, actual_length }` — the vectors have different lengths;
	* `UnequalElements { index_of_first_unequal_element, expected_value_of_first_unequal_element, actual_value_of_first_unequal_element }` — same length but at least one element pair is unequal under the evaluator;


### Features

The following optional features are defined in **Cargo.toml**:

* **Crate-specific features**:

	* `nan-equality` — allows two `f64::NAN` values to be treated as equal for stock comparisons (does not affect custom [`ApproximateEqualityEvaluator`](https://docs.rs/test_help-rs/latest/test_helpers/traits/trait.ApproximateEqualityEvaluator.html) implementations);
	* `nightly-constants` — enables unit tests for additional `std::f64` constants that require the unstable `more_float_constants` feature; build and test with a nightly toolchain via `./scripts/test-nightly-constants` (this feature is for crate development only and is not required by downstream consumers);

* **General features**:

	* `null-feature` — a feature that has no effect (and, thus, is useful for simplifying driver scripts);


### Functions

The following functions are defined:

* `margin() -> impl ApproximateEqualityEvaluator` - creates an implementation of the `ApproximateEqualityEvaluator` trait that defines a margin-based evaluator instance;
* `multiplier() -> impl ApproximateEqualityEvaluator` - creates an implementation of the `ApproximateEqualityEvaluator` trait that defines a multiplier-based evaluator instance;
* `zero_margin_or_multiplier() -> impl ApproximateEqualityEvaluator` - creates an implementation of the `ApproximateEqualityEvaluator` trait that defines both a margin to be used when expected value and/or actual value is zero, and a multiplier to be used in all other cases;
* `evaluate_scalar_eq_approx()` - a generic function that may be used to compare expected and actual scalar values of types that are logically `f64`, along with an evaluator (of type `&dyn ApproximateEqualityEvaluator`). This function is used in the crate macros, but may also be used as part of the implementation of such macros for testing application-defined types;
* `evaluate_vector_eq_approx()` - a generic function that may be used to compare expected and actual values that are vectors of types that are logically `f64`, along with an evaluator (of type `&dyn ApproximateEqualityEvaluator`). This function is used in the crate macros, but may also be used as part of the implementation of such macros for testing application-defined types;


### Macros

The following macros are defined (re-exported at the crate root via [`test_helpers`](https://docs.rs/test_help-rs/latest/test_helpers/index.html)):

* `assert_scalar_eq_approx!(expected, actual)` — asserts approximate equality using the stock [`zero_margin_or_multiplier()`](https://docs.rs/test_help-rs/latest/test_helpers/fn.zero_margin_or_multiplier.html) evaluator (`DEFAULT_MULTIPLIER` / `DEFAULT_MARGIN`);
* `assert_scalar_eq_approx!(expected, actual, evaluator)` — asserts approximate equality using a custom [`ApproximateEqualityEvaluator`](https://docs.rs/test_help-rs/latest/test_helpers/traits/trait.ApproximateEqualityEvaluator.html);
* `assert_scalar_ne_approx!(expected, actual)` — asserts approximate inequality using the stock evaluator;
* `assert_scalar_ne_approx!(expected, actual, evaluator)` — asserts approximate inequality using a custom evaluator;
* `assert_vector_eq_approx!(expected, actual)` — asserts approximate equality of two vectors (slices, arrays, or `Vec`) using the stock evaluator;
* `assert_vector_eq_approx!(expected, actual, evaluator)` — asserts approximate vector equality using a custom evaluator;
* `assert_vector_ne_approx!(expected, actual)` — asserts approximate vector inequality using the stock evaluator;
* `assert_vector_ne_approx!(expected, actual, evaluator)` — asserts approximate vector inequality using a custom evaluator;


### Structures

No public structures are defined at this time.


### Traits

The following traits are defined:

* `ApproximateEqualityEvaluator` - prescribes the (non-mutating) instance method `#evaluate()`, allowing custom evaluators to be defined for use with the assertion macros;
* `TestableAsF64` - prescribes the (non-mutating) instance method `#testable_as_f64() : f64`, and provides implementation for any type that implements the `ToF64` trait defined in the [**base-traits**](https://github.com/synesissoftware/base-traits) crate;


## Examples

Example programs are provided in the **examples** directory:

* [**examples/scalars.rs**](./examples/scalars.rs) — scalar approximate equality and inequality:

```sh
cargo run --example scalars
```

* [**examples/vectors.rs**](./examples/vectors.rs) — vector approximate equality and inequality (slice and `Vec`):

```sh
cargo run --example vectors
```

**scalars** exercises `assert_scalar_eq_approx!()` and `assert_scalar_ne_approx!()` with margin- and multiplier-based evaluators. **vectors** includes the README introduction example and similar pass/fail demonstrations for vector assertions.


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/test_help-rs "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/test_help-rs.


### Dependencies


#### Efferent (fan-out)

Libraries upon which **test_help-rs** depends:

* [**base-traits**](https://github.com/synesissoftware/base-traits) — [`ToF64`](https://docs.rs/base-traits/latest/base_traits/trait.ToF64.html) trait used by [`TestableAsF64`](https://docs.rs/test_help-rs/latest/test_helpers/traits/trait.TestableAsF64.html);


#### Build Dependencies

Libraries used only when building **test_help-rs** (not required by downstream consumers):

* [**bt-rs**](https://github.com/synesissoftware/bt-rs) — [`rustc::compiler_version()`](https://docs.rs/bt-rs/latest/bt_rs/rustc/fn.compiler_version.html) in **build.rs** to detect Rust 1.94+ and set the `rustc_1_94_or_newer` cfg for unit tests of stable `std::f64::consts` added in that release;


#### Development Dependencies

None currently.


#### Afferent (fan-in)

Projects that depend on **test_help-rs** (typically as a **dev-dependency** for unit tests):

* [**auto_buffer.Rust**](https://github.com/synesissoftware/auto_buffer.Rust);
* [**collect-rs**](https://github.com/synesissoftware/collect-rs);
* [**libpath.Rust**](https://github.com/synesissoftware/libpath.Rust);
* [**p99.Rust**](https://github.com/synesissoftware/p99.Rust);
* [**shwild.Rust**](https://github.com/synesissoftware/shwild.Rust);


### Related projects

* [**shwild.Rust**](https://github.com/synesissoftware/shwild.Rust) defines functionality for matching strings against **SH**ell-compatible **WILD**card patterns, including matching assertion macros that are useful in unit-testing;


### License

**test_help-rs** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->

