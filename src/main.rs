mod logger;
mod platform;
mod systime;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::sys::{gpio_mode_t_GPIO_MODE_OUTPUT, gpio_num_t_GPIO_NUM_38, gpio_reset_pin, gpio_set_direction, gpio_set_level};

fn main() {
    // do platform specific initialisation
    platform::init_platform();
    logger::init().expect("Failed to initialize the logger.");

    // blinky test!
    let mut led_level: u32 = 0;
    let mut delay: u32 = 20;
    let mut factor: f64 = 1.05;
    unsafe {
        gpio_reset_pin(gpio_num_t_GPIO_NUM_38);
        gpio_set_direction(gpio_num_t_GPIO_NUM_38, gpio_mode_t_GPIO_MODE_OUTPUT);
    }

    log::info!("info channel");
    log::warn!("watch out kids it's working! {}", "foo!");
    log::error!("gremlins at work");
    log::debug!("get that debug printed!");
    log::trace!("get that trace printed!");

    loop {
        unsafe { gpio_set_level(gpio_num_t_GPIO_NUM_38 as i32, led_level); }
        led_level = 1 - led_level;
        FreeRtos::delay_ms(delay);
        delay = (delay as f64 * factor).round() as u32;
        if delay > 400 {
            factor = 0.95238;
        }
        if delay <= 20 {
            factor = 1.05;
        }
    }
}
