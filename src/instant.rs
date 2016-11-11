use duration::*;
use helpers::*;
use libc;
#[allow(unused_imports)]
use std::mem::uninitialized;
use std::ops::*;
#[allow(unused_imports)]
use std::ptr::*;

#[cfg(feature = "nightly")]
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(not(feature = "nightly"))]
use std::sync::Mutex;

#[derive(Copy, Clone, Debug, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct Instant(u64);

#[cfg(feature = "nightly")]
type Recent = AtomicU64;

#[cfg(feature = "nightly")]
static mut RECENT: Recent = AtomicU64::new(0);

#[cfg(not(feature = "nightly"))]
type Recent = Mutex<u64>;

#[cfg(not(feature = "nightly"))]
lazy_static! {
  static ref RECENT: Recent = Mutex::new(0);
}

#[cfg(windows)]
extern "system" {
    pub fn GetTickCount() -> libc::c_ulong;
}

impl Instant {
    pub fn now() -> Instant {
        let now = Self::_now();
        Self::_update(now);
        Instant(now)
    }

    pub fn recent() -> Instant {
        match Self::_recent() {
            0 => Instant::now(),
            recent => Instant(recent),
        }
    }

    pub fn update() {
        let now = Self::_now();
        Self::_update(now);
    }

    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        *self - earlier
    }

    #[inline]
    pub fn elapsed_since_recent(&self) -> Duration {
        Self::recent() - *self
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn _now() -> u64 {
        let mut tp: libc::timespec = unsafe { uninitialized() };
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut tp) };
        _timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u64)
    }

    #[cfg(all(unix, not(any(target_os = "linux", target_os = "android"))))]
    fn _now() -> u64 {
        let mut tv: libc::timeval = unsafe { uninitialized() };
        unsafe { libc::gettimeofday(&mut tv, null_mut()) };
        _timeval_to_u64(tv.tv_sec as u64, tv.tv_usec as u64)
    }

    #[cfg(windows)]
    fn _now() -> u64 {
        let tc = unsafe { GetTickCount() } as u64;
        _millis_to_u64(tc)
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn _update(now: u64) {
        unsafe { RECENT.store(now, Ordering::Relaxed) };
    }

    #[cfg(not(feature = "nightly"))]
    #[inline]
    fn _update(now: u64) {
        *RECENT.lock().unwrap() = now;
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn _recent() -> u64 {
        unsafe { RECENT.load(Ordering::Relaxed) }
    }

    #[cfg(not(feature = "nightly"))]
    #[inline]
    fn _recent() -> u64 {
        *RECENT.lock().unwrap()
    }
}

impl Default for Instant {
    fn default() -> Instant {
        Self::now()
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, other: Instant) -> Duration {
        Duration::from_u64(self.0 - other.0)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn sub(self, rhs: Duration) -> Instant {
        Instant(self.0 - rhs.as_u64())
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn add(self, rhs: Duration) -> Instant {
        Instant(self.0 + rhs.as_u64())
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}
