use std::str::FromStr;
use lazy_static::lazy_static;
use log::{LevelFilter};
use crate::esp_systime::AppEspSystemTime;

pub(crate) struct EspLogger;

/*
 * Our own implementation of the log function that we give to println!
 * This one uses slightly different output with format:
 * T/D/I/W/E (time) - Message
 *
 * It uses value from envioronment value ESP_LOGLEVEL to configure the default minimum logging level, defaulting to INFO if unset.
 * This does not use the ESP logging framework, which is turned off in sdkconfig.defaults (or rather the level is set to ERROR).
 */

const ESP_LOGLEVEL: Option<&'static str> = option_env!("ESP_LOGLEVEL");

impl log::Log for EspLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    #[allow(unused)]
    fn log(&self, record: &log::Record) {
        const RESET: &str = "\u{001B}[0m";
        const RED: &str = "\u{001B}[31m";
        const GREEN: &str = "\u{001B}[32m";
        const YELLOW: &str = "\u{001B}[33m";
        const BLUE: &str = "\u{001B}[34m";
        const CYAN: &str = "\u{001B}[35m";

        let reset = RESET;

        let (color, lvl) = match record.level() {
            log::Level::Error => (RED, "E"),
            log::Level::Warn =>  (GREEN, "W"),
            log::Level::Info =>  (YELLOW,"I"),
            log::Level::Debug => (BLUE, "D"),
            log::Level::Trace => (CYAN, "T"),
        };
        let d = AppEspSystemTime::now().as_millis();
        esp_println::println!("{}{} ({}): {}{}", color, lvl, d, record.args(), reset);
    }

    fn flush(&self) {}
}

lazy_static! {
    static ref LOGGER: EspLogger = EspLogger;
}

pub fn init() -> Result<(), log::SetLoggerError> {
    let log_level_str = ESP_LOGLEVEL.unwrap_or("INFO");
    let log_level = LevelFilter::from_str(&log_level_str).unwrap_or(LevelFilter::Info);
    unsafe {
        log::set_logger_racy(&EspLogger).map(|()| log::set_max_level_racy(log_level))
    }
}
