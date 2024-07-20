mod logger;
mod platform;
mod systime;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::sys::{gpio_mode_t_GPIO_MODE_OUTPUT, gpio_num_t_GPIO_NUM_38, gpio_reset_pin, gpio_set_direction, gpio_set_level};

fn main() {
    // do platform specific initialisation
    platform::init_platform();
    logger::init().expect("Failed to initialize the logger.");

    log::info!("info channel");
    log::warn!("watch out kids it's working! {}", "foo!");
    log::error!("gremlins at work");
    log::debug!("get that debug printed!");
    log::trace!("get that trace printed!");

    unsafe {
        gpio_reset_pin(gpio_num_t_GPIO_NUM_38);
        gpio_set_direction(gpio_num_t_GPIO_NUM_38, gpio_mode_t_GPIO_MODE_OUTPUT);
        gpio_set_level(gpio_num_t_GPIO_NUM_38 as i32, 0);
        FreeRtos::delay_ms(1000);
        gpio_set_level(gpio_num_t_GPIO_NUM_38 as i32, 1);
    }
}
