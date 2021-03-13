extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/c/snes_spc/spc.h");
    cc::Build::new()
        .include("src/c/")
        .flag("-fPIC")
        .flag("-fno-exceptions")
        // .flag("-Wno-implicit-fallthrough")
        .flag("-fno-rtti")
        .flag("-Wall")
        // .flag("-Wextra")
        .flag("-std=c++11")
        .flag("-DNDEBUG")
        .flag("-DSPC_ISOLATED_ECHO_BUFFER")
        .flag("-DBLARGG_BUILD_DLL")
        .opt_level(3)
        .cpp(true)
        .file("src/c/snes_spc/SNES_SPC.cpp")
        .file("src/c/snes_spc/SNES_SPC_misc.cpp")
        .file("src/c/snes_spc/SNES_SPC_state.cpp")
        .file("src/c/snes_spc/SPC_DSP.cpp")
        .file("src/c/snes_spc/SPC_Filter.cpp")
        .file("src/c/snes_spc/dsp.cpp")
        .file("src/c/snes_spc/spc.cpp")
        .compile("libspc.a");

    let bindings = bindgen::Builder::default()
        .header("src/c/snes_spc/spc.h")
        .derive_copy(true)
        .derive_debug(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
