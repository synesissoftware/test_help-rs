# test_help-rs Changes <!-- omit in toc -->


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
