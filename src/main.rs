mod esp_log;
mod esp_systime;

use std::thread;
use std::time::Duration;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_log::init().expect("Failed to initialize the logger.");

    log::info!("will this work?");
    thread::sleep(Duration::from_millis(22));
    log::warn!("watch out kids it's working! {}", "foo!");
    thread::sleep(Duration::from_millis(69));
    log::error!("gremlins at work");
    log::debug!("get that debug printed!");
    log::trace!("get that trace printed!");
}
