#[cfg(target_arch = "xtensa")]
mod esp;
#[cfg(target_arch = "xtensa")]
pub use esp::init_platform;

// #[cfg(target_arch = "arm")]
// mod rp2040;
// #[cfg(target_arch = "arm")]
// pub use rp2040::init_platform;
