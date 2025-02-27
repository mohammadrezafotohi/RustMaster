fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=dylib=SDL2_image");

    println!(r"cargo:rustc-link-search=native=C:\Users\pc\Desktop\Desktop\Projects\RustMaster\6-Check_Night_Light\libs");
    
    println!("cargo:rustc-link-lib=static=SDL2");
}