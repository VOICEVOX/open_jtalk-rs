use std::path::Path;
fn main() {
    let dst_dir = cmake::build("openjtalk");
    let lib_dir = dst_dir.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=openjtalk");
    generate_bindings(dst_dir.join("include"));
}

#[cfg(not(feature = "generate-bindings"))]
fn generate_bindings(#[allow(unused_variables)] include_dir: impl AsRef<Path>) {}

//#[cfg(feature = "generate-bindings")]
fn generate_bindings(include_dir: impl AsRef<Path>) {
    use std::env;
    use std::path::PathBuf;
    let include_dir = include_dir.as_ref();
    let clang_args = &[format!("-I{}", include_dir.display())];
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rerun-if-changed=src/generated/bindings.rs");
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .allowlist_recursively(true)
        .allowlist_file(include_dir.join("mecab.h").display().to_string())
        .allowlist_file(include_dir.join("njd.h").display().to_string())
        .allowlist_file(include_dir.join("jpcommon.h").display().to_string())
        .allowlist_file(include_dir.join("njd2jpcommon.h").display().to_string())
        .allowlist_file(
            include_dir
                .join("njd_set_accent_phrase.h")
                .display()
                .to_string(),
        )
        .allowlist_file(
            include_dir
                .join("njd_set_accent_type.h")
                .display()
                .to_string(),
        )
        .allowlist_file(include_dir.join("njd_set_digit.h").display().to_string())
        .allowlist_file(
            include_dir
                .join("njd_set_long_vowel.h")
                .display()
                .to_string(),
        )
        .allowlist_file(
            include_dir
                .join("njd_set_pronunciation.h")
                .display()
                .to_string(),
        )
        .allowlist_file(
            include_dir
                .join("njd_set_unvoiced_vowel.h")
                .display()
                .to_string(),
        )
        .allowlist_file(include_dir.join("text2mecab.h").display().to_string())
        .allowlist_file(include_dir.join("mecab2njd.h").display().to_string())
        .clang_args(clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .rustified_enum("*")
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
