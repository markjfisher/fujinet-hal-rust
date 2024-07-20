// This script is compiled and run before the build phase happens.
fn main() {
    embuild::espidf::sysenv::output();

    // DEBUG: output the current target_arch
    // let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "unknown".to_string());
    // println!("cargo:warning=Compiling for target architecture: {}", target_arch);
}
