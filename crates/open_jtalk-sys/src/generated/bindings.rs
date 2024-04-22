#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/linux/x86_64/bindings.rs"
));

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/linux/aarch64/bindings.rs"
));

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/macos/x86_64/bindings.rs"
));

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/macos/aarch64/bindings.rs"
));

#[cfg(all(target_os = "ios", target_arch = "aarch64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/ios/aarch64/bindings.rs"
));

#[cfg(all(target_os = "ios", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/ios/x86_64/bindings.rs"
));

#[cfg(all(target_os = "windows", target_arch = "x86"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/windows/x86/bindings.rs"
));

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/windows/x86_64/bindings.rs"
));

#[cfg(all(target_os = "android", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/x86_64/bindings.rs"
));

#[cfg(all(target_os = "android", target_arch = "aarch64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/aarch64/bindings.rs"
));

#[cfg(all(target_os = "emscripten", target_arch = "wasm32"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/emscripten/wasm32/bindings.rs"
));
