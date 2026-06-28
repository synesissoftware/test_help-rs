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
* aligned **.vscode/settings.json** and **rustfmt.toml** with **Diagnosticism.Rust**;
* added **scripts/fmt**, **scripts/check_doc_76.py**, and **scripts/check_test_names.py**;


## 0.1.0 - 17th September 2024

* first release;


<!-- ########################### end of file ########################### -->
