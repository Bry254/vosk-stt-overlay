fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=dylib=vosk");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}
