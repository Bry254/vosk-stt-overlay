fn main() {
    println!("cargo:rustc-link-search=native=libs");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}
