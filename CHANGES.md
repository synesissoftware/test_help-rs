# test_help-rs - Changes <!-- omit in toc -->


## 0.2.2 - 11th September 2026

* modernised repository metadata, editor configuration, ignore rules, and Rust formatting configuration;
* refined stable and nightly CI checks for features, documentation, examples, packaging, and the MSRV;
* added the **versions** example and excluded development-only files from published packages;
* documented the MSRV and added the `_NEVER_TO_BE_ENABLED` placeholder feature;
* canonicalised **Cargo.toml** metadata, feature declarations, and dependency specifications;
* improved the test-name checker to handle numeric and padded construct names;
* refreshed the formatting and nightly feature-test drivers for the pinned nightly toolchain;


## 0.2.1 - 31st August 2026

* strengthened **.github/workflows/ci.yml** with locked stable, nightly, and MSRV checks, documentation, examples, checkers, and package validation;
* pinned the nightly formatter and feature-test toolchains in **scripts/fmt**, **scripts/test-nightly-constants**, and **rustfmt.toml**;
* added **scripts/check_derives.py** and expanded public rustdoc;
* improved **Cargo.toml**, **README.md**, **NEWS.md**, **EXAMPLES.md**, and **TODO.md** release documentation;
* removed the empty **.cargo/config.toml**;


## 0.2.0 - 20th July 2026

* added `assert_as_str_eq!()` and `assert_as_str_ne!()` macros that compare instances of string-like type via `as_str()` method — either inherently or via `crate::base_traits::AsStr`;
* added feature `"full"`;


## 0.1.3 - 17th July 2026

* updated internal utility macro `declare_and_publish!()` to match that of **base-traits**;
* preparatory changes;
* applied improved formatting;


## 0.1.2 - 14th July 2026

* updated dependencies;
* various further boilerplate improvements;


## 0.1.1 - 28th June 2026

* added **CHANGES.md**;
* added **TODO.md**;
* added **.gitattributes**;
* renamed **.rustfmt.toml** => **rustfmt.toml** (and aligned `edition` with **Cargo.toml**);
* **Cargo.toml** metadata (**repository**, **documentation**, **categories**, **keywords**, **rust-version**);
* **README.md** badges, installation reference, and **examples/scalars.rs** target documentation;
* added optional **`"nightly-constants"`** feature for extended `std::f64` constant tests on nightly (**./scripts/test-nightly-constants**);
* stable **cargo test** no longer requires the `"more_float_constants"` feature;
* added **.github/workflows/ci.yml** (test, clippy, fmt, MSRV, `"nan-equality"`, nightly `"nightly-constants"`, DOC_76 and RUST_TEST_NAMING checkers);
* aligned **README.md** badge order and layout with **Diagnosticism.Rust**;
* completed **README.md** documentation (constants, enumerations, features, macros, efferent/afferent dependencies);
* added **examples/vectors.rs** example program;
* extended stable **`f64` / `consts`** exact-equality tests for **`GOLDEN_RATIO`** and **`EULER_GAMMA`** (Rust 1.94+, via **build.rs** `rustc_1_94_or_newer` cfg);
* split exact-equality scalar tests into nested module with per-group test functions;
* added **`[build-dependencies]`** **bt-rs** and **build.rs** (`rustc_1_94_or_newer` cfg);
* **README.md** Build Dependencies section documenting **bt-rs**;
* aligned **.vscode/settings.json** and **rustfmt.toml** with **Diagnosticism.Rust**;
* added **scripts/fmt**, **scripts/check_doc_76.py**, and **scripts/check_test_names.py**;
* refactored monolithic **src/lib.rs** into modular **src/** layout (`traits/`, `api/`, `utils/`, `internal/`, **macros.rs**, **tests/**);
* added crate-level **`//!`** rustdoc in **lib.rs**;
* replaced **T.B.C.** documentation placeholders in internal modules;
* added **`///`** rustdoc for all four assertion macros in **macros.rs** (Forms, Examples, Panics);
* added **`///`** rustdoc for **`evaluate_scalar_eq_approx`** and **`evaluate_vector_eq_approx`** in **src/api/** (Returns, Examples);
* aligned **src/** with SIS **STD_ALIASING** and **RUST_FILE_LAYOUT** conventions;


## 0.1.0 - 17th September 2024

* first release;


<!-- ########################### end of file ########################### -->
