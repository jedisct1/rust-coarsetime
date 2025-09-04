#[inline]
pub const fn _sec_to_u64(sec: u64) -> u64 {
    sec.saturating_mul(1 << 32)
}

#[inline]
pub const fn _millis_to_u64(millis: u64) -> u64 {
    let secs = millis / 1_000;
    secs.saturating_mul(1 << 32) | ((millis - secs * 1_000) << 22)
}

#[inline]
pub const fn _nsecs_to_u64(nsecs: u64) -> u64 {
    let secs = nsecs / 1_000_000_000;
    _timespec_to_u64(secs, (nsecs - secs * 1_000_000_000) as u32)
}

#[inline]
pub const fn _timespec_to_u64(tp_sec: u64, tp_nsec: u32) -> u64 {
    tp_sec.saturating_mul(1 << 32) | ((tp_nsec as u64 * 9_223_372_037) >> 31)
}

#[inline]
pub const fn _timeval_to_u64(tv_sec: u64, tv_usec: u32) -> u64 {
    tv_sec.saturating_mul(1 << 32) | ((tv_usec as u64 * 9_223_372_036_855) >> 31)
}
