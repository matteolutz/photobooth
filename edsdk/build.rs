fn main() {


    #[cfg(target_os = "macos")]
    {
        println!(
            "cargo:rustc-link-search=framework={}",
            "/Library/Frameworks"
        );
        println!("cargo:rustc-link-lib=framework=EDSDK");
        println!("cargo:rustc-link-arg=-Wl,-rpath,/Library/Frameworks")
    }

    #[cfg(target_os = "windows")]
    {
        println!(
            "cargo:rustc-link-search=native={}",
            "C:\\EDSDKv132010W\\Windows\\EDSDK_64\\Library"
        );
        println!("cargo:rustc-link-lib=EDSDK");
        // println!("cargo:rustc-link-lib=framework=EDSDK");
    }
}
