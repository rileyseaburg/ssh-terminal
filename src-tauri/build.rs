fn main() {
    // Link system zlib - required for libssh2 compression support on iOS
    // Use CARGO_CFG_TARGET_OS to detect cross-compilation targets
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "ios" || target_os == "macos" {
        println!("cargo:rustc-link-lib=z");
    }
    
    // Standard Tauri build
    tauri_build::build()
}
