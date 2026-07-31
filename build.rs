fn main() {
    cc::Build::new()
        .flag("-std=c23")
        .define("_DEFAULT_SOURCE", None)
        .file("c_src/its_ffi.c")
        .file("ITS/common.c")
        .include("ITS")
        .compile("its_ffi");
    println!("cargo:rustc-link-lib=gmp");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rerun-if-changed=c_src/its_ffi.c");
    println!("cargo:rerun-if-changed=ITS/common.c");
    println!("cargo:rerun-if-changed=ITS/common.h");
}
