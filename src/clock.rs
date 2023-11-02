#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
use std::time;

use std::sync::atomic::{AtomicU64, Ordering};

use super::Duration;

static RECENT: AtomicU64 = AtomicU64::new(0);

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
mod js_imports {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Date;

        #[wasm_bindgen(static_method_of = Date)]
        pub fn now() -> f64;
    }
}

/// System time
#[derive(Debug)]
pub struct Clock;

/// Alias for `Duration`.
pub type UnixTimeStamp = Duration;

impl Clock {
    /// Returns the elapsed time since the UNIX epoch
    #[inline]
    pub fn now_since_epoch() -> UnixTimeStamp {
        Duration::from_u64(unix_ts())
    }

    /// Returns the elapsed time since the UNIX epoch, based on the latest
    /// explicit time update
    #[inline]
    pub fn recent_since_epoch() -> UnixTimeStamp {
        Duration::from_u64(RECENT.load(Ordering::Relaxed))
    }

    /// Updates the cached system time.
    ///
    /// This function should be called frequently, for example in an event loop
    /// or using an `Updater` task.
    #[inline]
    pub fn update() {
        let now = unix_ts();
        RECENT.store(now, Ordering::Relaxed)
    }

    /// Sets the cached system time to the specified timestamp.
    /// This function is intended for testing purposes only.
    /// It should not be used in production code.
    pub fn set_recent_since_epoch(recent: UnixTimeStamp) {
        RECENT.store(recent.as_u64(), Ordering::Relaxed)
    }
}

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
#[inline]
fn unix_ts() -> u64 {
    let unix_ts_now_sys = (js_imports::Date::now() / 1000.0).round() as u64;
    let unix_ts_now = Duration::from_secs(unix_ts_now_sys);
    unix_ts_now.as_u64()
}

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
#[inline]
fn unix_ts() -> u64 {
    let unix_ts_now_sys = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("The system clock is not properly set");
    let unix_ts_now = Duration::from(unix_ts_now_sys);
    unix_ts_now.as_u64()
}
