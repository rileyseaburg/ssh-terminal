fn main() {
    // Link system zlib - required for libssh2 compression support on iOS
    // The 'z' library is the system zlib on Apple platforms
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("apple") || target.contains("ios") {
        println!("cargo:rustc-link-lib=z");
    }
    
    // Standard Tauri build
    tauri_build::build()
}
