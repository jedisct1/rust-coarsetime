use super::{Duration, Instant};
use std::time;

/// System time
#[derive(Debug)]
pub struct Clock;

lazy_static! {
    static ref CLOCK_OFFSET: u64 = clock_offset();
}

impl Clock {
    /// Returns the elapsed time since the UNIX epoch
    #[inline]
    pub fn now_since_epoch() -> Duration {
        let offset = *CLOCK_OFFSET;
        let unix_ts_now = Instant::now().as_u64().wrapping_sub(offset);
        Duration::from_u64(unix_ts_now)
    }

    /// Returns the elapsed time since the UNIX epoch, based on the latest explicit time update
    #[inline]
    pub fn recent_since_epoch() -> Duration {
        let offset = *CLOCK_OFFSET;
        let unix_ts_now = Instant::recent().as_u64().wrapping_sub(offset);
        Duration::from_u64(unix_ts_now)
    }

    /// Updates the system time - This is completely equivalent to calling Instant::update()
    #[inline]
    pub fn update() {
        Instant::update()
    }
}

fn clock_offset() -> u64 {
    let unix_ts_now_sys = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("The system clock is not properly set");
    let unix_ts_now = Duration::from(unix_ts_now_sys);
    let unix_ts_now = unix_ts_now.as_u64();
    let instant_now = Instant::now().as_u64();
    instant_now.wrapping_sub(unix_ts_now)
}
