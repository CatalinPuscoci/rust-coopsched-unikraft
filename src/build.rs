extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //
    println!("cargo:rerun-if-changed=wrapper.h");
        
    println!("current dir:{}",env::current_dir().unwrap().display());

    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET defined");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST defined");

    //include folder
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::builder()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        .clang_arg("-I../../libs/lib-newlib/musl-imported/include")
        .clang_arg("-I../../unikraft/lib/ukdebug/include")
        .clang_arg("-I../../unikraft/include")
        .clang_arg("../../unikraft/lib/nolibc/include/")
        .clang_arg("-I../../unikraft/arch/x86/x86_64/include")
        .clang_arg("-I../../unikraft/include/uk")
        .clang_arg("-I../../unikraft/plat/kvm/include")
        .clang_arg("-I../../unikraft/include/uk/plat")
        .clang_arg("-I./build/include")
        .clang_arg("-I../../unikraft/lib/ukalloc/include")
        .clang_arg("-I../../unikraft/lib/uktime/include")
        .clang_arg("-I../../unikraft/lib/uksched/include")
        .clang_arg("-I../../unikraft/lib/uksignal/include")
        .clang_arg("-I../../unikraft/lib/ukschedcoop/include")
        .clang_arg("-target").clang_arg("x86_64-unknown-linux-gnu");
        //.clang_arg("-I../../../unikraft/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        bindings.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
}
