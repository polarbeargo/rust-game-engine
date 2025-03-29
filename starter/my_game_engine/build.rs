fn main() {
    // Specify the path to the C library
    let c_lib_path = "opengl_wrapper_lib";

    // Compile the C library
    cc::Build::new()
        .file(format!("{}/opengl_wrapper.c", c_lib_path))
        .compile("opengl_wrapper");
   
}
