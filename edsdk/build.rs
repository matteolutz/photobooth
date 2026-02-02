fn main() {
    println!(
        "cargo:rustc-link-search=framework={}",
        "/Library/Frameworks"
    );

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=EDSDK");
        println!("cargo:rustc-link-arg=-Wl,-rpath,/Library/Frameworks")
    }
}
