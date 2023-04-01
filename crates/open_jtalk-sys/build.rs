use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};
fn main() {
    let mut cmake_conf = cmake::Config::new("open_jtalk");
    let target = env::var("TARGET").unwrap();
    let mut include_dirs: Vec<PathBuf> = Vec::new();
    let cmake_conf = if target.starts_with("i686") {
        let cmake_conf = cmake_conf.define("OPEN_JTALK_X86", "true");

        if target.contains("windows") {
            cmake_conf.define("CMAKE_GENERATOR_PLATFORM", "Win32")
        } else {
            cmake_conf
        }
    } else {
        &mut cmake_conf
    };

    let debug = env::var("DEBUG").is_ok();
    // open_jtalkのビルドprofileがdebugだとWindowsでリンクエラーになるため、Releaseにする
    if debug {
        cmake_conf.profile("Release");
    }

    // androidのminSdkを指定する
    if target.contains("android") {
        // nfkとcmake間でパスに問題があるため１にする
        cmake_conf.define("CMAKE_SYSTEM_VERSION", "1");
    }

    // iOS SDKで必要な引数を指定する
    if target.contains("ios") {
        // iOSとiPhone simulatorは別扱いになる
        let sdk = if target.ends_with("sim") || target.starts_with("x86_64") {
            "iphonesimulator"
        } else {
            "iphoneos"
        };
        let cmake_osx_sysroot = Command::new("xcrun")
            .args(["--sdk", sdk, "--show-sdk-path"])
            .output()
            .expect("Failed to run xcrun command");
        let cmake_osx_sysroot = String::from_utf8_lossy(&cmake_osx_sysroot.stdout)
            .trim()
            .to_string();
        cmake_conf.define("CMAKE_OSX_SYSROOT", &cmake_osx_sysroot);
        // x86_64アーキテクチャのiPhoneシミュレータではC++のヘッダーのパスが通っていないので、通す
        if target.starts_with("x86_64") {
            let include_dir = PathBuf::from(&cmake_osx_sysroot)
                .join("usr")
                .join("include")
                .join("c++")
                .join("v1");
            include_dirs.push(include_dir);
        }
    }

    let dst_dir = cmake_conf.build();
    let lib_dir = dst_dir.join("lib");
    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=openjtalk");
    generate_bindings(dst_dir.join("include"), include_dirs.iter());
}

#[cfg(not(feature = "generate-bindings"))]
fn generate_bindings(
    #[allow(unused_variables)] allow_dir: impl AsRef<Path>,
    include_dirs: impl Iterator<Item = impl AsRef<Path>>,
) {
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(
    allow_dir: impl AsRef<Path>,
    include_dirs: impl Iterator<Item = impl AsRef<Path>>,
) {
    let include_dir = allow_dir.as_ref();
    let clang_arg = &[format!("-I{}", include_dir.display())];
    let mut clang_args: Vec<String> = include_dirs
        .map(|dir| format!("-I{}", dir.as_ref().display()))
        .collect();
    clang_args.extend(clang_arg.iter().cloned());
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
    let paths = std::fs::read_dir(include_dir).unwrap();
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
