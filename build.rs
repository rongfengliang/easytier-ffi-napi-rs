extern crate napi_build;

fn main() {
  println!("cargo:rustc-link-search=native=./");
  println!("cargo:rustc-link-lib=dylib=easytier_ffi");
  let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
  napi_build::setup();
}
