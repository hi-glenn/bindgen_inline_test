use bindgen::Builder;
use std::path::PathBuf;
use std::{env, path};

fn main() {
    // Tell cargo to statically link against the `libextern` static library.
    println!("cargo:rustc-link-lib=static=extern");
    println!("cargo:rerun-if-changed=input.h");

    let input = "input.h";

    let proj_root_path = path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let headers_dir = proj_root_path.join("");

    let output_path = proj_root_path.join("src"); // PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // This is the path to the object file.
    let obj_path = output_path.join("extern.o");
    // This is the path to the static library file.
    let lib_path = output_path.join("libextern.a");

    // Tell bindgen to generate wrappers for static functions
    let bindings = Builder::default()
        .header(input)
        .clang_arg(format!("-L{}", lib_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .generate()
        .unwrap();

    // Compile the generated wrappers into an object file.
    let clang_output = std::process::Command::new("clang")
        .arg("-flto=thin")
        .arg("-O")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(std::env::temp_dir().join("bindgen").join("extern.c"))
        .arg("-I")
        .arg(format!("{}", headers_dir.display())) // input
        .output()
        .unwrap();

    if !clang_output.status.success() {
        panic!(
            "Could not compile object file:\n{}",
            String::from_utf8_lossy(&clang_output.stderr)
        );
    }

    // Turn the object file into a static library
    #[cfg(not(target_os = "windows"))]
    let lib_output = std::process::Command::new("ar")
        .arg("crus")
        .arg(output_path.join("libextern.a"))
        .arg(obj_path)
        .output()
        .unwrap();

    // #[cfg(target_os = "windows")]
    // let lib_output = Command::new("LIB")
    //     .arg(obj_path)
    //     .arg(format!("/OUT:{}", out_dir_path.join("libextern.lib").display())
    //     .output()
    //     .unwrap();

    if !lib_output.status.success() {
        panic!(
            "Could not emit library file:\n{}",
            String::from_utf8_lossy(&lib_output.stderr)
        );
    }

    // Write the rust bindings.
    bindings
        .write_to_file(output_path.join("bindings.rs"))
        .expect("Cound not write bindings to the Rust file");
}
