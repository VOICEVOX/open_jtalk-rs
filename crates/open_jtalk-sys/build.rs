use std::{
    env, fs,
    io::BufRead,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str,
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
        let cmake_osx_sysroot = str::from_utf8(&cmake_osx_sysroot.stdout)
            .unwrap()
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

    if target.contains("emscripten") {
        include_dirs.extend(search_emscripten_include_directories());
    }

    let dst_dir = cmake_conf.build();
    let lib_dir = dst_dir.join("lib");
    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=openjtalk");
    generate_bindings(dst_dir.join("include"), include_dirs);
}

#[cfg(not(feature = "generate-bindings"))]
#[allow(unused_variables)]
fn generate_bindings(
    allow_dir: impl AsRef<Path>,
    include_dirs: impl IntoIterator<Item = impl AsRef<Path>>,
) {
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(
    allow_dir: impl AsRef<Path>,
    include_dirs: impl IntoIterator<Item = impl AsRef<Path>>,
) {
    let include_dir = allow_dir.as_ref();
    let clang_args = include_dirs
        .into_iter()
        .map(|dir| format!("-I{}", dir.as_ref().to_str().unwrap()))
        .chain([format!("-I{}", include_dir.to_str().unwrap())])
        .chain(["-fvisibility=default".to_string()])
        .collect::<Vec<_>>();
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rerun-if-changed=src/generated/bindings.rs");
    let mut bind_builder = bindgen::Builder::default()
        .header("wrapper.hpp")
        .allowlist_recursively(true)
        .clang_args(clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .size_t_is_usize(true)
        .formatter(bindgen::Formatter::Prettyplease)
        .rustified_enum(".*");
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

fn search_emscripten_include_directories() -> impl IntoIterator<Item = PathBuf> {
    let empty_cpp_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("empty.cpp");
    fs::write(&empty_cpp_path, b"").unwrap();

    let mut command;
    if cfg!(target_os = "windows") {
        command = Command::new("cmd");
        command.arg("/C");
    } else {
        command = Command::new("sh");
        command.arg("-c");
    };

    let empp_output = command
        .arg(format!("em++ --verbose {}", empty_cpp_path.display()))
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    empp_output
        .stderr
        .lines()
        .map(Result::unwrap)
        .skip_while(|line| line.trim() != "#include <...> search starts here:")
        .skip(1)
        .take_while(|line| line.trim() != "End of search list.")
        .map(|line| PathBuf::from(line.trim()))
        .collect::<Vec<_>>()
}
