#[allow(unused_imports)]
use std::mem::MaybeUninit;
use std::ops::*;
#[allow(unused_imports)]
use std::ptr::*;
use std::sync::atomic::{AtomicU64, Ordering};

use super::duration::*;
#[allow(unused_imports)]
use super::helpers::*;

/// A measurement of a *monotonically* increasing clock. Opaque and useful only
/// with `Duration`.
/// Resulting durations are actual durations; they do not get affected by
/// clock adjustments, leap seconds, or similar.
/// In order to get a measurement of the *wall clock*, use `Date` instead.
#[derive(Copy, Clone, Debug, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct Instant(u64);

static RECENT: AtomicU64 = AtomicU64::new(0);

#[cfg(windows)]
extern "system" {
    pub fn GetTickCount64() -> libc::c_ulonglong;
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
#[allow(non_camel_case_types)]
type clockid_t = libc::c_int;

#[cfg(target_os = "macos")]
const CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;

#[cfg(target_os = "macos")]
extern "system" {
    fn clock_gettime_nsec_np(clk_id: clockid_t) -> u64;
}

#[cfg(target_os = "freebsd")]
const CLOCK_MONOTONIC_FAST: clockid_t = 12;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
mod js_imports {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[allow(non_camel_case_types)]
        pub type performance;

        #[wasm_bindgen(static_method_of = performance)]
        pub fn now() -> f64;
    }
}

impl Instant {
    /// Returns an instant corresponding to "now"
    ///
    /// This function also updates the stored instant.
    pub fn now() -> Instant {
        let now = Self::_now();
        Self::_update(now);
        Instant(now)
    }

    /// Returns an instant corresponding to "now" without updating the cached value.
    /// After this, `recent()` will still return the old instant.
    ///
    /// `now()` is generally preferred over this function.
    pub fn now_without_cache_update() -> Instant {
        let now = Self::_now();
        Instant(now)
    }

    /// Returns an instant corresponding to the latest update
    pub fn recent() -> Instant {
        match Self::_recent() {
            0 => Instant::now(),
            recent => Instant(recent),
        }
    }

    /// Update the stored instant
    ///
    /// This function should be called frequently, for example in an event loop
    /// or using an `Updater` task.
    pub fn update() {
        let now = Self::_now();
        Self::_update(now);
    }

    /// Returns the amount of time elapsed from another instant to this one
    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        *self - earlier
    }

    /// Returns the amount of time elapsed between the this instant was created
    /// and the latest update
    #[inline]
    pub fn elapsed_since_recent(&self) -> Duration {
        Self::recent() - *self
    }

    /// Returns the amount of time elapsed since this instant was created
    ///
    /// This function also updates the stored instant.
    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    /// Return a representation of this instant as a number of "ticks".
    ///
    /// Note that length of a 'tick' is not guaranteed to represent
    /// the same amount of time across different platforms, or from
    /// one version of `coarsetime` to another.
    ///
    /// Note also that the instant represented by "0" ticks is
    /// unspecified.  It is not guaranteed to be the same time across
    /// different platforms, or from one version of `coarsetime` to
    /// another.
    ///
    /// This API is mainly intended for applications that need to
    /// store the value of an `Instant` in an
    /// [`AtomicU64`](std::sync::atomic::AtomicU64).
    #[inline]
    pub fn as_ticks(&self) -> u64 {
        self.as_u64()
    }

    #[doc(hidden)]
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn _now() -> u64 {
        let mut tp = MaybeUninit::<libc::timespec>::uninit();
        let tp = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC_COARSE, tp.as_mut_ptr());
            tp.assume_init()
        };
        _timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u32)
    }

    #[cfg(target_os = "macos")]
    fn _now() -> u64 {
        let nsec = unsafe { clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW_APPROX) };
        _nsecs_to_u64(nsec)
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    fn _now() -> u64 {
        let mut tp = MaybeUninit::<libc::timespec>::uninit();
        let tp = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC_FAST, tp.as_mut_ptr());
            tp.assume_init()
        };
        _timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u32)
    }

    #[cfg(all(
        unix,
        not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "android",
            target_os = "freebsd",
            target_os = "dragonfly"
        ))
    ))]
    fn _now() -> u64 {
        let mut tv = MaybeUninit::<libc::timeval>::uninit();
        let tv = unsafe {
            libc::gettimeofday(tv.as_mut_ptr(), null_mut());
            tv.assume_init()
        };
        _timeval_to_u64(tv.tv_sec as u64, tv.tv_usec as u32)
    }

    #[cfg(windows)]
    fn _now() -> u64 {
        let tc = unsafe { GetTickCount64() } as u64;
        _millis_to_u64(tc)
    }

    #[cfg(any(
        target_os = "wasix",
        all(target_os = "wasi", not(feature = "wasi-abi2"))
    ))]
    fn _now() -> u64 {
        use wasix::{clock_time_get, CLOCKID_MONOTONIC, CLOCKID_REALTIME};
        let nsec = unsafe { clock_time_get(CLOCKID_MONOTONIC, 1_000_000) }
            .or_else(|_| unsafe { clock_time_get(CLOCKID_REALTIME, 1_000_000) })
            .expect("Clock not available");
        _nsecs_to_u64(nsec)
    }

    #[cfg(all(target_os = "wasi", feature = "wasi-abi2"))]
    fn _now() -> u64 {
        let nsec = wasi_abi2::clocks::monotonic_clock::now();
        _nsecs_to_u64(nsec)
    }

    #[cfg(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    ))]
    fn _now() -> u64 {
        _millis_to_u64(js_imports::performance::now() as u64)
    }

    #[cfg(all(target_arch = "x86_64", target_env = "sgx", target_vendor = "fortanix"))]
    fn _now() -> u64 {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        timestamp.as_secs() * 1_000_000_000 + (timestamp.subsec_nanos() as u64)
    }

    #[inline]
    fn _update(now: u64) {
        RECENT.store(now, Ordering::Relaxed)
    }

    #[inline]
    fn _recent() -> u64 {
        let recent = RECENT.load(Ordering::Relaxed);
        if recent != 0 {
            recent
        } else {
            let now = Self::_now();
            Self::_update(now);
            Self::_recent()
        }
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
        Duration::from_u64(self.0.saturating_sub(other.0))
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
