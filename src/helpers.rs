#[inline]
pub fn _sec_to_u64(sec: u64) -> u64 {
    sec << 32
}

#[inline]
pub fn _millis_to_u64(millis: u64) -> u64 {
    let secs = millis / 1_000;
    (secs << 32) | ((millis - secs * 1_000) << 22)
}

#[inline]
pub fn _nsecs_to_u64(nsecs: u64) -> u64 {
    let secs = nsecs / 1_000_000_000;
    _timespec_to_u64(secs, (nsecs - secs * 1_000_000_000) as u32)
}

macro_rules! ts_tv_normalise { { $sec:ident, $subsec:ident, $div:expr } => {
    $sec = $sec.saturating_add(($subsec / $div) as _);
    $subsec %= $div;
} }

#[inline]
pub fn _timespec_to_u64(mut tp_sec: u64, mut tp_nsec: u32) -> u64 {
    ts_tv_normalise!(tp_sec, tp_nsec, 1_000_000_000);
    (tp_sec << 32) | ((tp_nsec as u64 * 9_223_372_037) >> 31)
}

#[inline]
pub fn _timeval_to_u64(mut tv_sec: u64, mut tv_usec: u32) -> u64 {
    ts_tv_normalise!(tv_sec, tv_usec, 1_000_000);
    (tv_sec << 32) | ((tv_usec as u64 * 9_223_372_036_855) >> 31)
}
