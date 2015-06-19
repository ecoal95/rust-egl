extern crate gl_generator;
extern crate khronos_api;

use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("egl_bindings.rs")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Egl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::EGL_XML,
                                    vec!["EGL_KHR_image_base".to_owned()],
                                    "1.5", "core", &mut file).unwrap();
}
