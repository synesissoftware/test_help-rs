// build.rs : test_help-rs

use bt_rs::rustc;


fn main() {
    println!("cargo:rustc-check-cfg=cfg(rustc_1_94_or_newer)");

    let version = rustc::compiler_version().expect("failed to execute `rustc --version`");

    if version >= (1, 94) {
        println!("cargo:rustc-cfg=rustc_1_94_or_newer");
    }
}


// ///////////////////////////// end of file //////////////////////////// //
