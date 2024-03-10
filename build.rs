fn main() {
    // don't link with stdlib
    println!("cargo:rustc-link-arg-bin=nostd0001=-nostartfiles");
}
