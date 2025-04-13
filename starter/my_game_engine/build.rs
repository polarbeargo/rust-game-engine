fn main() {
    let c_lib_path = "../opengl_wrapper_lib";

    cc::Build::new()
        .file(format!("{}/opengl_wrapper_lib.c", c_lib_path))
        .compile("opengl_wrapper");

    // Link against necessary shared libraries
    println!("cargo:rustc-link-lib=static=opengl_wrapper");
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-search=native=c_output");
}
