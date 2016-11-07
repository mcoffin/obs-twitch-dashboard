extern crate pkg_config;

const OBS_LIBRARY_NAME: &'static str = "obs";

fn main() {
    match pkg_config::probe_library(OBS_LIBRARY_NAME) {
        Ok(..) => (),
        Err(..) => println!("cargo:rustc-link-lib={}", OBS_LIBRARY_NAME),
    };
}
