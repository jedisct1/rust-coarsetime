#[inline]
pub fn _sec_to_u64(sec: u64) -> u64 {
    sec << 32
}

#[inline]
#[allow(clippy::arithmetic_side_effects)] // we're careful not to overflow
pub fn _millis_to_u64(millis: u64) -> u64 {
    let secs = millis / 1_000;
    (secs << 32) | ((millis - secs * 1_000) << 22)
}

#[inline]
#[allow(clippy::arithmetic_side_effects)] // we're careful not to overflow
pub fn _nsecs_to_u64(nsecs: u64) -> u64 {
    let secs = nsecs / 1_000_000_000;
    _timespec_to_u64(secs, (nsecs - secs * 1_000_000_000) as u32)
}

macro_rules! ts_tv_normalise { { $sec:ident, $subsec:ident, $div:expr } => {
    $sec = $sec.saturating_add(($subsec / $div) as _);
    $subsec %= $div;
} }

#[inline]
#[allow(clippy::arithmetic_side_effects)] // we're careful not to overflow
pub fn _timespec_to_u64(mut tp_sec: u64, mut tp_nsec: u32) -> u64 {
    ts_tv_normalise!(tp_sec, tp_nsec, 1_000_000_000);
    (tp_sec << 32) | ((tp_nsec as u64 * 9_223_372_037) >> 31)
}

#[inline]
#[allow(clippy::arithmetic_side_effects)] // we're careful not to overflow
pub fn _timeval_to_u64(mut tv_sec: u64, mut tv_usec: u32) -> u64 {
    ts_tv_normalise!(tv_sec, tv_usec, 1_000_000);
    (tv_sec << 32) | ((tv_usec as u64 * 9_223_372_036_855) >> 31)
}

#[test]
fn no_overflow() {
    let _: u64 = _millis_to_u64(0);
    let _: u64 = _millis_to_u64(u64::MAX);
    let _: u64 = _nsecs_to_u64(0);
    let _: u64 = _nsecs_to_u64(u64::MAX);
    let _: u64 = _timespec_to_u64(0, 0);
    let _: u64 = _timespec_to_u64(0, u32::MAX);
    let _: u64 = _timespec_to_u64(u64::MAX, 0);
    let _: u64 = _timespec_to_u64(u64::MAX, u32::MAX);
}
