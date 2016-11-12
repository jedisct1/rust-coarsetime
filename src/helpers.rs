#[inline]
pub fn _sec_to_u64(sec: u64) -> u64 {
    sec << 32
}

#[inline]
pub fn _millis_to_u64(millis: u64) -> u64 {
    (millis * 1_099_511_628) >> 8
}

#[inline]
pub fn _timespec_to_u64(tp_sec: u64, tp_nsec: u64) -> u64 {
    (tp_sec << 32) | ((tp_nsec * 9_223_372_037) >> 31)
}

#[inline]
pub fn _timeval_to_u64(tv_sec: u64, tv_usec: u64) -> u64 {
    (tv_sec << 32) | ((tv_usec * 9_223_372_036_855) >> 31)
}
