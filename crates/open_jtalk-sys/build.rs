use std::{env, path::Path};
fn main() {
    let mut cmake_conf = cmake::Config::new("open_jtalk");
    let target = env::var("TARGET").unwrap();
    let cmake_conf = if target.starts_with("i686") {
        cmake_conf.define("OPEN_JTALK_X86", "true")
    } else {
        &mut cmake_conf
    };

    let debug = env::var("DEBUG").is_ok();
    // open_jtalkのビルドprofileがdebugだとWindowsでリンクエラーになるため、Releaseにする
    if debug {
        cmake_conf.profile("Release");
    }

    let dst_dir = cmake_conf.build();
    let lib_dir = dst_dir.join("lib");
    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=openjtalk");
    generate_bindings(dst_dir.join("include"));
}

#[cfg(not(feature = "generate-bindings"))]
fn generate_bindings(#[allow(unused_variables)] include_dir: impl AsRef<Path>) {}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(include_dir: impl AsRef<Path>) {
    use std::path::PathBuf;
    let include_dir = include_dir.as_ref();
    let clang_args = &[format!("-I{}", include_dir.display())];
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rerun-if-changed=src/generated/bindings.rs");
    let mut bind_builder = bindgen::Builder::default()
        .header("wrapper.hpp")
        .allowlist_recursively(true)
        .clang_args(clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .rustified_enum("*");
    let paths = std::fs::read_dir(&include_dir).unwrap();
    for path in paths {
        let path = path.unwrap();
        let file_name = path.file_name().to_str().unwrap().to_string();
        bind_builder =
            bind_builder.allowlist_file(format!(".*{}", file_name.replace(".h", "\\.h")));
    }

    let bindings = bind_builder
        .generate()
        .expect("Unable to generate bindings");
    let generated_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("generated")
        .join(env::var("CARGO_CFG_TARGET_OS").unwrap())
        .join(env::var("CARGO_CFG_TARGET_ARCH").unwrap())
        .join("bindings.rs");
    println!("cargo:rerun-if-changed={:?}", generated_file);
    bindings
        .write_to_file(&generated_file)
        .expect("Couldn't write bindings!");
}
