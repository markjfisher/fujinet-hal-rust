use std::time::Duration;
use esp_idf_svc::sys::{gettimeofday, timeval};

pub struct AppEspSystemTime;

impl AppEspSystemTime {
    // static version of systime version
    pub fn now() -> Duration {
        let mut tv_now: timeval = Default::default();
        unsafe {
            gettimeofday(&mut tv_now as *mut _, core::ptr::null_mut());
        }
        Duration::from_micros(tv_now.tv_sec as u64 * 1000000_u64 + tv_now.tv_usec as u64)
    }
}